mod day_one;
mod day_two;

// https://adventofcode.com/2023

fn main() {
    println!("========== Day 1 ==========");
    println!(
        "  Digits Calibration Value - {}",
        day_one::find_calibration_with_digits()
    );
    println!(
        "  All forms Calibration Value - {}",
        day_one::find_calibration_with_all()
    );

    println!("========== Day 2 ==========");
    println!(
        "  Sum of Possible Game ID's - {}",
        day_two::sum_possible_game_ids()
    );
    println!("  Sum of Power of Cubes- {}", day_two::sum_power_of_cubes())
}
