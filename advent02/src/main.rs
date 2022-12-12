use advent02::Strategy;

fn main() {
    let strategy = Strategy::new(input());
    let strategy_with_end_result = Strategy::new_with_end_result(input());
    let score = strategy.score();

    println!("Score: {}", score);
    println!("Score with end result: {}", strategy_with_end_result.score());
}

fn input() -> &'static str {
    include_str!("input.txt")
}