use advent03::{ElveGroup, Rucksack};

fn main() {
    sum_priorities();
    sum_priorities_with_elve_groups();
}

fn sum_priorities() {
    let rucksacks = input().lines().flat_map(Rucksack::try_from);

    let priorities = rucksacks.flat_map(|rucksack| {
        rucksack
            .find_duplicated_element()
            .map(|item| item.priority() as u32)
    });

    let priority_sum = priorities.sum::<u32>();
    println!("Sum of priorities: {}", priority_sum);
}

fn sum_priorities_with_elve_groups() {
    let elve_group = ElveGroup::try_from(input()).unwrap();
    let iter = elve_group.duplicated_element_per_group_of(3);
    let priorities = iter.map(|item| item.priority() as u32).sum::<u32>();

    println!("Sum of priorities from groups: {}", priorities);
}

fn input() -> &'static str {
    include_str!("input.txt")
}
