use traits::{News, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Flutter"),
        content: String::from("Flutter 2.8 is coming"),
        reply: true,
        retweet: false,
    };
    println!("{}", tweet.summarize());
}
