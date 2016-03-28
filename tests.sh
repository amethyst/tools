#!/bin/bash

# Perform basic integration testing on amethyst_cli.

function check_new() {
    echo "--- amethyst new"

    ln -s ./target/debug/amethyst .
    ./amethyst new mygame

    if [ $? -eq 0 ] &&
       [ -f mygame/Cargo.toml ] &&
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
	if [ -d "target/debug" ]; then
            echo "--- Passed!"
	    echo
	fi
    else
	ls -l
	exit 1
    fi

    echo "--- amethyst build --release"
    ../amethyst build --release

    # TODO: check if build actually was ran in release mode
    if [ $? -eq 0 ]; then
	if [ -d "target/release" ] ; then
	   echo "-- Passed!"
	   echo
	   return
	fi
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
    else
	ls -la
	exit 1
    fi

    echo "--- amethyst run --release"

    ../amethyst run --release

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

function check_deploy() {
    echo "--- amethyst deploy"

    ../amethyst deploy --clean

    #check that deploy dir, resources.zip, binary exists
    if [ $? -eq 0 ] &&
       [ -d deploy ] &&
       [ -f deploy/mygame ] &&
       [ -f deploy/resources.zip ]; then
        echo "--- Passed!"
        echo "--- Cleaning compiled and deployed files"
        ../amethyst clean
        rm -rf mygame/deployed
  	    echo
  	    return
    fi

    ls -l mygame/deployed
    exit 1
}

function check_corrupt_build() {
    echo "--- amethyst build (corrupt)"

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

function clean_up() {
    rm -r amethyst mygame
}

clean_up
cargo build

check_new
check_build
check_run
check_clean
check_deploy
check_corrupt_build

echo
echo "All tests pass!"
cd ..
clean_up
