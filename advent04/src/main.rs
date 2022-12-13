use advent04::SectionAssignments;

fn main() {
    let assigments = SectionAssignments::try_from(input()).unwrap();

    println!("Number of fully contained sections: {}", assigments.fully_contained_count());
    println!("Number of sections overlapping: {}", assigments.overlap_count());
}

fn input() -> &'static str {
    include_str!("input.txt")
}
