#!/bin/bash
DIR="bin"
FILE1="basm"
FILE2="bdump"

install() {
    echo "Installing..."
    cp $DIR/$FILE1 ~/.local/bin
    cp $DIR/$FILE2 ~/.local/bin
    echo "Installation complete."
    if [ $cleanup ]; then
        echo "Deleting '$DIR'..."
        rm -rf $DIR
    fi
}

print_help() {
    printf "The install script for the BELLE programs and utilities\n\n"
    printf "\e[4mUsage\e[0m: $1 [OPTIONS]\n"
    printf "Options:\n"
    printf "  -c, --cleanup        Clean the binary directory\n"
    exit 0
}

if [ $# -eq 1 ]; then
    if [ "$1" == "--cleanup" ]; then
        cleanup=true
    elif [ "$1" == "-c" ]; then
        cleanup=true
    elif [ "$1" == "--help" ]; then
        print_help $0
    elif [ "$1" == "-h" ]; then
        print_help $0
    elif [ "$1" == "help" ]; then
        print_help $0
    fi
fi

if [ ! -d "$DIR" ]; then
    echo "Directory '$DIR' does not exist."
    BUILD=true
else
    FILE1_PATH="$DIR/$FILE1"
    FILE2_PATH="$DIR/$FILE2"

    if [ ! -f "$FILE1_PATH" ] && [ ! -f "$FILE2_PATH" ]; then
        echo "Both binaries '$FILE1' and '$FILE2' do not exist in '$DIR'."
        BUILD=true
    elif [ ! -f "$FILE1_PATH" ]; then
        echo "Binary '$FILE1' does not exist in '$DIR'."
        BUILD=true
    elif [ ! -f "$FILE2_PATH" ]; then
        echo "Binary '$FILE2' does not exist in '$DIR'."
        BUILD=true
    fi
fi

if [ "$BUILD" = true ]; then
    read -p "Do you want to build BELLE to create the binaries?? [Y/n]: " ANSWER
    ANSWER=${ANSWER:-Y} # default to 'Y' if no input

    if [[ "$ANSWER" =~ ^[Yy]$ ]]; then
        echo "Building the project..."
        ./build.sh
    else
        echo "Exiting"
    fi
else
    install
fi
