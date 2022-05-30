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
    pub verbosity: u8,
}
