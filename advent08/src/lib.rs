#![allow(unused)]

use itertools::Itertools;

struct Tree {
    height: u8,
}

impl Tree {
    pub fn new(size: u8) -> Tree {
        Tree { height: size }
    }

    pub fn height(&self) -> u8 {
        self.height
    }
}

struct Forest {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

impl Forest {
    pub fn from_str(input: &str) -> Result<Forest, String> {
        let mut trees = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in input.lines() {
            width = line.trim().len();
            for char in line.chars() {
                let tree = Tree::new(char.to_digit(10).unwrap_or(0) as u8);
                trees.push(tree);
            }
            height += 1;
        }

        Ok(Forest {
            trees,
            width,
            height,
        })
    }

    fn get_tree(&self, x: usize, y: usize) -> Option<&Tree> {
        if y >= self.height {
            return None;
        }
        let x = x % self.width;
        let index = y * self.width + x;
        self.trees.get(index)
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let tree_to_check = self.get_tree(x, y).unwrap();
        let mut visibility_from_sides = [true, true, true, true];

        let is_tree_visible_from_left = (0..x).all(|x| {
            let tree = self.get_tree(x, y).unwrap();
            tree.height() < tree_to_check.height()
        });

        let is_tree_visible_from_right = (x + 1..self.width).all(|x| {
            let tree = self.get_tree(x, y).unwrap();
            tree.height() < tree_to_check.height()
        });

        let is_tree_visible_from_top = (0..y).all(|y| {
            let tree = self.get_tree(x, y).unwrap();
            tree.height() < tree_to_check.height()
        });

        let is_tree_visible_from_bottom = (y + 1..self.height).all(|y| {
            let tree = self.get_tree(x, y).unwrap();
            tree.height() < tree_to_check.height()
        });

        is_tree_visible_from_top
            || is_tree_visible_from_bottom
            || is_tree_visible_from_left
            || is_tree_visible_from_right
    }

    pub fn visible_tree_count(&self) -> usize {
        (0..self.width())
            .cartesian_product((0..self.height()))
            .filter(|(x, y)| self.is_tree_visible(*x, *y))
            .count()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_the_forest() {
        let forest = Forest::from_str(input()).unwrap();

        assert_eq!(forest.width(), 5);
        assert_eq!(forest.height(), 5);
        assert_eq!(forest.get_tree(0, 0).unwrap().height(), 3);
        assert_eq!(forest.get_tree(1, 0).unwrap().height(), 0);
        assert_eq!(forest.get_tree(2, 0).unwrap().height(), 3);
        assert_eq!(forest.get_tree(3, 0).unwrap().height(), 7);
        assert_eq!(forest.get_tree(4, 0).unwrap().height(), 3);
        assert_eq!(forest.get_tree(0, 1).unwrap().height(), 2);
        assert_eq!(forest.get_tree(0, 2).unwrap().height(), 6);
        assert_eq!(forest.get_tree(0, 3).unwrap().height(), 3);
        assert_eq!(forest.get_tree(0, 4).unwrap().height(), 3);
        assert_eq!(forest.get_tree(4, 4).unwrap().height(), 0);
    }

    #[test]
    fn it_shows_that_trees_from_the_outside_of_the_grid_are_visible() {
        let forest = Forest::from_str(input()).unwrap();

        assert!(forest.is_tree_visible(0, 0));
        assert!(forest.is_tree_visible(1, 0));
        assert!(forest.is_tree_visible(2, 0));
        assert!(forest.is_tree_visible(3, 0));
        assert!(forest.is_tree_visible(4, 0));
        assert!(forest.is_tree_visible(0, 1));
        assert!(forest.is_tree_visible(0, 2));
        assert!(forest.is_tree_visible(0, 3));
        assert!(forest.is_tree_visible(0, 4));
        assert!(forest.is_tree_visible(4, 1));
        assert!(forest.is_tree_visible(4, 2));
        assert!(forest.is_tree_visible(4, 3));
        assert!(forest.is_tree_visible(4, 4));
        assert!(forest.is_tree_visible(1, 4));
        assert!(forest.is_tree_visible(2, 4));
        assert!(forest.is_tree_visible(3, 4));
    }

    #[test]
    fn it_shows_that_trees_that_are_higher_than_any_row_or_column_are_visible() {
        let forest = Forest::from_str(input()).unwrap();

        assert!(forest.is_tree_visible(1, 1));
    }

    #[test]
    fn it_shows_that_trees_that_are_lower_than_all_row_or_column_are_not_visible() {
        let forest = Forest::from_str(input()).unwrap();

        assert!(!forest.is_tree_visible(3, 1));
    }

    #[test]
    fn it_counts_the_number_of_visible_trees() {
        let forest = Forest::from_str(input()).unwrap();

        let visible_trees = forest.visible_tree_count();

        assert_eq!(visible_trees, 21);
    }

    fn input() -> &'static str {
        "\
30373
25512
65332
33549
35390"
    }
}
