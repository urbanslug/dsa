use crate::types::{self, AppConfig, SubCommand};
use clap::{Arg, Command};
use std::env;

// Env vars
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub fn start() -> AppConfig {
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .subcommand(
            Command::new("align")
                .about("Approximate alignment to an (E)DT")
                .short_flag('A')
                .arg(
                    Arg::new("eds")
                        .required(true)
                        .takes_value(true)
                        .help("Path to text eds"),
                )
                .arg(
                    Arg::new("fasta")
                        .required(true)
                        .takes_value(true)
                        .help("Path to query fasta"),
                )
                .arg(
                    Arg::new("v")
                        .short('v')
                        .multiple_occurrences(true)
                        .help("Sets the level of verbosity [default: 0]"),
                ),
        )
        .subcommand(
            Command::new("stats")
                .about("Print stats on an (E)DT")
                .short_flag('S')
                .arg(
                    Arg::new("eds")
                        .required(true)
                        .takes_value(true)
                        .help("Path to text eds"),
                )
                .arg(
                    Arg::new("type")
                        .short('t')
                        .long("eds-type")
                        .multiple_values(false)
                        .default_value("i")
                        .help("e (for elastic) or i (for inelastic)"),
                ),
        )
        .subcommand(
            Command::new("convert")
                .about("Convert an EDT into MSA (Fasta)")
                .short_flag('C')
                .arg(
                    Arg::new("eds")
                        .required(true)
                        .takes_value(true)
                        .help("Path to text eds"),
                )
                .arg(
                    Arg::new("type")
                        .short('t')
                        .long("eds-type")
                        .multiple_values(false)
                        .default_value("i")
                        .help("e (for elastic) or i (for inelastic)"),
                )
                .arg(
                    Arg::new("v")
                        .short('v')
                        .multiple_occurrences(true)
                        .help("Sets the level of verbosity [default: 0]"),
                ),
        )
        .get_matches();

    let handle_align_args = |matches: &clap::ArgMatches| -> AppConfig {
        // Gets a value for config if supplied by user, or defaults to "default.conf"
        let fasta: &str = matches.value_of("fasta").unwrap();
        let edt: &str = matches.value_of("eds").unwrap();
        let verbosity: u8 = matches.occurrences_of("v") as u8;

        AppConfig {
            alignment_options: SubCommand::Align,
            fasta: String::from(fasta),
            edt: String::from(edt),
            convert_to: types::Format::MSA,
            handle_as: types::Format::Degenerate,
            verbosity,
            k: 0,
            bind: true,
            bind_above: 0,
            bind_below: 0,
            penalties: types::Penalties {
                a: 0,
                x: 1,
                o: 2,
                e: 1,
            },
        }
    };

    let handle_convert_args = |matches: &clap::ArgMatches| -> AppConfig {
        let edt: &str = matches.value_of("eds").unwrap();
        let verbosity: u8 = matches.occurrences_of("v") as u8;

        let t: char = matches
            .value_of("type")
            .unwrap()
            .parse::<char>()
            .unwrap_or('i');

        let handle_as = match t {
            'e' => types::Format::Elastic,
            _ => types::Format::Degenerate,
        };

        AppConfig {
            alignment_options: SubCommand::Convert,
            fasta: String::new(),
            edt: String::from(edt),
            verbosity,
            k: 0,
            bind: true,
            convert_to: types::Format::MSA,
            handle_as,
            bind_above: 0,
            bind_below: 0,
            penalties: types::Penalties {
                a: 0,
                x: 0,
                o: 0,
                e: 0,
            },
        }
    };

    let handle_stats_args = |matches: &clap::ArgMatches| -> AppConfig {
        let edt: &str = matches.value_of("eds").unwrap();
        let t: char = matches
            .value_of("type")
            .unwrap()
            .parse::<char>()
            .unwrap_or('i');

        let handle_as = match t {
            'e' => types::Format::Elastic,
            _ => types::Format::Degenerate,
        };

        AppConfig {
            alignment_options: SubCommand::Stats,
            fasta: String::new(),
            edt: String::from(edt),
            verbosity: 0,
            k: 0,
            bind: true,
            convert_to: types::Format::MSA,
            handle_as,
            bind_above: 0,
            bind_below: 0,
            penalties: types::Penalties {
                a: 0,
                x: 0,
                o: 0,
                e: 0,
            },
        }
    };

    let conf = match matches.subcommand() {
        Some(("align", matches)) => handle_align_args(matches),
        Some(("convert", matches)) => handle_convert_args(matches),
        Some(("stats", matches)) => handle_stats_args(matches),
        Some((arg, _)) => panic!("[edsa::cli::start] Could not find argument {}", arg),
        arg @ _ => panic!("[edsa::cli::start] Could not find argument {:?}", arg),
    };

    conf
}
