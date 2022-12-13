#![allow(unused)]

use std::num::ParseIntError;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from_str(input: &str) -> Result<Range, String> {
        let mut parts = input.split('-');

        let start = parts
            .next()
            .ok_or("No start defined")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;
        let end = parts
            .next()
            .ok_or("No end defined")?
            .parse()
            .map_err(|e: ParseIntError| e.to_string())?;

        if parts.next().is_some() {
            return Err("Too many parts".to_string());
        }

        Ok(Range { start, end })
    }

    fn start(&self) -> u32 {
        self.start
    }

    fn end(&self) -> u32 {
        self.end
    }

    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

struct RangePair {
    left: Range,
    right: Range,
}

impl RangePair {
    fn from_str(input: &str) -> Result<RangePair, String> {
        let mut parts = input.split(',');

        let left = Range::from_str(parts.next().ok_or("No left defined")?)?;
        let right = Range::from_str(parts.next().ok_or("No right defined")?)?;

        if parts.next().is_some() {
            return Err("Too many parts".to_string());
        }

        Ok(RangePair { left, right })
    }

    fn is_fully_contained(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }

    fn overlaps(&self) -> bool {
        self.left.overlaps(&self.right)
    }
}

pub struct SectionAssignments {
    assignments: Vec<RangePair>,
}

impl TryFrom<&str> for SectionAssignments {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let assignments = input
            .lines()
            .map(RangePair::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(SectionAssignments { assignments })
    }
}

impl SectionAssignments {
    pub fn fully_contained_count(&self) -> usize {
        self.assignments
            .iter()
            .filter(|a| a.is_fully_contained())
            .count()
    }

    pub fn overlap_count(&self) -> usize {
        self.assignments
            .iter()
            .filter(|a| a.overlaps())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn it_retrieves_the_range_for_an_elf_to_clean() {
        let input = "2-4";

        let range = Range::from_str(input).unwrap();

        assert_eq!(range.start(), 2);
        assert_eq!(range.end(), 4);
    }

    #[test]
    fn checks_that_one_range_is_contained_in_other() {
        let range = Range::from_str("2-4").unwrap();
        let other = Range::from_str("3-4").unwrap();

        assert!(range.contains(&other));
        assert!(!other.contains(&range));
    }

    #[test]
    fn checks_that_one_range_is_not_contained_in_other() {
        let range = Range::from_str("2-4").unwrap();
        let other = Range::from_str("1-2").unwrap();

        assert!(!range.contains(&other));
        assert!(!other.contains(&range));
    }

    #[test]
    fn it_checks_that_a_range_pair_is_contained() {
        let range_pair = RangePair::from_str("2-8,3-7").unwrap();

        assert!(range_pair.is_fully_contained());
    }

    #[test]
    fn it_checks_that_a_range_pair_is_not_contained() {
        let range_pair = RangePair::from_str("2-7,6-10").unwrap();

        assert!(!range_pair.is_fully_contained());
    }

    #[test]
    fn it_checks_the_number_of_section_assignments_that_are_fully_contained() {
        let section_assignments = SectionAssignments::try_from(input()).unwrap();

        assert_eq!(section_assignments.fully_contained_count(), 2);
    }

    fn input() -> &'static str {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
    }
}
