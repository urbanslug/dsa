#!/usr/bin/env bash

# indel or SNP count
COUNT=$1

# Path to EDS file from EDSRand
EDS=x.eds

# Path to Multiple Sequence Alignment from dsa convert
MSA=x.msa

# One of the sequences from $MSA, this is the pattern we will use (P)
INPUT_FA=0.fa


echo -e "Count: $COUNT";
echo -e "EDS: $EDS"

for change_type in indels snps
do

    if [ $COUNT -eq 0 ]; then
        SIMULATED_FA=$INPUT_FA
    else
        SIMULATED_FA=$change_type.$COUNT.fa
    fi

    # GFA from minigraph or vg
    VG_GFA=vg.gfa
    MG_GFA=mg.gfa

    GA_OUT=ga.$change_type.$COUNT.aln
    MG_OUT=mg.$change_type.$COUNT.aln
    DSA_OUT=ed.$change_type.$COUNT.aln

    MG_GAF=mg.$change_type.$COUNT.gaf
    GA_GAF=ga.$change_type.$COUNT.gaf

    # dsa
    echo -e "dsa out: $DSA_OUT";
    if [[ -f $DSA_OUT ]]; then
        echo "Deleting ${DSA_OUT}"
        rm $DSA_OUT
    fi

    # GAF=ed.$change_type.$COUNT.gaf
    $( dsa align -t d $EDS $SIMULATED_FA 2>/dev/null | cig > $DSA_OUT )


    # minigraph
    echo -e "minigraph out: $MG_OUT";
    if [[ -f $MG_OUT ]]; then
        echo "Deleting ${DSA_OUT}"
        rm $MG_OUT
    fi

    $( minigraph -cx lr $MG_GFA $SIMULATED_FA -o $MG_GAF )
    $( sed -e 's/.*cg:Z:\(.*\)$*/\1/' $MG_GAF | cig > $MG_OUT  )

    # GraphAligner
    echo -e "GraphAligner out: $GA_OUT";

    if [[ -f $GA_OUT ]]; then
        echo "Deleting ${DSA_OUT}"
        rm $GA_OUT
    fi

    $( GraphAligner -g $VG_GFA -f $SIMULATED_FA -a $GA_GAF -x vg
    )
    $( sed -e 's/.*cg:Z:\(.*\)$*/\1/' $GA_GAF | cig > $GA_OUT  )

done
