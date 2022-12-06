#![allow(dead_code)]

use std::collections::VecDeque;

struct Ship {
    stacks: Vec<VecDeque<char>>,
}

impl Ship {
    pub fn from_str(input: &str) -> Result<Ship, String> {
        let stacks = Self::stacks_from_str(input)?;
        Ok(Ship { stacks })
    }

    pub fn stacks(&self) -> &Vec<VecDeque<char>> {
        &self.stacks
    }

    pub fn crates_message_to_elves(&self) -> String {
        let mut message = String::new();
        for stack in &self.stacks {
            if let Some(&c) = stack.back() {
                message.push(c);
            }
        }
        message
    }

    fn number_of_stacks_from_str(input: &str) -> Result<usize, String> {
        let first_line_without_square_brackets = input
            .lines()
            .find(|line| !line.contains('['))
            .ok_or("No first line without square brackets")?;

        let last_stack_id = first_line_without_square_brackets
            .trim()
            .split(' ')
            .last()
            .ok_or("No last stack id")?;

        let last_stack_id_as_u32 = last_stack_id
            .parse::<usize>()
            .map_err(|_| format!("Last stack id '{}' is not a number", last_stack_id))?;

        Ok(last_stack_id_as_u32)
    }

    fn stacks_from_str(input: &str) -> Result<Vec<VecDeque<char>>, String> {
        let number_of_stacks = Self::number_of_stacks_from_str(input)?;
        let mut stacks = Vec::with_capacity(number_of_stacks);
        stacks.resize_with(number_of_stacks, VecDeque::new);

        let lines_with_crates = input.lines().filter(|line| line.contains('['));
        for line in lines_with_crates {
            for (i, stack) in stacks.iter_mut().enumerate() {
                let slice = (i * 4)..=(i * 4 + 2);
                let crate_str = line.get(slice).unwrap_or("  ");
                let maybe_crate_without_square_brackets =
                    crate_str.replace(['[', ']'], "").trim().chars().next();

                if let Some(crate_char) = maybe_crate_without_square_brackets {
                    if crate_char != ' ' {
                        stack.push_front(crate_char);
                    }
                }
            }
        }

        Ok(stacks)
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Move {
        count: usize,
        from: usize,
        to: usize,
    },
}

fn instructions_from_str(input: &str) -> Result<Vec<Instruction>, String> {
    let lines = input.lines();

    let lines_without_crates = lines
        .filter(|line| !line.trim().is_empty())
        .filter(|line| !line.contains(['[', ']']))
        .skip(1); // Skip first line, which are the IDs

    lines_without_crates
        .map(instruction_from_str)
        .collect::<Result<Vec<_>, _>>()
}

fn instruction_from_str(line: &str) -> Result<Instruction, String> {
    let mut words = line.split(' ');
    let instruction = words.next().ok_or("No instruction")?;

    match instruction {
        "move" => {
            let mut words = words.step_by(2);
            let count = words.next().ok_or("No count for move instruction")?;
            let count = count
                .parse::<usize>()
                .map_err(|_| format!("Count '{}' is not a number", count))?;
            let from = words.next().ok_or("No from for move instruction")?;
            let from = from
                .parse::<usize>()
                .map_err(|_| format!("From '{}' is not a number", from))?;
            let to = words.next().ok_or("No to for move instruction")?;
            let to = to
                .parse::<usize>()
                .map_err(|_| format!("To '{}' is not a number", to))?;
            Ok(Instruction::Move { count, from, to })
        }
        _ => Err(format!("Unknown instruction '{}'", instruction)),
    }
}

struct CargoCrane {
    ship: Ship,
    instructions: Vec<Instruction>,
}

impl CargoCrane {
    pub fn from_str(input: &str) -> Result<CargoCrane, String> {
        let ship = Ship::from_str(input)?;
        let instructions = instructions_from_str(input)?;

        Ok(CargoCrane { ship, instructions })
    }

    pub fn ship(&self) -> &Ship {
        &self.ship
    }

    pub fn execute_instructions(&mut self) -> Result<(), String> {
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Move { count, from, to } => {
                    for _ in 0..*count {
                        let crate_char = {
                            let from_stack =
                                self.ship.stacks.get_mut(*from - 1).ok_or("No from stack")?;
                            from_stack.pop_back().ok_or("No crate to move")?
                        };

                        let to_stack = self.ship.stacks.get_mut(*to - 1).ok_or("No to stack")?;
                        to_stack.push_back(crate_char);
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_the_stacks_from_the_input() {
        let ship = Ship::from_str(input()).unwrap();

        assert_eq!(ship.stacks().len(), 3);
        assert_eq!(ship.stacks()[0], VecDeque::from(['Z', 'N']));
        assert_eq!(ship.stacks()[1], VecDeque::from(['M', 'C', 'D']));
        assert_eq!(ship.stacks()[2], VecDeque::from(['P']));
    }

    #[test]
    fn it_loads_the_instruction_from_a_string() {
        let input = "move 1 from 2 to 1";
        let instruction = instruction_from_str(input).unwrap();

        assert_eq!(
            instruction,
            Instruction::Move {
                count: 1,
                from: 2,
                to: 1
            }
        );
    }

    #[test]
    fn it_loads_the_instructions_from_the_input() {
        let instructions = instructions_from_str(input()).unwrap();

        assert_eq!(instructions.len(), 4);
        assert_eq!(
            instructions[0],
            Instruction::Move {
                count: 1,
                from: 2,
                to: 1
            }
        );
        assert_eq!(
            instructions[1],
            Instruction::Move {
                count: 3,
                from: 1,
                to: 3
            }
        );
        assert_eq!(
            instructions[2],
            Instruction::Move {
                count: 2,
                from: 2,
                to: 1
            }
        );
        assert_eq!(
            instructions[3],
            Instruction::Move {
                count: 1,
                from: 1,
                to: 2
            }
        );
    }

    #[test]
    fn it_executes_the_instructions_on_the_ship() {
        let mut cargo_crane = CargoCrane::from_str(input()).unwrap();

        cargo_crane.execute_instructions().unwrap();
        let ship = cargo_crane.ship();

        assert_eq!(ship.stacks().len(), 3);
        assert_eq!(ship.stacks()[0], VecDeque::from(['C']));
        assert_eq!(ship.stacks()[1], VecDeque::from(['M']));
        assert_eq!(ship.stacks()[2], VecDeque::from(['P', 'D', 'N', 'Z']));
    }

    #[test]
    fn it_gets_the_top_elements_from_the_ship() {
        let mut cargo_crane = CargoCrane::from_str(input()).unwrap();
        let ship_before = cargo_crane.ship();
        assert_eq!(ship_before.crates_message_to_elves(), "NDP");

        cargo_crane.execute_instructions().unwrap();
        let ship_after = cargo_crane.ship();
        assert_eq!(ship_after.crates_message_to_elves(), "CMZ");
    }

    fn input() -> &'static str {
        "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }
}
