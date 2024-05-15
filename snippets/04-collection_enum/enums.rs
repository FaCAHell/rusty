enum IceCream {
    Fruit(String),
    Milk(String),
}

impl IceCream {
    fn enjoy(&self) {
        match self {
            IceCream::Fruit(n) => println!("i love fruity {n}"),
            IceCream::Milk(n) => println!("{n}-EIS !!1!elf"),
        }
    }
}

fn main() {
    let strawberry = IceCream::Fruit(String::from("Erdbeere"));
    strawberry.enjoy();

    let schoki = IceCream::Milk(String::from("Schokolade"));
    schoki.enjoy();
}
