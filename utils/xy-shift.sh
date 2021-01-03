#!/bin/bash

if [ "$1" = "" ]; then
	echo 'Arguent missing: input gcode file'
	exit 1
fi
IN_FILE=$1
if ! [ -f $IN_FILE ]; then
	echo "File not found: ${IN_FILE}"
	exit 1
fi

if [ "$2" = "" ]; then
	echo "New x-coordinate missing"
	exit 1
fi

if [ "$3" = "" ]; then
	echo "New y-coordinate missing"
	exit 1
fi

OUT_FILE="${IN_FILE}-relative"
TMP_FILE="${IN_FILE}-tmp"

touch $OUT_FILE
echo "; New coordinates: X=$2, Y=$3" > $OUT_FILE
cat $IN_FILE >> $OUT_FILE
cp $OUT_FILE $TMP_FILE


for COORD in X Y; do
ABS="`cat $IN_FILE | sed 's/.*X//' | sed 's/\..*//' | sed 's/ .*//g' | sort | uniq`"
	for VAL in $ABS; do
		if [ "$COORD" = "X" ]; then
			REL=$((VAL-110+$2))
		else
			REL=$((VAL-110+$3))
		fi
		echo "Replacing $VAL with $REL for $COORD"
		cat $TMP_FILE | sed "s/${COORD}${VAL}/${COORD}${REL}/g" > $OUT_FILE
		cp $OUT_FILE $TMP_FILE
	done
done
rm $TMP_FILE