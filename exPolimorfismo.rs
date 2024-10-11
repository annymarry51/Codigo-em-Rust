pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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

pub struct Book {
    pub author: String,
    pub name: String,
    pub category: String,
    pub is_rented: bool,
    pub year: u32
}

impl Summary for Book{
    fn summarize(&self) -> String {
        format!("Dados do livro: {}\n {} \n {} \n {} \n {}",self.author, self.name, self.category, self.is_rented, self.year)
    }
}

fn main(){
    //Criado pela documentação
       let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    
    //Criado pela Equipe
        let book = Book {
            author: String::from("Rick Riordan"),
            name: String::from("A maldição do titã"),
            category: String::from("Aventura"),
            is_rented: false,
            year: 2014
    };
    
    println!("A new book added: {}",book.summarize());
    
}