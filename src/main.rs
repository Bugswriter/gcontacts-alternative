use rusqlite::Connection;
use tide::prelude::*;
use tide::{Body, Request};

#[derive(Debug, Deserialize, Serialize)]
struct Contact {
    name: String,
    phone: i64,
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
        let con = Connection::open("./site.db")?;
        let q: Params = req.query()?;
        let mut contacts: Vec<Contact> = vec![];
        let mut stmt = con.prepare(format!("SELECT * FROM phonebook where `names` LIKE '%{}%'", q.q).as_str())?;
        let person_iter = stmt.query_map([], |row| {
            let phone: String = row.get(1)?;
            Ok(Contact {
                name: row.get(0)?,
                phone: phone.parse().unwrap(),
            })
        })?;
        for person in person_iter {
            contacts.push(person.unwrap());
        }
        Body::from_json(&contacts)
    });
    app.at("/").serve_file("static/home.html")?;
    app.at("/js").serve_file("static/main.js")?;
    app.at("/css").serve_file("static/style.css")?;
    app.listen("127.0.0.1:5000").await?;
    Ok(())
}
