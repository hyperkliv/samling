#!/usr/bin/env bash
clorinde \
    --queries-path=./samling/queries \
    --destination=samling-clorinde \
    schema \
    ./samling/migrations/*.sql
cargo fmt -p samling-clorinde
