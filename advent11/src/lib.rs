#![allow(unused)]

mod monkey;

use itertools::Itertools;
use monkey::Monkey;

struct RoundExecutor {
    monkeys: Vec<Monkey>,
    round: usize,
}

impl TryFrom<&str> for RoundExecutor {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let monkeys = input
            .trim()
            .split("\n\n")
            .map(TryFrom::try_from)
            .collect::<Result<Vec<Monkey>, String>>()?;

        Ok(Self { monkeys, round: 0 })
    }
}

impl RoundExecutor {
    fn execute_round(&mut self) -> Result<(), String> {
        for i in 0..self.monkeys.len() {
            let monkey_items = self
                .monkeys
                .get(i)
                .ok_or("no monkey with this index")?
                .items()
                .len();

            for _ in 0..monkey_items {
                let monkey = self.monkeys.get_mut(i).ok_or("no monkey with this index")?;
                let item = monkey.evaluate_object();
                let monkey_to_throw = if monkey.does_test_pass(item) {
                    monkey.monkey_to_throw_if_passes()
                } else {
                    monkey.monkey_to_throw_if_fails()
                };

                self.monkeys
                    .get_mut(monkey_to_throw)
                    .ok_or("no money with this index")?
                    .receive_item(item);
            }
        }

        Ok(())
    }

    fn monkeys(&self) -> &Vec<Monkey> {
        &self.monkeys
    }

    fn monkey_business(&self) -> i64 {
        self.monkeys
            .iter()
            .map(|monkey| monkey.evaluations() as i64)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_the_monkeys_and_executes_a_round() {
        let mut round_executor = RoundExecutor::try_from(input()).unwrap();

        round_executor.execute_round().unwrap();

        assert_eq!(
            round_executor.monkeys()[0].items(),
            &Vec::from([20, 23, 27, 26])
        );
        assert_eq!(
            round_executor.monkeys()[1].items(),
            &Vec::from([2080, 25, 167, 207, 401, 1046])
        );
        assert_eq!(round_executor.monkeys()[2].items(), &Vec::from([]));
        assert_eq!(round_executor.monkeys()[3].items(), &Vec::from([]));
    }

    #[test]
    fn it_executes_20_rounds() {
        let mut round_executor = RoundExecutor::try_from(input()).unwrap();

        for _ in 0..20 {
            round_executor.execute_round().unwrap();
        }

        assert_eq!(
            round_executor.monkeys()[0].items(),
            &Vec::from([10, 12, 14, 26, 34])
        );
        assert_eq!(
            round_executor.monkeys()[1].items(),
            &Vec::from([245, 93, 53, 199, 115])
        );
        assert_eq!(round_executor.monkeys()[2].items(), &Vec::from([]));
        assert_eq!(round_executor.monkeys()[3].items(), &Vec::from([]));
    }

    #[test]
    fn it_retrieves_the_number_of_evaluations_per_monkey() {
        let mut round_executor = RoundExecutor::try_from(input()).unwrap();

        for _ in 0..20 {
            round_executor.execute_round().unwrap();
        }

        assert_eq!(round_executor.monkeys()[0].evaluations(), 101);
        assert_eq!(round_executor.monkeys()[1].evaluations(), 95);
        assert_eq!(round_executor.monkeys()[2].evaluations(), 7);
        assert_eq!(round_executor.monkeys()[3].evaluations(), 105);
    }

    #[test]
    fn it_calculates_the_monkey_business() {
        let mut round_executor = RoundExecutor::try_from(input()).unwrap();

        for _ in 0..20 {
            round_executor.execute_round().unwrap();
        }

        assert_eq!(round_executor.monkey_business(), 10605);
    }

    fn input() -> &'static str {
        include_str!("example.txt")
    }
}
