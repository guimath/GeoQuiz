#!/bin/sh
. ./env.sh

if [ $# -eq 0 ]; then
    echo "Error: No arguments provided."
    echo "Use 'run_emulator' or 'run_android'"
    exit 1
fi

case "$1" in 
    run_emulator)
        x run --release --device $ADB_EMULATOR_HANDLE --arch arm64
        ;;
    
    run_android)
        x run --release --device $ADB_HANDLE
        ;;
    
    debug_android)
        x run --device $ADB_HANDLE
        ;;

    *)
        echo "Error: Invalid argument."
        echo "Use 'run_emulator' or 'run_android'"
        exit 1
        ;;
esac
