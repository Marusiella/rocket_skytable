#[macro_use] extern crate rocket;

use rocket::response::content::Html;
use skytable::actions::Actions;
use skytable::Connection;
use skytable::ConnectionBuilder;
use rand::Rng;
use urlencoding::decode;
use rocket::{Build, Rocket};


const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                            0123456789";
const PASSWORD_LEN: usize = 4;


#[get("/api/set/<x>")]
fn index(x:&str) -> String {
    use rand::prelude::*;
    let mut thread_rng = thread_rng();
    let link: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = thread_rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    let mut con = Connection::new("127.0.0.1", 2003).unwrap();
    con.set(&link, x).unwrap();
    link
}

#[get("/<x>")]
fn get(x:&str) -> Html<String> {
    let mut con = Connection::new("127.0.0.1", 2003).unwrap();
    let key = match con.get(x) {
        Ok(t) => t,
        Err(_) => format!("null"),
    };
    // decode(&key).unwrap().to_string()
    Html(format!(r#"<script type="text/javascript">
            window.location.href = "{}"
        </script>"#,decode(&key).unwrap().to_string()))

}
#[post("/x", data="<input>")]
fn test(input: String) -> Html<String> {
    let input: String = input[5..].parse().unwrap();
    use rand::prelude::*;
    let mut thread_rng = thread_rng();
    let link: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = thread_rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    let mut con = ConnectionBuilder::new()
        .set_host("127.0.0.1".to_string())
        .set_port(2003)
        .set_entity("default:default".to_owned())
        .get_connection()
        .unwrap();
    // con.set(&link, &input).unwrap();
    // con.update(&link, input).unwrap();
    con.uset([link.clone()], [input]).unwrap();
    Html(format!(r#"<a href="http://localhost:8000/{}">http://localhost:8000/{}</a>"#,link,link))
    // Html(format!(r#"<a href="http://localhost:8000/{}">http://localhost:8000/{}</a>
    // <script type="text/javascript">
    //   function isValidHttpUrl(string) {{
    //     let url;
    //
    //     try {{
    //       url = new URL(string);
    //     }} catch (_) {{
    //       return false;
    //     }}
    //
    //     return true;
    //   }}
    //   if (isValidHttpUrl("http://www.google.com/")) {{
    //       document.write("")
    //   }}
    // </script>
    // "#,link,link))
}
#[get("/")]
fn wew() -> Html<&'static str> {
    Html(r#"<html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>link shortener</title>
    </head>
    <body>
        <h1 align="center">link shortener</h1>
        <form method="post" action="/x" id="form">
            <input type="text" id="link" name="link">
            <button type="submit"> apply </button>
        </form>
        <style type="text/css">
            #form{
                text-align: center;
            }
        </style>
    </body>
</html>"#)
}
#[get("/null")]
fn null() -> Html<&'static str> {
    Html("this page does not exist")
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index,get,test,wew,null])
}