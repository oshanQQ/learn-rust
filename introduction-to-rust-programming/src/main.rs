struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) -> &Self {
        println!("I am {}", self.name);
        self
    }

    fn say_age(&self) -> &Self {
        println!("I am {} year(s) old", self.age);
        self
    }
}

fn main() {
    let p = Person {
        name: String::from("Ahiahi"),
        age: 20,
    };

    p.say_name();
    p.say_age();
    p.say_name().say_age();
}
