#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        let a = inches as f64 * 2.54;

        Centimeters(a)
    }
}

struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement the `Debug` trait
    // エラー: `Seconds`はプリントできない。これは`Debug`トレイトを実装していないため
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Error: `Seconds` can't be compared; it doesn't implement the `PartialEq` trait
    // エラー: `Seconds`は比較できない。これは`PartialEq`トレイトを実装していないため
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter.", cmp);
}
