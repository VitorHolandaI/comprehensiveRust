fn main() {
    let mut name = String::from("A light that comes and goes maybe its it master yoda");
    while let Some(c) = name.pop() { //enquanto name nao for vazio ! vai continuar fazendo pop ate
                                     //ficar None
        dbg!(c);
    }
}
