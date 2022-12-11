use std::collections::VecDeque;

pub struct Monkey {
    items: VecDeque<i64>,
    worry_fn: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> bool>,
    monkey_to_throw_if_true: usize,
    monkey_to_throw_if_false: usize,
    evaluations: usize,
}

impl TryFrom<&str> for Monkey {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut lines_for_a_monkey = input.trim().lines().skip(1);
        let starting_items_line = lines_for_a_monkey.next().ok_or("No starting items")?;
        let operation_line = lines_for_a_monkey.next().ok_or("No operation")?;
        let test_line = lines_for_a_monkey.next().ok_or("No test")?;
        let monkey_to_throw_if_true_line = lines_for_a_monkey
            .next()
            .ok_or("No monkey to throw if true")?;
        let monkey_to_throw_if_false_line = lines_for_a_monkey
            .next()
            .ok_or("No monkey to throw if false")?;

        let starting_items = starting_items_from_str(starting_items_line)?;
        let operation = operation_from_str(operation_line)?;
        let test = test_from_str(test_line)?;
        let monkey_to_throw_if_true = monkey_to_throw_from_str(monkey_to_throw_if_true_line)?;
        let monkey_to_throw_if_false = monkey_to_throw_from_str(monkey_to_throw_if_false_line)?;

        Ok(Self {
            items: VecDeque::from(starting_items),
            worry_fn: Box::new(operation),
            test: Box::new(test),
            monkey_to_throw_if_true,
            monkey_to_throw_if_false,
            evaluations: 0,
        })
    }
}

impl Monkey {
    pub fn items(&self) -> &VecDeque<i64> {
        &self.items
    }

    fn worry(&self, item: i64) -> i64 {
        (self.worry_fn)(item)
    }

    pub fn does_test_pass(&self, item: i64) -> bool {
        (self.test)(item)
    }

    pub fn monkey_to_throw_if_passes(&self) -> usize {
        self.monkey_to_throw_if_true
    }

    pub fn monkey_to_throw_if_fails(&self) -> usize {
        self.monkey_to_throw_if_false
    }

    pub fn evaluate_object(&mut self) -> i64 {
        self.evaluations += 1;
        let item = self.items.pop_front().unwrap();
        let item_with_more_worry = self.worry(item);
        let item_is_not_broken_so_less_worry = (item_with_more_worry as f64 / 3.0);
        item_is_not_broken_so_less_worry.floor() as i64
    }

    pub(crate) fn evaluations(&self) -> usize {
        self.evaluations
    }

    pub fn receive_item(&mut self, item: i64) {
        self.items.push_back(item);
    }
}

fn parse_monkeys_from_str(input: &str) -> Result<Vec<Monkey>, String> {
    input
        .trim()
        .split("\n\n")
        .map(TryFrom::try_from)
        .collect::<Result<Vec<Monkey>, String>>()
}

fn starting_items_from_str(starting_items_line: &str) -> Result<Vec<i64>, String> {
    let starting_items = starting_items_line
        .trim()
        .split(':')
        .nth(1)
        .map(|starting_items_str| {
            starting_items_str
                .trim()
                .split(',')
                .filter_map(|item| item.trim().parse::<i64>().ok())
        })
        .ok_or("No starting items")?
        .collect::<Vec<_>>();

    Ok(starting_items)
}

fn operation_from_str(input: &str) -> Result<impl Fn(i64) -> i64, String> {
    let operation_words = input
        .trim()
        .split('=')
        .nth(1)
        .ok_or("No operation")?
        .trim()
        .to_string();

    let func = move |old: i64| -> i64 {
        let mut split = operation_words.split(' ');
        let first_operand = split.next().unwrap();
        let first_operand = match first_operand {
            "old" => old,
            _ => first_operand.parse::<i64>().unwrap(),
        };

        let operator = split.next().unwrap();

        let second_operand = split.next().unwrap();
        let second_operand = match second_operand {
            "old" => old,
            _ => second_operand.parse::<i64>().unwrap(),
        };

        match operator {
            "+" => first_operand + second_operand,
            "-" => first_operand - second_operand,
            "*" => first_operand * second_operand,
            "/" => first_operand.checked_div(second_operand).unwrap(),
            _ => 0,
        }
    };

    Ok(func)
}

