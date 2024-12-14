while true; do
    head -c 256 /dev/urandom > random_data.bin

    output=$(timeout 2 belle random_data.bin 2>&1)
    timeout 2 belle random_data.bin
    exit_code=$?

    if [ $exit_code -eq 101 ]; then
        echo "Error occurred with return code 101 at $(date):" >> errors.txt
        echo "$output" >> errors.txt
    fi

    sleep 0.2
done
