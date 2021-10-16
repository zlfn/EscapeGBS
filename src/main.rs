#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use core::time;
use std::collections::HashMap;
use std::net::Shutdown::Read;
use std::thread;
use rocket::http::{Cookie, Cookies};
use EscapeGBS::*;
use rocket::response::content;
use EscapeGBS::DataManager::SessionIO::{ClearSession, DeleteAllSessions, GenerateSession, ReadSession, WriteSession};
use EscapeGBS::DataManager::FileIO::{BuildHTML, BuildHTML_nostate, ReadHTMLFile};
use EscapeGBS::DataManager::GameState;

static mut sessions:  Option<HashMap<String, String>> = None;

#[get("/")]
fn index(mut cookies:Cookies) -> content::Html<String> {
    let session = cookies.get("session");
    if let Some(T) = session
    {
        let session = session.unwrap().value().parse::<u32>();
        if let Ok(T) = session {return BuildHTML_nostate(-2).unwrap()}
    }

    cookies.add(Cookie::new("session",GenerateSession(600).unwrap().to_string()));
    ClearSession();
    return DataManager::FileIO::BuildHTML_nostate(1).unwrap();
}

#[post("/",data="<selection>")]
fn index_post(selection:String, mut cookies:Cookies) -> content::Html<String> {
    let session = cookies.get("session");
    if let Some(T) = session{} else{return BuildHTML_nostate(-3).unwrap();}
    let session = session.unwrap().value().parse::<u32>();
    if let Err(E) = session {return BuildHTML_nostate(-3).unwrap();}
    let session = session.unwrap();

    let mut state = ReadSession(session);
    if let Err(T) = state {
        return BuildHTML_nostate(-3).unwrap();
    }
    let mut state = state.unwrap();

    println!("page:{}",state.page);
    let gotos = selection.replace("selection=","").replace("+"," ");
    println!("{:?}",gotos);
    let goto: Vec<i32> = gotos.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("current: {}",goto[0]);
    println!("selection: {}",goto[1]);

    if(goto[1]==-1){cookies.remove(Cookie::named("session"));}
    return select(goto,session,state);

}

fn select(goto:Vec<i32>,session:u32, mut state: GameState) -> content::Html<String>
{
    if goto[1]==0 {return select(vec![goto[0],goto[0]],session,state);}
    else {match goto[0]
    {
        -3=>
            {
                match goto[1]
                {
                    -3=>return BuildHTML(-3,-3,&state).unwrap(),
                    _=>return BuildHTML(-3,-3,&state).unwrap()
                }
            }
        -2=>
            {
                match goto[1]
                {
                    -2=>return BuildHTML(-2,-2,&state).unwrap(),
                    -1=>return ReadHTMLFile("HTML/re_index.html").unwrap(),
                    _=>return BuildHTML(-2,-2,&state).unwrap()
                }
            }
        1=>
            {
                if goto[0]==state.page
                {
                    match goto[1]
                    {
                        1=>
                            {
                                state.page = 1;
                                WriteSession(session,&state);
                                return BuildHTML(1,1,&state).unwrap()
                            }
                        2=>
                            {
                                state.page = 2;
                                WriteSession(session,&state);
                                return BuildHTML(1,2,&state).unwrap()
                            }
                        _=>return BuildHTML(1,-3,&state).unwrap()
                    }
                }
                else {return BuildHTML(1,-2,&state).unwrap()}
            }
        2=>
            {
                if goto[0]==state.page
                {
                    match goto[1]
                    {
                        1=>
                            {
                                if state.condition[0]
                                {
                                    state.page = 1;
                                    WriteSession(session,&state);
                                    return BuildHTML(2,1,&state).unwrap()
                                }
                                else {return BuildHTML_nostate(-3).unwrap()}
                            }
                        2=>
                            {
                                state.page = 2;
                                WriteSession(session,&state);
                                return BuildHTML(2,2,&state).unwrap()
                            }
                        3=>
                            {
                                state.condition[0] = true;
                                state.page = 3;
                                WriteSession(session,&state);
                                return BuildHTML(2,3,&state).unwrap()
                            }
                        _=>return BuildHTML(2,-3,&state).unwrap()
                    }
                }
                else {return BuildHTML(2,-3,&state).unwrap()}
            }
        3=>
            {
                if goto[0]==state.page
                {
                    match goto[1]
                    {
                        2=>
                            {
                                state.page=2;
                                WriteSession(session,&state);
                                return BuildHTML(3,2,&state).unwrap()
                            }
                        3=>
                            {
                                state.page = 3;
                                WriteSession(session,&state);
                                return BuildHTML(3,3,&state).unwrap()
                            }
                        _=>return BuildHTML(3,-3,&state).unwrap()
                    }
                }
                else {return BuildHTML(3,-3,&state).unwrap()}
            }
        _=>return BuildHTML_nostate(-3).unwrap()
    }}
}




fn main() {
    rocket::ignite().mount("/",
                           routes![index, index_post])
        .launch();
}