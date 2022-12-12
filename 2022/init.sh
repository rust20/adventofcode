#!/bin/env zsh

local name=$1

echo initializing $name
mkdir $1
cd $1
touch input.txt input_test.txt problem.md
cargo init --vcs none
cp ../template.rs src/main.rs
