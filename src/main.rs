use yarte::Template;

// Define a struct for article data
#[derive(Template)] // Derive Clone trait
#[template(path = "article")]
struct ArticleTemplate {
    title: String,
    date: String,
    content: String,
    article_url: String,
}

// Define a struct for the index page, including a vector of articles
#[derive(Template)]
#[template(path = "index")]
struct IndexTemplate {
    articles: Vec<ArticleTemplate>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define article data
    let articles: Vec<ArticleTemplate> = vec![
        ArticleTemplate {
            title: "Open source hooliganism and the TypeScript meltdown".to_string(),
            date: "September 7, 2023".to_string(),
            content: "I've seen a lot of true believers argue for virtues of their...".to_string(),
            article_url: "/dhh/open-source-hooliganism-and-the-typescript-meltdown-a474bfda".to_string(),
        },
        ArticleTemplate {
            title: "Open source hooliganism and the TypeScript meltdown".to_string(),
            date: "September 7, 2023".to_string(),
            content: "I've seen a lot of true believers argue for virtues of their...".to_string(),
            article_url: "/dhh/open-source-hooliganism-and-the-typescript-meltdown-a474bfda".to_string(),
        },
    ];

    // Create an instance of the IndexTemplate struct with the articles data
    let index_data = IndexTemplate { articles };

    // Render the index page
    let index_html = index_data.call().unwrap();

    // Print or save the generated HTML for the index page
    println!("{}", index_html);

    Ok(())
}

