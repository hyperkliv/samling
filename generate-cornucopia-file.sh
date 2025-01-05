#!/usr/bin/env bash
cornucopia \
    --queries-path=./queries \
    schema \
    ./migrations/*.sql
cargo fmt -- ./src/cornucopia.rs
