#!/bin/bash

if [ "$PRINTER" = "" ]; then
	echo 'Please set printer device. Use'
	echo '		export PRINTER=/dev/<printer_device>'
	exit 1
fi
if [ "$Z_REST" = "" ]; then
	echo 'Please set Z_REST'
	exit 1
fi
if [ "$Z_TOUCH" = "" ]; then
	echo 'Please set Z_TOUCH'
	exit 1
fi
if [ "$Z_DRILL" = "" ]; then
	echo 'Please set Z_DRILL'
	exit 1
fi
if [ "$DRILL_SPEED" = "" ]; then
	echo 'Please set DRILL_SPEED'
	exit 1
fi
if [ "$POSITION_SPEED" = "" ]; then
	echo 'Please set POSITION_SPEED'
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

function log() {
	echo "================ $1 ================" # >> $LOG_FILE
}

function set_parameter() {
	echo "$1" > $PRINTER
	sleep 0.2
	echo "$1"
}

function move() {
	COMMAND="$1"
	printf '%s\n' "$COMMAND" | while IFS= read -r line; do
		echo "$line" > $PRINTER
		echo "$line"
		echo "Waiting for status change..." >> $LOG_FILE
		# If we don't wait a bit, the state might be the previous "ok"
		sleep 2
		log_out=`tail -n1 "$LOG_FILE" | grep "^ok"`
		while [ "$log_out" = "" ]; do
			log_out=`tail -n1 $LOG_FILE | grep "^ok"`
			sleep 0.2
		done
	done
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

function make_via_at() {
	if ! [ -f "$1" ]; then
		echo "Input via file missing"
		exit 1
	fi
	VIA_FILE="$1"
	log "Go up not to scratch"
	move "G1 Z${Z_REST}"
	log "Go above the first point of the hole"
	VIA_START_COMMAND="`head -n1 ${VIA_FILE}` Z${Z_REST} F${POSITION_SPEED}"
	move "${VIA_START_COMMAND}"
	log "Go down to touch the plate"
	move "G1 Z${Z_TOUCH}"
	log "Drill the first hole"
	set_parameter "G1 F${DRILL_SPEED}"
	move "G1 Z${Z_DRILL}"
	log "Start drilling"
	move "`cat ${VIA_FILE}`"
	log "Go up to rest hight"
	set_parameter "G1 F${POSITION_SPEED}"
	move "G1 Z${Z_REST}"
}

function shutdown_motors() {
	set_parameter "M84 X Y E Z"
}
