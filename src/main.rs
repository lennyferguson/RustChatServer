#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;


#[derive(FromForm)]
struct ChatMsg {
    user:  String,
    msg:   String,
    topic: String,
}

#[post("/chat", data = "<chat>")]
fn chat(chat : Form<ChatMsg>, chat_hist : State<Arc<Hashmap<String,String>>>) -> Template {
    let context = TemplateContext {
        chat: chat,
        conversation: {
            match(chat_hist.get(chat.topic)) {
                Some(val) => {

                },
                None() => {

                }
            }
        }
    };
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/main.html")
}

fn main() {
    let convo_history : HashMap<String,String> = HashMap::new();
    let convo_arc = Arc::new(convo_history);
    let shared = convo_arc.clone();
    rocket::ignite()
    .mount("/", routes![index,chat])
    .manage(shared)
    .launch();
}
