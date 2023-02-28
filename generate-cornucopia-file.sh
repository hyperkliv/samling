#!/usr/bin/env bash
cornucopia \
    --serialize \
    --queries-path=./queries \
    schema \
    ./migrations/*
