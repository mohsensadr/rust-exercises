#[derive(Debug)]
enum Media{
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String }
}

impl Media{
    fn description(&self) -> String {
        match self{
            Media::Book {title, author} => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie {title, director} => {
                format!("Movie {} {}", title, director)
            }
            Media::Audiobook {title} => {
                format!("Audiobook {}", title)
            } 
        }
    }
}

fn main() {
    let audiobook = Media::Audiobook{
        title: String::from("An Audiobook")
    };

    let goodbook = Media::Book{
        title: String::from("Good Book"),
        author: String::from("Good Author")
    };

    let badmovie = Media::Movie{
        title: String::from("Bad Movie"),
        director: String::from("Bad Director")
    };

    println!("{}", audiobook.description());
    println!("{}", goodbook.description());
    println!("{}", badmovie.description());
}
