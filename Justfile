build:
    cargo build

run:
    cargo run

test:
    pytest -v --color=yes --basetemp=./tmp_tests
