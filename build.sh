#!/bin/bash

print_message() {
    local message="$1"
    local color="$2"
    case "$color" in
        green) echo -e "\e[32m$message\e[0m" ;;
        red) echo -e "\e[31m$message\e[0m" ;;
        yellow) echo -e "\e[33m$message\e[0m" ;;
        blue) echo -e "\e[34m$message\e[0m" ;;
        *) echo "$message" ;;
    esac
}

clear_line() {
    printf "\r\033[K"
}

spinner() {
    local pid=$1
    local delay=0.1
    local spin='/-\|'
    local msg=$2
    if [ "$quiet" != true ]; then
        print_message "$msg" blue
    fi
    local i=0
    while ps -p $pid > /dev/null; do
        local temp=${spin:i++%${#spin}:1}
        if [ "$quiet" != true ]; then
            printf "\r$temp"
        fi
        sleep $delay
    done
    clear_line
    if [ "$quiet" != true ]; then
        print_message "Done!" green
    fi
}

bouncing_text() {
    local pid=$1
    local msg=$2
    if [ "$quiet" != true ]; then
        print_message "$msg" blue
    fi
    local delay=0.1
    local spaces=0
    local direction=1
    local max_spaces=10

    while ps -p $pid > /dev/null; do
        if [ "$quiet" != true ]; then
            printf "\r%${spaces}sLoading" ""
        fi
        sleep $delay
        spaces=$((spaces + direction))
        if [ $spaces -eq $max_spaces ] || [ $spaces -eq 0 ]; then
            direction=$((direction * -1))
        fi
    done
    clear_line
    if [ "$quiet" != true ]; then
        print_message "Done!" green
    fi
}

moving_text() {
    local pid=$1
    local msg=$2
    if [ "$quiet" != true ]; then
        print_message "$msg" blue
    fi
    local delay=0.1
    local position=0
    local width=$(tput cols)

    while ps -p $pid > /dev/null; do
        if [ "$quiet" != true ]; then
            printf "\r%${position}s%s" "" "$msg"
        fi
        sleep $delay
        position=$(( (position + 1) % (width + ${#msg}) ))
    done
    clear_line
    if [ "$quiet" != true ]; then
        print_message "Done!" green
    fi
}

print_help() {
    printf "The build script for the BELLE programs and utilities\n\n"
    printf "\e[4mUsage\e[0m: $1 [OPTIONS]\n"
    printf "Options:\n"
    printf "  -c, --clean        Clean the build directories (doesn't build)\n"
    printf "  -w, --with-cleanup Clean directories after building\n"
    printf "  -q, --quiet        Suppress output\n"
    printf "  -h, --help         Display this help message\n"
    exit 0
}

default_build() {
    if [ ! -d "bin" ]; then
        mkdir bin
    fi

    cd basm
    if [ "$clean" ]; then
        cargo clean --quiet
        cd ..
        cd bdump
        make clean --quiet
        if [ "$quiet" != true ]; then
            print_message "Cleaned up!" green
        fi
        exit 0
    fi

    cargo build --release --quiet & 
    pid=$!

    local animations=(spinner bouncing_text moving_text)
    local selected_animation=${animations[RANDOM % ${#animations[@]}]}
    if [ "$quiet" != true ]; then
        echo ""
    fi
    $selected_animation $pid "Building BELLE-asm..."
    
    clear_line
    cd ..
    cp -f basm/target/release/basm bin
    if [ "$quiet" != true ]; then
        print_message "basm build complete" green
    fi

    cd bdump
    make --quiet &
    pid=$!

    selected_animation=${animations[RANDOM % ${#animations[@]}]}
    if [ "$quiet" != true ]; then
        echo ""
    fi
    $selected_animation $pid "Building BELLE-dump..."
    
    clear_line
    cd ..
    cp -f bdump/bdump bin
    if [ "$quiet" != true ]; then
        print_message "bdump build complete" green
    fi
    
    if [ "$quiet" != true ]; then
        printf "\n"
        print_message "Build complete" green
    fi
    
    if [ "$with_cleanup" ]; then
        if [ "$quiet" != true ]; then
            print_message "Cleaning up..." blue
        fi
        cd basm
        cargo clean --quiet
        cd ..
        cd bdump
        make clean --quiet
        cd ..
        if [ "$quiet" != true ]; then
            print_message "Cleaned up!" green
        fi
        exit 0
    fi
    exit 0
}

for arg in "$@"; do
    case $arg in
        --clean|-c)
            clean=true
            ;;
        --with-cleanup|-w)
            with_cleanup=true
            ;;
        --quiet|-q)
            quiet=true
            ;;
        --help|-h|help)
            print_help "$0"
            ;;
    esac
done

default_build
