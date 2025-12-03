
_default:
    just --list

run:
    cargo run

test:
    cargo test

new day:
    #!/usr/bin/env sh
    day=$(printf "day%02d" {{day}})
    touch inputs/${day}.txt
    touch examples/${day}.txt
    cp src/template.rs src/${day}.rs

