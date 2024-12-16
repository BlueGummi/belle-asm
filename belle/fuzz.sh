#!/bin/bash

trap "exit" SIGINT

run_belle() {
    while true; do
        head -c 256 /dev/urandom > random_data.bin
	timeout 2 belle random_data.bin
        output=$(timeout 2 belle random_data.bin 2>&1)
        exit_code=$?

        if echo "$output" | grep -q "panic"; then
            echo "$output" >> panic_errors.txt
        fi

        # sleep 0.2
    done
}

run_belle
wait
