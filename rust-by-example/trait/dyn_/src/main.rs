struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &str;
}

impl Animal for Sheep {
    fn noise(&self) -> &str {
        "bah!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &str {
        "mo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );
}
