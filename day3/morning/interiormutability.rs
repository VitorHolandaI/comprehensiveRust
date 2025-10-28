use std::cell::Cell;

fn main() {
    // Note that `cell` is NOT declared as mutable.
    let cell = Cell::new(5);
    //nao pode mudar o 5 em si somente fazendo getting e setting e rust faz runtime check

    cell.set(123);
    dbg!(cell.get());
}
