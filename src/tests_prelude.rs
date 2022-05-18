#![cfg(test)]
use crate::types;

pub fn get_config() -> types::AppConfig {
    types::AppConfig {
        alignment_options: types::SubCommand::Degenerate,
        fasta: String::from(""),
        edt: String::from(""),
        verbosity: 3,
        k: 3,
        bind: false,
        bind_above: 500,
        bind_below: 500,
        convert_to: types::Format::Fasta,
        penalties: types::Penalties {
            a: 0,
            x: 1,
            o: 2,
            e: 1,
        },
    }
}
