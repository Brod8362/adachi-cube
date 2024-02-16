#!/bin/sh
FILES=../scenes/*.blend

rm -rf render_output
mkdir render_output

for f in $FILES; do
	blender -b $f -o render_output/$(basename $f) -a
done
