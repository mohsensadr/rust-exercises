mod content;

use content::media::Media;
use content::catalog::Catalog;

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
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    println!("{}", audiobook.description());
    println!("{}", goodbook.description());
    println!("{}", badmovie.description());

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(goodbook);
    catalog.add(badmovie);
    catalog.add(podcast);
    catalog.add(placeholder);
    println!("{:#?}", catalog);

    let item = catalog.get_by_index(0);

    match item {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("No value here!");
        }
    }

}
