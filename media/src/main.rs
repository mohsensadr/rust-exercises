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

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog{
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media){
        self.items.push(media);
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

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(goodbook);
    catalog.add(badmovie);

    println!("{:#?}", catalog);
}
