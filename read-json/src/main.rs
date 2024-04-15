use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}


#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}


fn read_json(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}


fn main() {
    let json = r#"
    {
        "article": "how to work with json in rust",
        "author": "yemmyharry",
        "paragraph": [
            {
            "name": "start of paragraph"
        },
        {
            "name": " body of paragraph"
        },
        {
            "name": "end of paragraph"
        }
        ]
    }"#;

    let parsed: Article = read_json(json);
    println!("\n\n The name of the first paragraph is : {}", parsed.paragraph[0].name);

}