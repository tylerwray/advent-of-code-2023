mod day_one;

fn main() {
    let digits_calibration = day_one::find_calibration_with_digits();
    println!("Digits Calibration Value - {}", digits_calibration);

    let all_calibration = day_one::find_calibration_with_all();
    println!("All forms Calibration Value - {}", all_calibration);
}
