# dsa
(Base Level) Degenerate String Aligner

## Install
Install cargo and rust.
See the instructions here
[Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)


Clone and install dsa
```
git clone https://github.com/urbanslug/dsa.git
cd dsa
cargo install --path .
```

## Usage

Help
```
dsa -h
```

## Example

Generate degenerate string
Use https://github.com/urbanslug/simed

```
simed 100 > x.eds
```

Extract fasta MSA from `.eds`.

_Does not extract all possible paths but only a subset of them._

```
dsa convert x.eds > x.msa
```


Split msa into multiple fasta files

```
./scripts/split-msa.sh x.msa
```


Align one of the fasta files to the eds
```
dsa align x.eds 0.fa > x.aln
```
