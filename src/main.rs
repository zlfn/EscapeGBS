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
use EscapeGBS::DataManager::FileIO::{ReadFile, BuildHTML, BuildHTML_nostate, ReadHTMLFile, JsonFormat, GetState};
use EscapeGBS::DataManager::GameState;

static mut sessions:  Option<HashMap<String, String>> = None;

#[get("/")]
fn index() -> content::Html<String> {
    return ReadHTMLFile("HTML/Main.html").unwrap();
}

#[get("/game")]
fn game(mut cookies:Cookies) -> content::Html<String> {
    let session = cookies.get("session");
    if let Some(T) = session
    {
        let session = session.unwrap().value().parse::<u32>();
        if let Ok(T) = session {return BuildHTML_nostate(-2).unwrap()}
    }

    cookies.add(Cookie::new("session",GenerateSession(600).unwrap().to_string()));
    ClearSession();
    let session = cookies.get("session");
    if let Some(T) = session{} else{return BuildHTML_nostate(-3).unwrap();}
    let session = session.unwrap().value().parse::<u32>();
    if let Err(E) = session {return BuildHTML_nostate(-3).unwrap();}
    let session = session.unwrap();

    let mut state = ReadSession(session);
    return DataManager::FileIO::BuildHTML(0,111001,&state.unwrap()).unwrap();
}

#[post("/game",data="<selection>")]
fn game_post(selection:String, mut cookies:Cookies) -> content::Html<String> {

    let gotos = selection.replace("selection=","").replace("+"," ");
    println!("{:?}",gotos);
    let goto: Vec<i32> = gotos.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    if(goto[1]==-1){
        cookies.remove(Cookie::named("session"));
        cookies.add(Cookie::new("session",GenerateSession(600).unwrap().to_string()));
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
        return BuildHTML(0,111001,&state).unwrap();
    }

    if(goto[1]==-4){
        cookies.remove(Cookie::named("session"));
        cookies.add(Cookie::new("session",GenerateSession(600).unwrap().to_string()));
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
        return ReadHTMLFile("HTML/re_main.html").unwrap();
    }


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

    println!("{:?}",goto);
    return select(goto,session,state);
}

fn check(goto:Vec<i32>, session: u32, mut state: &mut GameState) -> content::Html<String>
{
    println!("b {} {}",goto[0],state.page);
    let mut json_path = format!("JSON/{}.json", goto[0]);
    let mut file = &ReadFile(&json_path).unwrap()[..];
    let mut data: JsonFormat = serde_json::from_str(file).unwrap();

    println!("{} {} {}", goto[0],goto[1],goto[2]);

    if goto[0]!=state.page
    {
        println!("Error");
        return BuildHTML_nostate(-3).unwrap();
    }
    if data.choices[usize::try_from(goto[2]).unwrap()].conreq==true
    {
        for i in data.choices[usize::try_from(goto[2]).unwrap()].condition.iter()
        {
            if (!state.condition[*i])
            {
                return BuildHTML_nostate(-3).unwrap();
            }
        }
    }
    if data.choices[usize::try_from(goto[2]).unwrap()].gadreq==true
    {
        for i in data.choices[usize::try_from(goto[2]).unwrap()].gadget.iter()
        {
            if (!state.gadget[*i])
            {
                return BuildHTML_nostate(-3).unwrap();
            }
        }
    }
    state.page = goto[1];
    GetState(goto[1],state);
    WriteSession(session,state);
    return BuildHTML(i32::try_from(goto[0]).unwrap(),i32::try_from(goto[1]).unwrap(),state).unwrap();
}

fn select(goto:Vec<i32>,session:u32, mut state: GameState) -> content::Html<String>
{
    println!("a {} {}", goto[0],goto[1]);
    if goto[1]==0 {println!("h");return check(vec![goto[0],goto[0],0],session,&mut state);}
    else { match goto[0]
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
        0=>
            {
                match goto[1]
                {
                    -1=> return BuildHTML(0, 111001,&state).unwrap(),
                    _=> return BuildHTML(0, -2, &state).unwrap()
                }
            }
        _=>return check(goto, session, &mut state)
    }}
}




fn main() {
    rocket::ignite().mount("/",
                           routes![index, game, game_post])
        .launch();
}