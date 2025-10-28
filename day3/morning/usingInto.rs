struct User {
    name: String,
    age: u32,
}



impl From<(String, u32)> for User {
    fn from(data: (String, u32)) -> Self {
        User { name: data.0, age: data.1 }
    }
}


let tuple = ("Alice".to_string(), 30);
let user: User = tuple.into(); // automatically calls From
//trait para transformar automaticamente
