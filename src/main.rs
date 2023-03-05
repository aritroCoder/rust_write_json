use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main(){
    let article: Article = Article{
        article: String::from("godfather"),
        author: String::from("Megastar Chiranjeevi"),
        paragraph: vec![
            Paragraph{
                name: String::from("Megastar Chiranjeevi")
            },
            Paragraph{
                name: String::from("Selmon Bhoi")
            },
            Paragraph{
                name: String::from("Godfather")
            },
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is: {}", json);
}


