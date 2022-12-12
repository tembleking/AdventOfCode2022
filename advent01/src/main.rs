use advent01::{count_calories, count_calories_top_three};

fn main() {
    let calories = count_calories(input());
    let calories_top_3 = count_calories_top_three(input());

    println!("Total calories: {}", calories);
    println!("Total calories of top 3: {}", calories_top_3);
}

fn input() -> &'static str {
    include_str!("input.txt")
}
