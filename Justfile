default: test

build:
    cargo build

run:
    cargo run

test: build
    pytest -v --color=yes --basetemp=./tmp_tests

cleantest:
    rm -rf tmp_tests