fn test_from_str(input: &str) -> Result<impl Fn(i64) -> bool, String> {
    let test_words = input.trim().split(':').nth(1).ok_or("No test line")?;

    let num_to_divide_str = test_words
        .split("by")
        .nth(1)
        .ok_or("No number to divide and test from")?;

    let num_to_divide = num_to_divide_str.trim().parse::<i64>().map_err(|e| {
        format!(
            "unable to parse number to divide and test from: {}",
            num_to_divide_str
        )
    })?;

    let func = move |num: i64| -> bool { num % num_to_divide == 0 };

    Ok(func)
}

fn monkey_to_throw_from_str(line: &str) -> Result<usize, String> {
    let monkey_str = line
        .trim()
        .split("throw to monkey ")
        .nth(1)
        .ok_or("no monkey to throw to")?;

    monkey_str
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("unable to parse monkey to throw to: {}", monkey_str))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_the_input_and_creates_the_monkeys() {
        let monkeys = parse_monkeys_from_str(input()).unwrap();

        assert_eq!(monkeys.len(), 4);
        assert_eq!(monkeys[0].items(), &Vec::from([79, 98]));
        assert_eq!(monkeys[0].worry(79), 79 * 19);
        assert!(monkeys[0].does_test_pass(23));
        assert!(!monkeys[0].does_test_pass(23 * 2 + 1));
        assert_eq!(monkeys[0].monkey_to_throw_if_passes(), 2);
        assert_eq!(monkeys[0].monkey_to_throw_if_fails(), 3);

        assert_eq!(monkeys[1].items(), &Vec::from([54, 65, 75, 74]));
        assert_eq!(monkeys[1].worry(54), 54 + 6);
        assert!(monkeys[1].does_test_pass(19));
        assert!(!monkeys[1].does_test_pass(19 * 2 + 1));
        assert_eq!(monkeys[1].monkey_to_throw_if_passes(), 2);
        assert_eq!(monkeys[1].monkey_to_throw_if_fails(), 0);

        assert_eq!(monkeys[2].items(), &Vec::from([79, 60, 97]));
        assert_eq!(monkeys[2].worry(79), 79 * 79);
        assert!(monkeys[2].does_test_pass(13));
        assert!(!monkeys[2].does_test_pass(13 * 2 + 1));
        assert_eq!(monkeys[2].monkey_to_throw_if_passes(), 1);
        assert_eq!(monkeys[2].monkey_to_throw_if_fails(), 3);

        assert_eq!(monkeys[3].items(), &Vec::from([74]));
        assert_eq!(monkeys[3].worry(74), 74 + 3);
        assert!(monkeys[3].does_test_pass(17));
        assert!(!monkeys[3].does_test_pass(17 * 2 + 1));
        assert_eq!(monkeys[3].monkey_to_throw_if_passes(), 0);
        assert_eq!(monkeys[3].monkey_to_throw_if_fails(), 1);
    }

    #[test]
    fn monkey_0_evaluates_an_object_and_sends_it_to_monkey_3() {
        let mut monkeys = parse_monkeys_from_str(input()).unwrap();

        let monkey_0 = &mut monkeys[0];
        assert_eq!(monkey_0.evaluate_object(), 500);
        assert_eq!(monkey_0.evaluations(), 1);

        let test_pass = monkey_0.does_test_pass(500);
        assert!(!test_pass);

        let monkey_to_throw = monkey_0.monkey_to_throw_if_fails();
        assert_eq!(monkey_to_throw, 3);

        let monkey_3 = &mut monkeys[3];
        monkey_3.receive_item(500);
    }

    fn input() -> &'static str {
        include_str!("input.txt")
    }
}
