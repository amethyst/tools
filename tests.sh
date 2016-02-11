#!/bin/bash

# Perform basic integration testing on amethyst_cli.

function check_new() {
    echo "--- amethyst new"

    ln -s ./target/debug/amethyst .
    ./amethyst new mygame

    if [ -f mygame/Cargo.toml ] &&
       [ -d mygame/resources/entities/ ] &&
       [ -d mygame/resources/prefabs/ ] &&
       [ -f mygame/resources/config.yml ] &&
       [ -f mygame/resources/input.yml ] &&
       [ -f mygame/src/main.rs ] &&
       [ -d mygame/.git/ ] &&
       [ -f mygame/.gitignore ]; then
        echo "--- Passed!"
	echo
	return
    fi

    ls -l mygame
    exit 1
}

function check_build() {
    echo "--- amethyst build"

    cd mygame
    ../amethyst build

    if [ $? -eq 0 ]; then
        echo "--- Passed!"
	echo
        return
    fi

    ls -l
    exit 1
}

function check_run() {
    echo "--- amethyst run"

    ../amethyst run

    if [ $? -eq 0 ]; then
        echo "--- Passed!"
	echo
        return
    fi

    ls -l
    exit 1
}

function check_clean() {
    echo "--- amethyst clean"

    ../amethyst clean

    if [ ! -d target ]; then
        echo "--- Passed!"
	echo
	return
    fi

    ls -l
    exit 1
}

function check_bad_build() {
    echo "--- amethyst build"

    cd mygame
    rm -rf src
    ../amethyst build

    if [ $? -ne 0 ]; then
        echo "--- Passed!"
	echo
        return
    fi

    ls -l
    exit 1
}

check_new
check_build
check_run
check_clean
check_bad_build

echo
echo "All tests pass!"
cd ..
rm -r amethyst mygame
