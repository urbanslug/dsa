# dsa
(Base Level) Degenerate String Aligner

## Install
```
git clone https://github.com/urbanslug/dsa.git
cd dsa
cargo install --path .
```

## Running

Align
```
dsa align x.eds 0.fa > x.aln
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

Help
```
dsa -h
```
