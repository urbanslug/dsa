#!/usr/bin/env bash

# indel or SNP count
COUNT=$1
VARIATION=$2

# Path to EDS file from EDSRand
EDS=x.eds

# One of the sequences from $MSA, this is the pattern we will use (P)
INPUT_FA=0.fa

# Path to Multiple Sequence Alignment from dsa convert
MSA=x.msa


# Path to Fasta file from simuG
if [ $COUNT -eq 0 ]; then
    SIMULATED_FA=$INPUT_FA
else
    SIMULATED_FA=$VARIATION.$COUNT.fa
fi


# GAF
GAF=x.$TOOL.gaf

# GFA from abPOA or vg
# INPUT_GFA=x.$TOOL.gfa
VG_GFA=vg.gfa
MG_GFA=mg.gfa

RUNS=10

GA_OUT=ga.$VARIATION.$COUNT.time
MG_OUT=mg.$VARIATION.$COUNT.time
DSA_OUT=ds.$VARIATION.$COUNT.time


echo -e "Count: $COUNT";
echo -e "EDS: $EDS"

echo -e "dsa out: $DSA_OUT";

if [[ -f $DSA_OUT ]]; then
    # echo "Deleting ${DSA_OUT}"
    rm $DSA_OUT
fi

for (( c=1; c<=$RUNS; c++ ))
do
    $( /usr/bin/time -f"%S\t%M" dsa align $EDS $SIMULATED_FA 2>&1 | tail -n 1 >> $DSA_OUT )
done

# awk -v lines="$RUNS" -F'\t' '{sum+=$1;}END{ if(lines) print sum/lines; else print "oo" }' $DSA_OUT
avg_time=$(awk -v lines="$RUNS" -F'\t' '{sum+=$1;}END{ print sum/lines; }' $DSA_OUT)
avg_mem=$(awk -v lines="$RUNS" -F'\t' '{sum+=$2;}END{ print sum/lines; }'  $DSA_OUT)
echo -e "----------------------" >> $DSA_OUT
echo -e "$avg_time\t$avg_mem" >> $DSA_OUT


echo -e "minigraph out: $MG_OUT";

if [[ -f $MG_OUT ]]; then
    # echo "Deleting ${MG_OUT}"
    rm $MG_OUT
fi

for (( c=1; c<=$RUNS; c++ ))
do
    MG_GAF=x.mg.$c.gaf
    $( /usr/bin/time -f"%S\t%M"  minigraph -cx lr $MG_GFA $SIMULATED_FA -o $MG_GAF 2>&1 | tail -n 1 >> $MG_OUT )
    rm $MG_GAF
done

avg_time=$(awk -v lines="$RUNS" -F'\t' '{sum+=$1;}END{ print sum/lines; }' $MG_OUT)
avg_mem=$(awk -v lines="$RUNS" -F'\t' '{sum+=$2;}END{ print sum/lines; }'  $MG_OUT)
echo -e "----------------------" >> $MG_OUT
echo -e "$avg_time\t$avg_mem" >> $MG_OUT


echo -e "GraphAligner out: $GA_OUT";


if [[ -f $GA_OUT ]]; then
    # echo "Deleting ${GA_OUT}"
    rm $GA_OUT
fi

for (( c=1; c<=$RUNS; c++ ))
do
    GA_GAF=x.ga.$c.gaf
    $( /usr/bin/time -f"%S\t%M"  GraphAligner -g $VG_GFA -f $SIMULATED_FA -a $GA_GAF -x vg 2>&1 | tail -n 1 >> $GA_OUT )
    rm $GA_GAF
done

avg_time=$(awk -v lines="$RUNS" -F'\t' '{sum+=$1;}END{ print sum/lines; }' $GA_OUT)
avg_mem=$(awk -v lines="$RUNS" -F'\t' '{sum+=$2;}END{ print sum/lines; }'  $GA_OUT)
echo -e "----------------------" >> $GA_OUT
echo -e "$avg_time\t$avg_mem" >> $GA_OUT
