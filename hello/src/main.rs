fn main() {
    // Define a tuple struct
    #[derive(Debug)]
    struct KeyPress(String, char);

    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick {
        x: i64,
        y: i64,
    }

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool),
        WEClick(MouseClick),
        WEKeys(KeyPress),
    }

    let we_load = WebEvent::WELoad(true);

    let click = MouseClick { x: 100, y: 250 };
    let we_click = WebEvent::WEClick(click);

    let keys = KeyPress(String::from("Ctrl+"), 'N');
    let we_key = WebEvent::WEKeys(keys);

    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );
    good_by("kasumi")
}

fn good_by (message: &str) {
  println!("hello world, {}", message)
}
