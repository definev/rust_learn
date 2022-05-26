
struct Human {
    name: String,
}

impl Human {
    #[allow(dead_code)]
    fn fly(&self) {
        println!("{} can fly with airplane!", self.name);
    }
}

trait Bird {
    fn fly(&self);
}

impl Bird for Human {
    fn fly(&self) {
        println!("{} can fly with wings!", self.name);
    }
}

trait Chicken {
    fn fly(&self);
}

impl Chicken for Human {
    fn fly(&self) {
        println!("{} cannot fly! Just wacking!", self.name);
    }
}

#[test]
fn advanced_trait_with_self_ref() {
    let human = Human {
        name: String::from("John"),
    };
    human.fly(); // What happens here?
    Bird::fly(&human);
    Chicken::fly(&human);
}
