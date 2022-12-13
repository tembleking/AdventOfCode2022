use advent05::CargoCrane;

fn main() {
    crane_mover_9000();
    crane_mover_9001();
}

fn crane_mover_9000() {
    let mut crane = CargoCrane::try_from(input()).unwrap();
    crane
        .execute_instructions_crate_mover_9000()
        .expect("Failed to execute instructions");

    let ship = crane.ship();
    let message = ship.crates_message_to_elves();

    println!("The message to the elves with CraneMover9000 is: {}", message);
}

fn crane_mover_9001() {
    let mut crane = CargoCrane::try_from(input()).unwrap();
    crane
        .execute_instructions_crate_mover_9001()
        .expect("Failed to execute instructions");

    let ship = crane.ship();
    let message = ship.crates_message_to_elves();

    println!("The message to the elves with CraneMover9001 is: {}", message);
}

fn input() -> &'static str {
    include_str!("input.txt")
}
