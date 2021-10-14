#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::collections::HashMap;
use rocket::http::{Cookie, Cookies};
use EscapeGBS::*;
use rocket::response::content;
use EscapeGBS::DataManager::SessionIO::{ClearSession, DeleteAllSessions, GenerateSession, ReadSession, WriteSession};
use EscapeGBS::DataManager::FileIO::{GameState, ReadHTMLFile};

static mut sessions:  Option<HashMap<String, String>> = None;

#[get("/")]
fn index(mut cookies:Cookies) -> content::Html<String> {
    cookies.add(Cookie::new("session",GenerateSession(600).unwrap().to_string()));
    ClearSession();
    return DataManager::FileIO::ReadHTMLFile("HTML/template.html").unwrap();
}

#[post("/",data="<selection>")]
fn index_post(selection:String, cookies:Cookies) -> content::Html<String> {
    let session = cookies.get("session").unwrap().value().parse::<u32>().unwrap(); //TODO:세션쿠키 없을때 예외처리
    let mut state = ReadSession(session).unwrap(); //TODO: 세션 만료되었을 때 예외처리
    println!("page:{}",state.page);
    let gotos = selection.replace("selection=","").replace("+"," ");
    println!("{:?}",gotos);
    let goto: Vec<i32> = gotos.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("current: {}",goto[0]);
    println!("selection: {}",goto[1]);

    return select(goto,session,state);

}

fn select(goto:Vec<i32>, session:u32, mut state: EscapeGBS::DataManager::GameState) -> content::Html<String>
{
    match goto[1]
    {
        0=>
            {
                if state.page == goto[0]
                {
                    return select(vec![goto[0],goto[0]],session,state);
                }
                else
                {
                    return ReadHTMLFile("HTML/Error.html").unwrap();
                }
            }
        1=>
            {
                if (state.page == goto[0]&&(goto[0]==2||goto[0]==1))
                {
                    state.page = 1;
                    WriteSession(session, state);
                    return ReadHTMLFile("HTML/template.html").unwrap();
                }
                else
                {
                    return ReadHTMLFile("HTML/Error.html").unwrap();
                }

            }
        2=>
            {
                if (state.page == goto[0]&&(goto[0]==1||goto[0]==2))
                {
                    state.page = 2;
                    WriteSession(session, state);
                    return ReadHTMLFile("HTML/postest.html").unwrap();
                }
                else
                {
                    return ReadHTMLFile("HTML/Error.html").unwrap();
                }
            }
        3=>
            {
                if (state.page == goto[0]&&(goto[0]==2||goto[0]==3))
                {
                    state.page=3;
                    WriteSession(session,state);
                    return ReadHTMLFile("HTML/sectest.html").unwrap();
                }
                else {
                    return ReadHTMLFile("HTML/Error.html").unwrap();
                }
            }
        _=>
            {
                return ReadHTMLFile("/HTML/Error.html").unwrap();
            }
    };
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