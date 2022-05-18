// --------
// Aliases
// --------

// --------
// Structs
// --------

#[derive(Debug)]
pub enum Operation {
    I,
    D,
    M,
    R,
}

impl Operation {
    pub fn char_rep(&self) -> char {
        match self {
            Operation::I => 'I',
            Operation::D => 'D',
            Operation::M => 'M',
            Operation::R => 'R',
        }
    }
}

pub struct Config {
    pub penalties: Penalties,
    pub verbosity: u8,
}

#[derive(Debug)]
pub enum SubCommand {
    Convert,
    Align,
    Stats,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Format {
    Degenerate,
    Elastic,
    Fasta,
    MSA,
}

#[derive(Debug)]
pub struct Penalties {
    pub a: usize, // match
    pub x: usize, // mismatch
    pub o: usize, // gap open
    pub e: usize, // gap extension
}

#[derive(Debug)]
pub struct AppConfig {
    pub alignment_options: SubCommand,
    pub penalties: Penalties,
    pub fasta: String,
    pub convert_to: Format,
    pub handle_as: Format,
    pub edt: String,
    pub k: usize,
    pub verbosity: u8,
    pub bind: bool,
    pub bind_above: usize,
    pub bind_below: usize,
}
