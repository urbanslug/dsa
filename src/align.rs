use crate::types::{self as edsa_types, AppConfig, Format, Penalties, SubCommand};

use dwflambda::align::wf_align;
use dwflambda::{self, types};

pub fn align(query: &[u8], dt: &eds::DT, config: &AppConfig) -> Result<(usize, String), String> {
    let verbosity = config.verbosity;
    if verbosity > 1 {
        eprintln!("[gap_affine::align]");
    }

    let tlen = dt.p();
    let qlen = query.len();

    // -----
    // Align
    // -----

    let mut match_lambda = |v: &mut i32, h: &mut i32, offsets: &mut types::Offset| -> bool {
        if verbosity > 4 {
            eprint!("v ({}, {})\n", h, v);
        }

        if *v < 0 || *h < 0 || *h as usize >= tlen || *v as usize >= qlen {
            return false;
        }

        let text_chars: &Vec<u8> = &dt[*h as usize];
        let query_char: u8 = query[*v as usize];
        let z = text_chars.len();

        let l: usize = offsets.offset_count();

        if z > l {
            // copy over
            let prev: i32 = offsets.max();
            *offsets = types::Offset::from_vec(&vec![prev; z]);

            if z > 1 {
                offsets.abandoned_all_null();
            }
        }

        if z < l {
            let furthest: i32 = offsets.max();
            *offsets = types::Offset::from_vec(&vec![furthest; z]);

            if z == 1 {
                offsets.abdandoned = None;
            }
        }

        if verbosity > 2 {
            // eprint!("offsets {:?} q {} row {:?}", offsets, col_char as char, r);
        }

        let mut found = false;
        let mut increment_once = false;

        for idx in 0..z {
            if text_chars[idx] == query_char && !offsets.is_abandoned(idx) {
                offsets.data[idx] += 1;

                found = true;

                if increment_once == false {
                    *v += 1;
                    *h += 1;

                    increment_once = true;
                }
            }

            if text_chars[idx] != query_char {
                offsets.set_abandon(idx);
            }
        }

        if verbosity > 4 {
            eprint!("\tfound={} {:?}", found, offsets);
            eprintln!();
        }

        found
    };

    let mut traceback_lambda =
        |(q_start, q_stop): (i32, i32), (t_start, t_stop): (i32, i32)| -> bool {
            if q_start < 0 || q_stop < 0 || t_start < 0 || t_stop < 0 {
                return false;
            }

            let res = (q_start as usize..q_stop as usize)
                .zip(t_start as usize..t_stop as usize)
                .fold(true, |acc, (q_index, t_index)| {
                    dt[t_index]
                        .iter()
                        .copied()
                        .any(|t_char| t_char == query[q_index])
                        && acc
                });

            // eprintln!("{:?} {:?} {}", q, t, res);
            res
        };

    let wflambda_config = dwflambda::types::Config {
        adapt: false,
        verbosity: 1,
        penalties: dwflambda::types::Penalties {
            mismatch: config.penalties.x as i32,
            matches: config.penalties.a as i32,
            gap_open: config.penalties.o as i32,
            gap_extend: config.penalties.e as i32,
        },
    };

    wf_align(
        tlen as u32,
        qlen as u32,
        &wflambda_config,
        &mut match_lambda,
        &mut traceback_lambda,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    static CONFIG: edsa_types::AppConfig = edsa_types::AppConfig {
        alignment_options: edsa_types::SubCommand::Degenerate,
        fasta: String::new(),
        edt: String::new(),
        convert_to: edsa_types::Format::Fasta,
        k: 0,
        bind: true,
        bind_above: 0,
        bind_below: 0,
        verbosity: 0,
        penalties: edsa_types::Penalties {
            a: 0,
            x: 4,
            o: 6,
            e: 2,
        },
    };

    #[test]
    fn test_matches() {
        let fasta = "ATCGAA";
        let ed_string = "ATC{TA,GA}A";
        let edt = eds::EDT::from_str(ed_string);
        let dt: eds::DT = edt.extract_inelastic();

        if let Ok((score, cigar)) = align(fasta.as_bytes(), &dt, &CONFIG) {
            assert_eq!("MMMMMM", cigar);
            assert_eq!(0, score);
        } else {
            assert!(false);
        }
    }

    // also tests replacement
    #[test]
    fn test_artifact_match() {
        let fasta = "ATCGAA";
        let ed_string = "ATC{TA,GC}A";
        let edt = eds::EDT::from_str(ed_string);
        let dt: eds::DT = edt.extract_inelastic();

        eprintln!("{}", fasta);
        eprintln!("{}", dt);

        if let Ok((score, cigar)) = align(fasta.as_bytes(), &dt, &CONFIG) {
            assert_eq!("MMMMXM", cigar);
            assert_eq!(4, score);
        } else {
            assert!(false);
        }
    }
}
