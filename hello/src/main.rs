fn main() {
    // Days of the week
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // Get the first day of the week
    let first = days[0];

    // Get the second day of the week
    let second = days[1];

    let never = days[8];

    println!(
        "days: {:?}, first: {}, second: {}, never: {:?}",
        days, first, second, never
    )
}
