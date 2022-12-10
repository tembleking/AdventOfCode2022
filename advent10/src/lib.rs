#![allow(unused)]

use std::collections::VecDeque;

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Noop,
    Addx(i32),
}

struct Computer {
    commands: VecDeque<Command>,
    clock: usize,
    register: i32,
}

impl Computer {
    fn new(commands: Vec<Command>) -> Self {
        let new_commands = Self::extend_commands(commands);
        Self {
            clock: 1,
            commands: VecDeque::from(new_commands),
            register: 1,
        }
    }

    fn extend_commands(commands: Vec<Command>) -> Vec<Command> {
        let mut new_commands = Vec::with_capacity(commands.len() * 2);

        for command in commands {
            match command {
                Command::Noop => {
                    new_commands.push(Command::Noop);
                }
                Command::Addx(count) => {
                    new_commands.push(Command::Noop);
                    new_commands.push(Command::Addx(count));
                }
            }
        }
        new_commands
    }

    fn run(&mut self, clock_count: usize) -> Result<i32, String> {
        for _ in self.clock..clock_count {
            let command = self.commands.pop_front().ok_or("No more commands to run")?;
            match command {
                Command::Noop => {}
                Command::Addx(count) => {
                    self.register += count;
                }
            }
            self.clock += 1;
        }

        Ok(self.register)
    }
}

fn parse_command(input: &str) -> Option<Command> {
    let mut words = input.trim().split(' ');
    let command = words.next()?;
    match command {
        "addx" => {
            let count = words.next()?.parse::<i32>().ok()?;
            Some(Command::Addx(count))
        }
        "noop" => Some(Command::Noop),
        _ => None,
    }
}

fn parse_program(input: &str) -> Result<Vec<Command>, String> {
    input
        .trim()
        .lines()
        .map(parse_command)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "Invalid input".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_program() {
        let commands = parse_program(input()).unwrap();

        assert_eq!(commands.len(), 146);
        assert_eq!(commands[0], Command::Addx(15));
        assert_eq!(commands[1], Command::Addx(-11));
        assert_eq!(commands[2], Command::Addx(6));
        assert_eq!(commands[9], Command::Noop);
    }

    #[test]
    fn it_returns_the_signal_strength_of_the_20th_cycle() {
        let commands = parse_program(input()).unwrap();
        let mut computer = Computer::new(commands);

        let register = computer.run(20).unwrap();
        assert_eq!(register, 21);
    }

    #[test]
    fn it_returns_the_signal_strength_of_the_60th_cycle() {
        let commands = parse_program(input()).unwrap();
        let mut computer = Computer::new(commands);

        let register = computer.run(60).unwrap();
        assert_eq!(register, 19);
    }

    #[test]
    fn it_returns_the_signal_strength_of_the_220th_cycle() {
        let commands = parse_program(input()).unwrap();
        let mut computer = Computer::new(commands);

        let register = computer.run(220).unwrap();
        assert_eq!(register, 18);
    }

    #[test]
    fn it_executes_all_steps() {
        let commands = parse_program(input()).unwrap();
        let mut computer = Computer::new(commands);

        let cycles = (20..=220).step_by(40);
        let signals = cycles
            .map(|cycle| computer.run(cycle).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(signals, vec![21, 19, 18, 21, 16, 18]);
    }

    #[test]
    fn it_calculates_all_the_signal_value() {
        let commands = parse_program(input()).unwrap();
        let mut computer = Computer::new(commands);

        let cycles = (20..=220).step_by(40);
        let signals = cycles.map(|cycle| {
            let register = computer.run(cycle).unwrap();
            register * cycle as i32
        });
        let signal_sum = signals.sum::<i32>();

        assert_eq!(signal_sum, 13140);
    }

    fn input() -> &'static str {
        "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
    }
}
