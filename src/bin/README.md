Each `dayX.rs` file in this directory should use the following template.

```rust
use adventofcode_2022::runner;

fn parse_input(input: &str) -> T {}

fn part1(input: &str) {}

fn part2(input: &str) {}

fn main() {
    runner(part1);
    runner(part2);
}
```