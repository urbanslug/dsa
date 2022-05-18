# extract cigar
sed -e 's/.*cg:Z:\(.*\)$*/\1/' $FILE | cig | wl-copy

# extract time and memory
tail -n 1 $TIME_FILE | awk -F'\t' '{printf "%.3f | %.0f", $1, $2}' | wl-copy
