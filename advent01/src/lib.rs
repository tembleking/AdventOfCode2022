#![allow(unused)]

pub fn count_calories(content: &str) -> u64 {
    let items_per_elve = content.split("\n\n");
    let calories_per_elve =
        items_per_elve.map(|lines| lines.lines().map(|line| line.parse().unwrap_or(0)).sum());
    calories_per_elve.max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_the_calories() {
        let result = count_calories(calories());

        assert_eq!(result, 24000);
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
