#!/bin/bash

if [ "$PRINTER" = "" ]; then
	echo 'Please set printer device. Use'
	echo '		export PRINTER=/dev/<printer_device>'
	exit 1
fi

LOG_FILE="status.log"
function kill_tail() {
	TAIL_PROCESS=`ps | grep 'tail -f' | grep $PRINTER | awk '{print $1}'`
	for PID in $TAIL_PROCESS; do kill $PID; done
}

kill_tail
rm $LOG_FILE
tail -f $PRINTER >> $LOG_FILE &

X_START=20
Y_START=100
X_STEP=20
NX_STEPS=5
Z_REST=5
Z_TOUCH=3
Z_DRILL=0

POSITION_SPEED=8000
DRILL_SPEED=50

function log() {
	echo "================ $1 ================" >> $LOG_FILE
}

function set_parameter() {
	echo "$1" > $PRINTER
}

function move() {
	# read -r -t3 output <&3
	echo "$1" > $PRINTER
	echo "Waiting for status change..." >> $LOG_FILE
	# If we don't wait a bit, the state might be the previous "ok"
	sleep 2
	log_out=`tail -n1 "$LOG_FILE" | grep "^ok"`
	while [ "$log_out" = "" ]; do
		log_out=`tail -n1 $LOG_FILE | grep "^ok"`
		sleep 0.2
	done
	# kill_tail
}

function home() {
	log "Homing"
	move "G28"
}

function drill_at() {
	log "Going up to ($Z_REST)"
	move "G1 Z${Z_REST} F${POSITION_SPEED}"
	log "Positioning at ($1, $2 ,$Z_REST)"
	move "G1 X$1 Y$2 Z${Z_REST}"
	log "Going down to ($1, $2 ,$Z_TOUCH)"
	move "G1 Z${Z_TOUCH}"
	log "Drilling at ($1, $2 ,$Z_DRILL)"
	move "G1 Z${Z_DRILL} F${DRILL_SPEED}"
	move "G1 Z${Z_REST} F${POSITION_SPEED}"
}

shutdown_motors() {
	set_parameter "M84 X Y E Z"
}

# Drill two holes
home
drill_at 20 30
drill_at 40 30
home
shutdown_motors
kill_tail
