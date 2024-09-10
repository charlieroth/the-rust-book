use traits::{NewsArticle, Summary, Tweet};

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound
pub fn notify_two<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("charlieroth"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Fire in Big City!"),
        location: String::from("The Big City"),
        author: String::from("Big City Writer #1"),
        content: String::from("Fire started in the Big City from stove explosion!"),
    };
    println!("1 new article: {}", article.summarize());
}
