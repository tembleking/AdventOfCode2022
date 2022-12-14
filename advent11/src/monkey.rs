use regex::Regex;
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
        let regex = Regex::new(
            r#"Monkey \d+:
  Starting items: ([\d,\s]+)
  Operation: new = (\w+) ([+\-*/]) (\w+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)"#,
        )
        .map_err(|e| format!("failed to create regex: {}", e))?;
        let captures = regex.captures(input).ok_or("input does not match regex")?;

        let starting_items_line = captures.get(1).ok_or("no starting items found")?.as_str();
        let starting_items = starting_items_from_str(starting_items_line)?;

        let left_operand = captures.get(2).ok_or("no left operand found")?.as_str();
        let operator = captures.get(3).ok_or("no operator found")?.as_str();
        let right_operand = captures.get(4).ok_or("no right operand found")?.as_str();
        let operation = operation_from_str(left_operand, operator, right_operand)?;

        let test_divisor = captures.get(5).ok_or("no test divisor found")?.as_str();
        let test = test_from_str(test_divisor)?;

        let monkey_to_throw_if_true = captures
            .get(6)
            .ok_or("no monkey to throw if true found")?
            .as_str()
            .trim()
            .parse::<usize>()
            .map_err(|_| "unable to parse monkey to throw to")?;

        let monkey_to_throw_if_false = captures
            .get(7)
            .ok_or("no monkey to throw if false found")?
            .as_str()
            .trim()
            .parse::<usize>()
            .map_err(|_| "unable to parse monkey to throw to")?;

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
        .map(Monkey::try_from)
        .collect::<Result<Vec<Monkey>, String>>()
}

fn starting_items_from_str(starting_items_line: &str) -> Result<Vec<i64>, String> {
    let starting_items = starting_items_line
        .trim()
        .split(',')
        .filter_map(|item| item.trim().parse::<i64>().ok())
        .collect::<Vec<_>>();

    Ok(starting_items)
}

fn operation_from_str(
    left_operand: &str,
    operator: &str,
    right_operand: &str,
) -> Result<impl Fn(i64) -> i64, String> {
    let left_operand = left_operand.to_string();
    let operator = operator.to_string();
    let right_operand = right_operand.to_string();

    let func = move |old: i64| -> i64 {
        let first_operand = match left_operand.as_str() {
            "old" => old,
            _ => left_operand.parse::<i64>().unwrap(),
        };

        let second_operand = match right_operand.as_str() {
            "old" => old,
            _ => right_operand.parse::<i64>().unwrap(),
        };

        match operator.as_str() {
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
    let num_to_divide = input
        .parse::<i64>()
        .map_err(|e| format!("failed to parse divisor: {} for input: {}", e, input))?;

    let func = move |num: i64| -> bool { num % num_to_divide == 0 };

    Ok(func)
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
        include_str!("example.txt")
    }
}
