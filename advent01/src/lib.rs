#![allow(unused)]

use itertools::Itertools;

pub fn count_calories(content: &str) -> u64 {
    let items_per_elve = content.split("\n\n");
    let calories_per_elve =
        items_per_elve.map(|lines| lines.lines().map(|line| line.parse().unwrap_or(0)).sum());
    calories_per_elve.max().unwrap_or(0)
}

pub fn count_calories_top_three(content: &str) -> u64 {
    let items_per_elve = content.split("\n\n");
    let calories_per_elve =
        items_per_elve.map(|lines| lines.lines().map(|line| line.parse().unwrap_or(0)).sum::<u64>());

    calories_per_elve.sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_the_calories() {
        let result = count_calories(calories());

        assert_eq!(result, 24000);
    }

    #[test]
    fn it_counts_the_top_three_carriers_sum() {
        let result = count_calories_top_three(calories());

        assert_eq!(result, 45000);
    }

    fn calories() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }
}
