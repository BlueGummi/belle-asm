#!/bin/bash

spinner() {
    local pid=$1
    local delay=0.1
    local spin='/-\|'
    local i=0
    local msg=$2
    while ps -p $pid > /dev/null; do
        local temp=${spin:i++%${#spin}:1}
        printf "\r$msg $temp"
        sleep $delay
    done
    printf "\rDone!                \n"
}

print_help() {
    printf "The build script for the BELLE programs and utilities\n\n"
    printf "\e[4mUsage\e[0m: $1 [OPTIONS]\n"
    printf "Options:\n"
    printf "  -c, --clean        Clean the build directories (doesn't build)\n"
    printf "  -w, --with-cleanup Clean directories after building\n"
    printf "  -i, --install      Install the BELLE programs and utilities\n"
    exit 0
}
default_build() {
    if [ -d "bin" ]; then
        true
    else
        mkdir bin
    fi
    cd basm
    if [ $clean ]; then
        cargo clean --quiet
        cd ..
        cd bdump
        make clean --quiet
        echo "Cleaned up!"
        exit 0
    fi
    cargo build --release --quiet & # spin spin spin
    pid=$!
    spinner $pid "\nBuilding BELLE-asm..."
    cd ..
    cp -f basm/target/release/basm bin
    echo "basm build complete"
    cd bdump
    make --quiet &
    pid=$!
    spinner $pid "\nBuilding BELLE-dump..."
    cd ..
    cp -f bdump/bdump bin
    echo "bdump build complete"
    echo "\nBuild complete"
    if [ $with_cleanup ]; then
        echo "Cleaning up..."
        cd basm
        cargo clean --quiet
        cd ..
        cd bdump
        make clean --quiet
        cd ..
        echo "Cleaned up!"
        exit 0
    fi
    exit 0
}

if [ $# -eq 1 ]; then
    if [ "$1" == "--clean" ]; then
        clean=true
    elif [ "$1" == "-c" ]; then
        clean=true
    elif [ "$1" == "--with-cleanup" ]; then
        with_cleanup=true
    elif [ "$1" == "-w" ]; then
        with_cleanup=true
    elif [ "$1" == "--help" ]; then
        print_help $0
    elif [ "$1" == "-h" ]; then
        print_help $0
    elif [ "$1" == "help" ]; then
        print_help $0
    fi
fi
default_build
