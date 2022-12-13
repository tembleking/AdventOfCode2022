use advent06::marker_position;

fn main() {
    let count = marker_position(input().chars());
    println!("count: {}", count.unwrap());
}

fn input() -> &'static str {
    include_str!("input.txt")
}