struct Mankind;

impl Mankind {
    // It like static method in Dart
    #[allow(dead_code)]
    fn talk() {
        println!("Hi, I'm a human!");
    }
}

trait Talkative {
    fn talk();
}

impl Talkative for Mankind {
    fn talk() {
        println!("Hi, I'm just wanna talk! So much!!!!!");
    }
}

trait Introvert {
    fn talk();
}

impl Introvert for Mankind {
    fn talk() {
        println!("I don't like to talk ...");
    }
}

#[test]
fn advanced_trait_with_multiple_traits() {
    Mankind::talk();
    <Mankind as Talkative>::talk();
    <Mankind as Introvert>::talk();
}
