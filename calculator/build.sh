#!/usr/bin/env bash

grep -r TODO src
grep -r FIXME src
cargo build


