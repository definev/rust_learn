pub struct Post {}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: "".to_string(),
        }
    }
}

pub struct DraftPost {
    pub content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, content: &str) {
        self.content.push_str(content);
    }

    pub fn request_review(&self) -> PendingReviewPost {
        PendingReviewPost {
            content: String::from(self.content.as_str()),
        }
    }
}

pub struct PendingReviewPost {
    pub content: String,
}

impl PendingReviewPost {
    pub fn approve(&self) -> PublishedPost {
        PublishedPost {
            content: String::from(self.content.as_str()),
        }
    }
}

pub struct PublishedPost {
    pub content: String,
}

impl PublishedPost {
    pub fn content(&self) -> &str {
        self.content.as_str()
    }
}
