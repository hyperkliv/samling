#!/usr/bin/env bash
clorinde \
    --queries-path=./queries \
    --destination=samling-clorinde \
    schema \
    ./migrations/*.sql
cargo fmt -p samling-clorinde
