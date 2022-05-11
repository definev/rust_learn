use oop_blog_post::oop_aproach;
use oop_blog_post::rust_aproach;

fn main() {
    let mut post = oop_aproach::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = rust_aproach::Post::new();
    assert_eq!(post.content, String::from(""));

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
