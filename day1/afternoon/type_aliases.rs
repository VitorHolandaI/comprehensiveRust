enum CarryableCOncreteItem {
    Big,
    Small,
}

type Item = CarryableCOncreteItem; // bom para nomes grandes
fn main() {
    let item = Item::Big;
    match item {
        Item::Big => println!("Big item"),
        Item::Small => println!("Small item"),
    }
}
