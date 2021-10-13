#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket::http::{Cookie, Cookies};
use EscapeGBS::*;
use rocket::response::content;
use EscapeGBS::DataManager::FileIO::{ClearSession, GenerateSession};
use EscapeGBS::DataManager::FileIO::GameState;

static mut sessions:  Option<HashMap<String, String>> = None;

#[get("/")]
fn index(mut cookies:Cookies) -> content::Html<String> {
    cookies.add(Cookie::new("session",GenerateSession(3600).unwrap().to_string()));
    ClearSession();
    return DataManager::FileIO::ReadHTMLFile("HTML/template.html").unwrap();
}

#[post("/",data="<selection>")]
fn index_post(selection:String, cookies:Cookies) -> content::Html<String> {
    selection.replace("selection=","");
    println!("selection: {}",selection);
    return DataManager::FileIO::ReadHTMLFile("HTML/postest.html").unwrap();
}

#[get("/css")]
fn css() -> content::Css<String> {
    return DataManager::FileIO::ReadCSSFile("CSS/template.css").unwrap();
}

#[get("/js")]
fn js() -> content::JavaScript<String> {
    return DataManager::FileIO::ReadJSFile("JavaScript/template.js").unwrap();
}

#[get("/postest")]
fn postest() -> content::JavaScript<String> {
    return DataManager::FileIO::ReadJSFile("JavaScript/postest.js").unwrap();
}

fn main() {
    rocket::ignite().mount("/",
                           routes![index, index_post, postest, css, js])
        .launch();
}