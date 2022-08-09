#!/usr/bin/env bash

ROOT="tests/inputs"
FILES="$ROOT/m00p0801.txt $ROOT/m01a0208.txt $ROOT/m04a0101.txt $ROOT/m12a0102.txt"
OUT_DIR="tests/expected"
ACTORS="tests/test_actors.json"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $FILES; do
    BASENAME=$(basename "$FILE")
    ./exptext   --db ${ACTORS}           $FILE > ${OUT_DIR}/${BASENAME}.out
    ./exptext   --db ${ACTORS}  -f       $FILE > ${OUT_DIR}/${BASENAME}.f.out
    ./exptext   --db ${ACTORS}  -n       $FILE > ${OUT_DIR}/${BASENAME}.n.out
    ./exptext   --db ${ACTORS}  -s       $FILE > ${OUT_DIR}/${BASENAME}.s.out
    ./exptext   --db ${ACTORS}  -fn      $FILE > ${OUT_DIR}/${BASENAME}.fn.out
    ./exptext   --db ${ACTORS}  -fs      $FILE > ${OUT_DIR}/${BASENAME}.fs.out
    ./exptext   --db ${ACTORS}  -ns      $FILE > ${OUT_DIR}/${BASENAME}.ns.out
    ./exptext   --db ${ACTORS}  -l e     $FILE > ${OUT_DIR}/${BASENAME}.l-e.out
    ./exptext   --db ${ACTORS}  -nl e    $FILE > ${OUT_DIR}/${BASENAME}.nl-e.out
    ./exptext   --db ${ACTORS}  -fnsl e  $FILE > ${OUT_DIR}/${BASENAME}.fnsl-e.out 
done

./exptext  --db ${ACTORS}          $FILES > $OUT_DIR/all.out
./exptext  --db ${ACTORS}  -f      $FILES > $OUT_DIR/all.f.out
./exptext  --db ${ACTORS}  -n      $FILES > $OUT_DIR/all.n.out
./exptext  --db ${ACTORS}  -s      $FILES > $OUT_DIR/all.s.out
./exptext  --db ${ACTORS}  -fn     $FILES > $OUT_DIR/all.fn.out
./exptext  --db ${ACTORS}  -fs     $FILES > $OUT_DIR/all.fs.out
./exptext  --db ${ACTORS}  -ns     $FILES > $OUT_DIR/all.ns.out
./exptext  --db ${ACTORS}  -l e    $FILES > $OUT_DIR/all.l-e.out
./exptext  --db ${ACTORS}  -nl e   $FILES > $OUT_DIR/all.nl-e.out
./exptext  --db ${ACTORS}  -fnsl e $FILES > $OUT_DIR/all.fnsl-e.out
