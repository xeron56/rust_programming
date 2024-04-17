// Chapter 6: Traits and Generics
use std::iter::Iterator;
// 1. Traits
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 2. Trait Bounds
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 3. Generics
fn maximum<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// 4. Associated Types
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Range {
    start: i32,
    end: i32,
    current: i32,
}

impl Iterator for Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let value = self.current;
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}



pub(crate) fn chapter6_main() {
    // Traits
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup!"),
        location: String::from("Pittsburgh, PA"),
        author: String::from("Abe Knightly"),
        content: String::from("The Pittsburgh Penguins won the Stanley Cup last night."),
    };
    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // Trait Bounds
    notify(&article);
    notify(&tweet);

    // Generics
    let max_value = maximum(5, 10);
    println!("The maximum of 5 and 10 is {}", max_value);

    let max_float = maximum(3.14, 2.71);
    println!("The maximum of 3.14 and 2.71 is {}", max_float);

    // Associated Types
    let range = Range {
        start: 1,
        end: 5,
        current: 1,
    };

    for num in range {
        println!("{}", num);
    }
}