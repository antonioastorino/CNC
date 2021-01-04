#!/bin/bash

if [ "$PRINTER" = "" ]; then
	echo 'Please set printer device. Use'
	echo '		export PRINTER=/dev/<printer_device>'
	exit 1
fi

X_START=110
Y_START=110
X_STEP=0
NX_STEPS=1
export Z_REST=5
export Z_TOUCH=3
export Z_DRILL=1
export POSITION_SPEED=6000
export DRILL_SPEED=50

. cnc-controller/functions.sh

# Always home first
home
# # Drill two holes

# # drill_at 20 30
# # drill_at 40 30
# home
# Shift the pattern
for i in $(seq 0 $((NX_STEPS-1))); do
	X_CURR=$((X_START+X_STEP*i))
    ./utils/xy-shift.sh gcode-files/via-4mm.gcode $X_CURR $X_START
	make_via_at "gcode-files/via-4mm.gcode-relative"
done

shutdown_motors
# Don't forget to kill 
kill_tail
