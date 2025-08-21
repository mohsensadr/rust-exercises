#[derive(Debug)]
enum Media{
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String }
}

impl Media{
    fn description(&self) -> String {
        if let Media::Book {title, author} = self {
            format!("Book: {} {}", title, author)
        }
        else if let Media::Movie {title, director} = self { 
            format!("Movie {} {}", title, director)
        }
        else if let Media::Audiobook {title} = self {
            format!("Audiobook {}", title)
        }
        else{
            format!("Media not found!")
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
