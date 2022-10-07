fn fizzbuzz(number: i32) -> String {
    if number % 15 == 0 {
        "FizzBuzz".to_string()
    } else if number % 3 == 0 {
        "Fizz".to_string()
    } else if number % 5 == 0 {
        "Buzz".to_string()
    } else {
        number.to_string()
    }
}

fn main() {
    let seed = 1..100;
    let result = seed
        .map(fizzbuzz)
        .fold("".to_string(), |acc, cur| format!("{}\n{}", acc, cur));

    println!("{}", result);
}
