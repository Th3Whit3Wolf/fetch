alias m := Measure
alias t := Test
alias c := Compare

all := ' '

# Measure performace with hyperfine
Measure:
    @cargo build --release
    @hyperfine target/release/fetch

# Run test (add parameter to run specific test)
Test test=all:
    @cargo test

# Compare Output with neofetch
Compare:
    @neofetch --off --color_blocks off
    @target/release/fetch