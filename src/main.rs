mod theme;

use dotenv::dotenv;
use newsapi::{Article, Country, Endpoint, NewsAPI};
use std::error::Error;

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for i in articles {
        theme.print_text(&format!("`{}`", i.title()));
        theme.print_text(&format!("> *{}*", i.content()));
        theme.print_text(&format!("Source: *{}*", i.source()));
        theme.print_text("---");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let mut newsapi = NewsAPI::new();
    newsapi
        .endpoint(Endpoint::TopHeadlines)
        .country(Country::Us);

    let newsapi_response = newsapi.fetch_async().await?;
    render_articles(&newsapi_response);

    Ok(())
}
