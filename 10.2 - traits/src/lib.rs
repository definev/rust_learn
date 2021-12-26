pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct News {
    title: String,
    subtitle: String,
    author: String,
    content: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        format!(
            "TITLE: {}\nCONTENT: {}\nAUTHOR: {}\nSUBTITLE: {}",
            self.title, self.content, self.author, self.subtitle
        )
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}
