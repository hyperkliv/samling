#!/usr/bin/env bash
cornucopi \
    --queries-path=./queries \
    --destination=src/cornucopia.rs \
    schema \
    ./migrations/*.sql
cargo fmt -- ./src/cornucopia.rs
