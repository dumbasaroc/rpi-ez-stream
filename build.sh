#!/usr/bin/env bash

function help() {
    echo "========================"
    echo "| EZ Stream Build Tool |"
    echo "========================"
    echo
    echo
    echo "Available options:"
    echo "  - build (default): Builds and packages EZStream into the 'build' directory"
    echo "  - clean: Cleans up build files and the most recently generated release."
    echo

    usage

    echo
    echo "Coded by DumbAsARoc"
}

function usage() {
    echo "Usage:"
    echo "  - build.sh --help"
    echo "    Shows the help screen"
    echo "  - build.sh <option>"
    echo "    Performs <option>. See the help screen for more info."
}

function last_cmd_did_fail() {
    RETVAL=$?
    echo "$RETVAL"
    if [[ "$RETVAL" == "0" ]]; then
        echo "last command worked"
        return 1
    else
        echo "last command failed"
        return 0
    fi
}

function on_error() {
    echo "An error occurred... Cleaning up."
    cleanup

    exit 1
}

function cleanup() {
    rm -rf release/
    cargo clean
}

function build() {
    echo "Building EZStream..."

    cargo build --release
    rm -rf release/
    mkdir -p release

    # Copy required files into their proper places
    echo "Copying files to the release package..."

    echo "  - Copying settings..."
    mkdir -p release/data
    cp data/gschemas.compiled release/data/ || true
    if last_cmd_did_fail; then
        on_error
    fi

    echo "  - Copying resources..."
    cp -r res/ release/
    if last_cmd_did_fail; then
        on_error
    fi

    echo "  - Copying binary..."
    cp target/release/ez-stream-oxidized release/EZStream
    if last_cmd_did_fail; then
        on_error
    fi

    echo "Done!"

    exit 0
}

if [[ "$1" == "--help" ]]; then
    help
    exit 0
elif [[ "$1" == "build" ]]; then
    build
elif [[ "$1" == "clean" ]]; then
    echo "Cleaning up..."
    cleanup
    echo "Done!"
    exit 0
else
    echo "Invalid command"
    usage
    exit 1
fi
