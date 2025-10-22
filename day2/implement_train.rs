trait Pet {
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("Oh you are cutie! Whays your name? {} ", self.talk());
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

fn main() {
    let fido = Dog {
        name: String::from("Fido"),
        age: 5,
    };
    dbg!(fido.talk());
    fido.greet();
}

//theres aditional like inheritance to it on supertraits that means trait that depends on other
//trait
//
//
trait Animal {
    fn leg_count(&self) -> u32;
}

trait Pet: Animal {
    fn name(&self) -> String;
}

struct Dog(String);

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}
