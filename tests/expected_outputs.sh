#!/bin/bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" > $OUTDIR/hello1.txt

# vec![]
echo "Hello" "there" > $OUTDIR/hello2.txt 

# omit newline
echo -n "Hello there" > $OUTDIR/hello1.n.txt 
echo -n "Hello" "there" > $OUTDIR/hello2.n.txt