#![allow(unused)]

use std::collections::HashSet;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    width: usize,
    height: usize,
    start_position: (usize, usize),
    head_position: (usize, usize),
    tail_position: (usize, usize),
    tail_movements: HashSet<(usize, usize)>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut line = String::with_capacity(self.width);

        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == self.head_position {
                    line.push('H');
                } else if (x, y) == self.tail_position {
                    line.push('T');
                } else {
                    line.push('.');
                }
            }
            writeln!(f, "{}", line)?;
            line.clear();
        }

        Ok(())
    }
}

impl Grid {
    pub fn from_str(input: &str) -> Result<Grid, String> {
        let mut width = 0;
        let mut height = 0;
        let mut head_position = None;
        let mut tail_position = None;

        for (y_pos, line) in input.lines().enumerate() {
            width = line.trim().len();
            for (x_pos, char) in line.chars().enumerate() {
                if char == 'H' {
                    head_position = Some((x_pos, y_pos));
                } else if char == 'T' {
                    tail_position = Some((x_pos, y_pos));
                }
            }
            height += 1;
        }

        let head_position = head_position.ok_or("No head position found")?;
        let tail_position = tail_position.unwrap_or(head_position);
        Ok(Grid {
            width,
            height,
            start_position: head_position,
            head_position,
            tail_position,
            tail_movements: HashSet::from([tail_position]),
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn head_position(&self) -> (usize, usize) {
        self.head_position
    }

    pub fn tail_position(&self) -> (usize, usize) {
        self.tail_position
    }

    pub fn move_head(&mut self, direction: &Direction) -> Result<(), String> {
        self.head_position = self.get_new_head_position(direction)?;
        self.tail_position = self.get_new_tail_position();
        self.tail_movements.insert(self.tail_position);

        Ok(())
    }

    fn get_new_head_position(&self, direction: &Direction) -> Result<(usize, usize), String> {
        let (x, y) = self.head_position;
        let (new_x, new_y) = match direction {
            Direction::Up => (
                x,
                y.checked_sub(1)
                    .ok_or("Cannot move up, would be out of bounds")?,
            ),
            Direction::Down => (x, y + 1),
            Direction::Left => (
                x.checked_sub(1)
                    .ok_or("Cannot move left, would be out of bounds")?,
                y,
            ),
            Direction::Right => (x + 1, y),
        };

        if new_x >= self.width || new_y >= self.height {
            return Err("cannot move head there, it would be out of bounds".to_string());
        }

        Ok((new_x, new_y))
    }

    fn get_new_tail_position(&self) -> (usize, usize) {
        let (x_head, y_head) = self.head_position;
        let (x_tail, y_tail) = self.tail_position;

        let x_diff = x_head as i32 - x_tail as i32;
        let y_diff = y_head as i32 - y_tail as i32;

        let new_pos = if x_diff.abs() >= 2 {
            (x_tail as i32 + x_diff / 2, y_tail as i32 + y_diff)
        } else if y_diff.abs() >= 2 {
            (x_tail as i32 + x_diff, y_tail as i32 + y_diff / 2)
        } else {
            (x_tail as i32, y_tail as i32)
        };

        (new_pos.0 as usize, new_pos.1 as usize)
    }

    pub fn tail_movements_in_grid(&self) -> String {
        let mut result = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.start_position == (x, y) {
                    result.push('s');
                } else if self.tail_movements.contains(&(x, y)) {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }

        result.trim().to_string()
    }

    pub fn unique_tail_movements(&self) -> usize {
        self.tail_movements.len()
    }
}

fn instructions_from_str(input: &str) -> Result<Vec<Direction>, String> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let mut split = line.trim().split(' ');
        let direction_char = split
            .next()
            .ok_or("No direction found")?
            .chars()
            .next()
            .ok_or("Char could not be extracted from direction")?;
        let direction_count = split
            .next()
            .map(|s| s.parse::<usize>())
            .ok_or("No count found")?
            .map_err(|_| "Could not parse count")?;

        let direction = match direction_char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return Err("Invalid direction".to_string()),
        };

        for _ in 0..direction_count {
            instructions.push(direction);
        }
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_the_grid_from_str() {
        let grid = Grid::from_str(grid_input()).unwrap();

        assert_eq!(grid.width(), 6);
        assert_eq!(grid.height(), 5);
        assert_eq!(grid.head_position(), (0, 4));
        assert_eq!(grid.tail_position(), (0, 4));
        assert_eq!(grid.to_string(), grid_input());
    }

    #[test]
    fn it_moves_the_head_to_the_right_without_moving_the_tail_because_they_are_near() {
        let input = "..H..";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Right).unwrap();

        let expected = "..TH.";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_to_the_left_without_moving_the_tail_because_they_are_near() {
        let input = "..H..";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Left).unwrap();

        let expected = ".HT..";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_to_the_right_without_moving_the_tail_because_they_are_near_but_in_other_lines_1(
    ) {
        let input = "\
..H..
...T.";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Right).unwrap();

        let expected = "\
...H.
...T.";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_to_the_right_without_moving_the_tail_because_they_are_near_but_in_other_lines_2(
    ) {
        let input = "\
..H..
..T..";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Right).unwrap();

        let expected = "\
...H.
..T..";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_to_the_bottom_without_moving_the_tail_because_they_are_near() {
        let input = "\
H.
.T
..";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Down).unwrap();

        let expected = "\
..
HT
..";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_right_and_moves_the_tail_because_they_are_far() {
        let input = ".TH...";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Right).unwrap();

        let expected = "..TH..";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_left_and_moves_the_tail_because_they_are_far() {
        let input = ".HT...";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Left).unwrap();

        let expected = "HT....";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_down_and_moves_the_tail_because_they_are_far() {
        let input = "\
.
T
H
.
.";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Down).unwrap();

        let expected = "\
.
.
T
H
.";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_up_and_moves_the_tail_because_they_are_far() {
        let input = "\
.
H
T
.
.";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Up).unwrap();

        let expected = "\
H
T
.
.
.";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_right_and_moves_the_tail_diagonally_up() {
        let input = "\
..H..
.T...";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Right).unwrap();

        let expected = "\
..TH.
.....";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn it_moves_the_head_right_and_moves_the_tail_diagonally_down() {
        let input = "\
.T...
..H..";
        let mut grid = Grid::from_str(input).unwrap();
        grid.move_head(&Direction::Right).unwrap();

        let expected = "\
.....
..TH.";
        assert_eq!(grid.to_string().trim(), expected);
    }

    #[test]
    fn creates_an_instruction_execution_list_from_str() {
        let instructions = instructions_from_str(instructions_input()).unwrap();

        assert_eq!(instructions.len(), 4 + 4 + 3 + 1 + 4 + 1 + 5 + 2);
        assert_eq!(
            instructions,
            Vec::from([
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Up,
                Direction::Up,
                Direction::Up,
                Direction::Up,
                Direction::Left,
                Direction::Left,
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left,
                Direction::Left,
                Direction::Left,
                Direction::Left,
                Direction::Right,
                Direction::Right,
            ])
        );
    }

    #[test]
    fn it_moves_the_head_around_and_displays_the_movements_of_the_tail() {
        let mut grid = Grid::from_str(grid_input()).unwrap();
        let instructions = instructions_from_str(instructions_input()).unwrap();

        for instruction in instructions {
            grid.move_head(&instruction).unwrap();
        }

        let expected = "\
..##..
...##.
.####.
....#.
s###..";
        assert_eq!(grid.tail_movements_in_grid(), expected);
    }

    #[test]
    fn it_returns_the_number_of_tail_movements() {
        let mut grid = Grid::from_str(grid_input()).unwrap();
        let instructions = instructions_from_str(instructions_input()).unwrap();

        for instruction in instructions {
            grid.move_head(&instruction).unwrap();
        }

        assert_eq!(grid.unique_tail_movements(), 13);
    }

    fn grid_input() -> &'static str {
        "\
......
......
......
......
H.....
"
    }

    fn instructions_input() -> &'static str {
        "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
    }
}
