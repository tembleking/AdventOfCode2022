#![deny(unused)]

use std::collections::HashSet;
use std::slice::Chunks;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Item(char);

impl Item {
    pub fn priority(&self) -> u8 {
        if !self.0.is_ascii() {
            return 0;
        }

        if self.0.is_ascii_lowercase() {
            self.0 as u8 - b'a' + 1
        } else {
            self.0 as u8 - b'A' + 27
        }
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        Item(c)
    }
}

impl From<&Item> for char {
    fn from(item: &Item) -> Self {
        item.0
    }
}

pub struct Rucksack {
    left: HashSet<Item>,
    right: HashSet<Item>,
}

impl TryFrom<&str> for Rucksack {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if input.len() % 2 == 1 {
            return Err(format!(
                "Input length must be even, but was {}",
                input.len()
            ));
        }

        let (left, right) = input.split_at(input.len() / 2);

        Ok(Rucksack {
            left: left.chars().map(Item::from).collect(),
            right: right.chars().map(Item::from).collect(),
        })
    }
}

impl Rucksack {
    fn left(&self) -> HashSet<char> {
        self.left.iter().map(char::from).collect()
    }

    fn right(&self) -> HashSet<char> {
        self.right.iter().map(char::from).collect()
    }

    pub fn find_duplicated_element(&self) -> Option<Item> {
        self.left.intersection(&self.right).next().cloned()
    }
}

pub struct ElveGroup {
    rucksacks: Vec<Rucksack>,
}

impl TryFrom<&str> for ElveGroup {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let rucksacks = input
            .lines()
            .map(Rucksack::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ElveGroup { rucksacks })
    }
}

impl ElveGroup {
    pub fn duplicated_element_per_group_of(&self, num: usize) -> DuplicatedElementIter {
        DuplicatedElementIter {
            iter: self.rucksacks.chunks(num),
        }
    }
}

pub struct DuplicatedElementIter<'a> {
    iter: Chunks<'a, Rucksack>,
}

impl<'a> Iterator for DuplicatedElementIter<'a> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.iter.next()?;
        let mut set = Vec::new();

        for rucksack in chunk {
            let rucksack_set = rucksack
                .left()
                .union(&rucksack.right())
                .cloned()
                .map(Item::from)
                .collect::<HashSet<Item>>();
            set.push(rucksack_set);
        }

        let mut intersection = set[0].clone();
        for rucksack_set in set {
            intersection = intersection.intersection(&rucksack_set).cloned().collect();
        }

        intersection.iter().next().cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn it_retrieves_the_set_of_elements_of_each_rucksack() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::try_from(input).unwrap();

        assert_eq!(
            rucksack.left(),
            HashSet::from(['v', 'J', 'r', 'w', 'p', 'W', 't', 'g', 'W', 'r'])
        );
        assert_eq!(
            rucksack.right(),
            HashSet::from(['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p'])
        );
    }

    #[test]
    fn it_retrieves_the_duplicated_element_in_the_rucksack() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::try_from(input).unwrap();

        assert_eq!(rucksack.find_duplicated_element(), Some(Item('p')));
    }

    #[test]
    fn it_retrieves_the_priority_of_the_item() {
        let a_to_z_lower = ('a'..='z').map(Item::from);
        let a_to_z_upper = ('A'..='Z').map(Item::from);
        let items = a_to_z_lower.chain(a_to_z_upper).collect::<Vec<Item>>();

        let items_priorities = items.iter().map(Item::priority).collect::<Vec<u8>>();

        assert_eq!(items_priorities, (1..=52).collect::<Vec<u8>>());
    }

    #[test]
    fn it_retrieves_the_priority_of_the_rucksack() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::try_from(input).unwrap();
        let duplicated = rucksack.find_duplicated_element().unwrap();

        assert_eq!(duplicated.priority(), 16);
    }

    #[test]
    fn it_retrieves_the_priority_sum_of_all_rucksacks() {
        let rucksacks = input()
            .lines()
            .flat_map(Rucksack::try_from)
            .collect::<Vec<Rucksack>>();

        let duplicated_elements = rucksacks.iter().flat_map(Rucksack::find_duplicated_element);
        let priorities = duplicated_elements.map(|item| item.priority() as u32);
        let priority_sum = priorities.sum::<u32>();

        assert_eq!(priority_sum, 157);
    }

    fn input() -> &'static str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }
}
