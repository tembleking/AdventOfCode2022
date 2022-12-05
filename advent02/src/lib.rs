pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// A -> Rock
/// B -> Paper
/// C -> Scissors
/// X -> Rock
/// Y -> Paper
/// Z -> Scissors
#[derive(Debug, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    pub fn from_char(c: char) -> Option<HandShape> {
        match c {
            'A' | 'X' => Some(HandShape::Rock),
            'B' | 'Y' => Some(HandShape::Paper),
            'C' | 'Z' => Some(HandShape::Scissors),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    enemy: HandShape,
    myself: HandShape,
}

impl Round {
    pub fn new(left: HandShape, right: HandShape) -> Round {
        Round { enemy: left, myself: right }
    }

    pub fn from_line(line: &str) -> Option<Round> {
        let mut chars = line.chars();

        let left = chars.next().and_then(HandShape::from_char)?;
        chars.next(); // skip space
        let right = chars.next().and_then(HandShape::from_char)?;

        Some(Round { enemy: left, myself: right })
    }

    pub fn score(&self) -> u32 {
        self.shape_score() + self.outcome_score()
    }

    fn won(&self) -> bool {
        matches!(
            (&self.enemy, &self.myself),
            (HandShape::Rock, HandShape::Paper) | (HandShape::Paper, HandShape::Scissors) | (HandShape::Scissors, HandShape::Rock)
        )
    }

    fn draw(&self) -> bool {
        self.enemy == self.myself
    }

    fn lost(&self) -> bool {
        !self.won() && !self.draw()
    }

    fn shape_score(&self) -> u32 {
        match &self.myself {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }

    fn outcome_score(&self) -> u32 {
        if self.lost() {
            0
        } else if self.draw() {
            3
        } else {
            6
        }
    }
}

struct Strategy {
    rounds: Vec<Round>,
}

impl Strategy {
    pub fn new(input: &str) -> Self {
        let rounds = input
            .lines()
            .filter_map(Round::from_line)
            .collect::<Vec<Round>>();

        Strategy { rounds }
    }

    pub fn rounds(&self) -> usize {
        self.rounds.len()
    }

    pub fn round(&self, round: usize) -> Option<&Round> {
        self.rounds.get(round)
    }

    pub fn score(&self) -> u32 {
        self.rounds.iter().map(Round::score).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_retrieves_the_strategy() {
        let strategy = Strategy::new(input());

        assert_eq!(strategy.rounds(), 3);
        assert_eq!(
            strategy.round(0),
            Some(&Round::new(HandShape::Rock, HandShape::Paper))
        );
        assert_eq!(strategy.score(), 15);
    }

    #[test]
    fn it_returns_the_shapes_from_the_letters() {
        let shapes = [
            HandShape::from_char('A'),
            HandShape::from_char('B'),
            HandShape::from_char('C'),
            HandShape::from_char('X'),
            HandShape::from_char('Y'),
            HandShape::from_char('Z'),
            HandShape::from_char('t'),
        ];

        assert_eq!(
            shapes,
            [
                Some(HandShape::Rock),
                Some(HandShape::Paper),
                Some(HandShape::Scissors),
                Some(HandShape::Rock),
                Some(HandShape::Paper),
                Some(HandShape::Scissors),
                None,
            ]
        );
    }

    #[test]
    fn it_returns_the_round_from_a_line() {
        let line = "A X";
        let round = Round::from_line(line);

        assert_eq!(round, Some(Round::new(HandShape::Rock, HandShape::Rock)));
    }

    #[test]
    fn it_returns_the_score_from_the_lines() {
        let lines = [
            "A X", "A Y", "A Z", "B X", "B Y", "B Z", "C X", "C Y", "C Z",
        ];

        let rounds = lines
            .into_iter()
            .filter_map(Round::from_line)
            .map(|round| round.score())
            .collect::<Vec<u32>>();

        assert_eq!(rounds, vec![4, 8, 3, 1, 5, 9, 7, 2, 6]);
    }

    fn input() -> &'static str {
        "A Y
B X
C Z"
    }
}
