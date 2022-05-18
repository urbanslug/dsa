#![allow(unused_imports)]
//! # elastic degenerate string aligner (edsa)
//!
//! Exact and approximate String Matching to Elastic Degenerate Strings.

mod align;
mod cli;
mod tests_prelude;
mod types;

use eds::{self, Sequence};
use needletail::parse_fastx_file;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::time::Instant;

use fbox::string;

fn main() {
    let config: types::AppConfig = cli::start();
    let total_time = Instant::now();
    let verbosity = config.verbosity;

    if verbosity > 1 {
        eprintln!("{:#?}", config);
    }

    let get_dt = || -> eds::DT {
        let edt = eds::EDT::from_file(&config.edt);

        edt.extract_inelastic()
    };

    let opts = vec![config.handle_as, config.convert_to];

    match config.alignment_options {
        types::SubCommand::Align => {
            let mut reader = needletail::parse_fastx_file(&config.fasta)
                .unwrap_or_else(|_| panic!("[edsa::main] invalid fasta path/file."));
            let seq_record = reader
                .next()
                .expect("[edsa::main] end of iter")
                .expect("[edsa::main] invalid record");

            let seq = seq_record.seq();

            let dt: eds::DT = get_dt();

            if verbosity > 0 {
                eprintln!("Size: {} \nQuery length: {}", dt.size(), seq.len());
            }

            let now = Instant::now();
            let (score, cigar) = align::align(&seq, &dt, &config).expect("");
            eprintln!(
                "[edsa::main] done align DSA. Time taken {} seconds.",
                now.elapsed().as_millis() as f64 / 1000.0
            );

            let rle_cigar: Vec<u8> = fbox::string::run_length_encode(cigar.as_bytes());
            eprintln!("[edsa::main]");
            eprintln!("score = {}", score);
            println!("{}", std::str::from_utf8(&rle_cigar).unwrap());
        }

        types::SubCommand::Convert => match opts[..] {
            [types::Format::Degenerate, types::Format::MSA] => {
                let edt = eds::EDT::from_file(&config.edt);
                let dt: eds::DT = edt.extract_inelastic();
                dt.to_msa();
            }
            [types::Format::Elastic, types::Format::MSA] => {
                let edt = eds::EDT::from_file(&config.edt);
                edt.to_msa();
            }
            _ => panic!("[edsa::main] convert - unknown option"),
        },

        types::SubCommand::Stats => match config.handle_as {
            types::Format::Degenerate => {
                let dt: eds::DT = get_dt();
                println!(
                    "File: {}, \n\
                     size (N): {}, \n\
                     max_variants (s): {}, \n\
                     width (n): {}",
                    config.edt,
                    dt.size(),
                    dt.z(),
                    dt.p()
                );
            }
            types::Format::Elastic => {}
            _ => panic!("[edsa::main] stats - unknown option"),
        },
    }

    if verbosity > 0 {
        eprintln!(
            "[edsa::main] all done. Total time taken {} seconds.",
            total_time.elapsed().as_millis() as f64 / 1000.0
        )
    }
}
