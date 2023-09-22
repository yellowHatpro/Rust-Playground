pub mod traits {
    //Same as interfaces

    pub struct NewsArticle {
        pub author: String,
        pub headline: String,
        pub content: String,
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: String,
        pub retweet: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub fn traits() {}

    fn main() {
        let _article = NewsArticle {
            author: String::from("Ashu Aswal"),
            headline: String::from("The sky is falling"),
            content: String::from("The sky is not actually falling"),
        };
    }
}
