fn main() {
    let r = &factorial(6);

    assert_eq!(r + &1009, 1729);
}

fn factorial(n: usize) -> usize {
    (1..n + 1).product()
}
