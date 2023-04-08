use tide::prelude::*;
use tide::{Body, Request, Result};

#[derive(Debug, Deserialize, Serialize)]
struct Contact {
    name: String,
    phone: u64,
}

#[derive(Deserialize)]
struct Params {
    q: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();
    let mut app = tide::new();
    app.at("/search").get(|req: Request<()>| async move {
        let q: Params = req.query()?;
        println!("{}", q.q);
        let contacts: Vec<Contact> = vec![Contact {
            name: "Bruj".to_string(),
            phone: 9988497716,
        }];
        Body::from_json(&contacts)
    });
    app.at("/").serve_file("static/home.html")?;
    app.at("/js").serve_file("static/main.js")?;
    app.at("/css").serve_file("static/style.css")?;
    app.listen("127.0.0.1:5000").await?;
    Ok(())
}
