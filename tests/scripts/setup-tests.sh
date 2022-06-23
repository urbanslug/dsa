#!/usr/bin/env bash

# Constants
# ---------
SIZE=100000

# Other vars
# ----------

# Path to EDS file from EDSRand
EDS=x.eds

# Path to Multiple Sequence Alignment from dsa convert
MSA=x.msa

VARIATION_GRAPH=x.vg
VG_GFA=vg.gfa
MG_GFA=mg.gfa

# Functions
function percent () {
    divergence=$1
    x=$(echo "scale=0; $divergence * $SIZE / 100" | bc )
    echo $x # return number
}

# Set up
dir=$(pwd | rev |  cut -d'/' -f1 | rev)
d=$( echo $dir | gawk '{ match($0, /d([0-9]+)/, arr); if(arr[1] != "") print arr[1] }' )
s=$( echo $dir | gawk '{ match($0, /s([0-9]+)/, arr); if(arr[1] != "") print arr[1] }' )
l=$( echo $dir | gawk '{ match($0, /l([0-9]+)/, arr); if(arr[1] != "") print arr[1] }' )

# Generate EDS
# ------------
simed -d $d -s $s -l $l $SIZE > $EDS

# Extract MSA from EDS
# --------------------

dsa convert -t d $EDS > $MSA

# Extract separate fasta files from MSA
# -------------------------------------

num_lines=$(wc -l "$MSA" | awk '{print $1}')
for ((i=1; i<num_lines; i=i+2))
do
    stop=$((i+1))
    out=$((i/2))
    echo -e "$i to $stop > $out.fa"
    awk -v start="$i" -v stop="$stop" 'NR==start,NR==stop' $MSA > "$out.fa"
done


# Generate GFA
# ------------

# vg
vg construct --flat-alts -M $MSA > $VARIATION_GRAPH
vg view $VARIATION_GRAPH > $VG_GFA

# minigraph
# 16 threads
fasta_files=$(ls *.fa | egrep '^[0-9]+\.fa' | tr '\n' ' ')
minigraph -cxggs -t16 $fasta_files > $MG_GFA


# Simulation
# ----------

# Simulate variants using simuG
main_fasta=0.fa
simuG_output="./simulated"
if [[ -d $simuG_output ]]; then
    # echo "Deleting ${simuG_output}"
    rm $simuG_output
fi

mkdir $simuG_output


# simulate SNPs at 0.1% (100) and 1% (1,000).

for divergence in 0.1 1
do
    changes=$(percent $divergence)

    # SNPs
    simuG -refseq $main_fasta -snp_count $changes -prefix "$simuG_output/snps.$changes"
    ln -s "$simuG_output/snps.$changes.simseq.genome.fa" "./snps.$changes.fa"

    # indels
    simuG -refseq $main_fasta -indel_count $changes -prefix "$simuG_output/indels.$changes"
    ln -s "$simuG_output/indels.$changes.simseq.genome.fa" "./indels.$changes.fa"
done
