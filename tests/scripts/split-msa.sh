#!/usr/bin/env bash
FILE=$1
# OUT_DIR='./'

num_lines=$(wc -l "$FILE" | awk '{print $1}')
for ((i=1; i<num_lines; i=i+2))
do
    stop=$((i+1))
    out=$((i/2))
    echo -e "$i to $stop > $out.fa"
    awk -v start="$i" -v stop="$stop" 'NR==start,NR==stop' $FILE > "$out.fa"
done
