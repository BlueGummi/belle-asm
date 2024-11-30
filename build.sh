#!/bin/bash
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"
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
clean() {
    print_message "Cleaning up..." blue
    cd basm
    cargo clean --quiet    
    cd ..
    cd bdump
    make clean --quiet
    cd ..
    cd belle
    cargo clean --quiet
    cd ..
    print_message "Cleaned up!" green
}
spinner() {
    local pid=$1
    local delay=0.1
    local spin='/-\|'
    local msg=$2
    print_message "$msg" blue
    local i=0
    while ps -p $pid > /dev/null; do
        local temp=${spin:i++%${#spin}:1}
        printf "\r$temp"
        sleep $delay
    done
    clear_line
    print_message "Done!" green
}

print_help() {
    printf "The build script for the BELLE programs and utilities\n\n"
    printf "\e[4mUsage\e[0m: $1 [OPTIONS] [TARGETS]\n"
    printf "Options:\n"
    printf "  -c, --clean        Clean the build directories (doesn't build)\n"
    printf "  -w, --with-cleanup Clean directories after building\n"
    printf "  -q, --quiet        Suppress output\n"
    printf "  -h, --help         Display this help message\n"
    printf "\nTargets:\n"
    printf "  bdump, basm, belle (default: all)\n"
    exit 0
}

default_build() {
    if [ ! -d "bin" ]; then
        mkdir bin
    fi
    if [ "$clean" ]; then
        clean
        exit 0
    fi
    for target in "${targets[@]}"; do
        case "$target" in
            basm)
                cd basm
                cargo build --release --quiet &
                pid=$!
                spinner $pid "Building BELLE-asm..."
                cp -f target/release/basm ../bin
                cd ..
                print_message "basm build complete" green
                ;;
            bdump)
                cd bdump
                make --quiet &
                pid=$!
                spinner $pid "Building BELLE-dump..."
                cp -f bdump ../bin
                cd ..
                print_message "bdump build complete" green
                ;;
            belle)
                cd belle
                cargo build --release --quiet &
                pid=$!
                spinner $pid "Building BELLE..."
                cp -f target/release/belle ../bin
                cd ..
                print_message "belle build complete" green
                ;;
        esac
    done

    if [ "$with_cleanup" ]; then
        clean
    fi

    print_message "Build complete" green
    exit 0
}

targets=()

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
        bdump|basm|belle)
            targets+=("$arg")
            ;;
    esac
done

if [ ${#targets[@]} -eq 0 ]; then
    targets=(bdump basm belle)
fi

default_build
