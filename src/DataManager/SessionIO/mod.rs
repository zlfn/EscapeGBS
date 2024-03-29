use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, LineWriter, Read, Write};
use std::path::Path;
use super::*;

///입력된 시간(초) 후에 만료되는 세션을 생성합니다.
///
/// # Example
/// ```
/// DataManager::SessionIO::GenerateSession(3600).unwrap();
/// ```
///
pub fn GenerateSession(tt: i64) -> Result<u32,io::Error>
{
    let mut key :u32;
    let mut file_name:String;
    loop {
        key = rand::random::<u32>();
        file_name = format!("DataBase/{}",key);
        if match File::open(&file_name) { Ok(T)=>false, Err(E)=>true }
        {
            break;
        }
    }
    let mut file = File::create(file_name).unwrap();
    let now = GetUnixTime();

    let mut state = GameState
{
    page:111001,
    condition: vec![false;20],
    gadget: vec![false;20]
};
    let s = format!("{}\n{}\n\
                    {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\
                    {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    now+tt,
                    state.page,
                    state.condition[0],
                    state.condition[1],
                    state.condition[2],
                    state.condition[3],
                    state.condition[4],
                    state.condition[5],
                    state.condition[6],
                    state.condition[7],
                    state.condition[8],
                    state.condition[9],
                    state.condition[10],
                    state.condition[11],
                    state.condition[12],
                    state.condition[13],
                    state.condition[14],
                    state.condition[15],
                    state.condition[16],
                    state.condition[17],
                    state.condition[18],
                    state.condition[19],
                    state.gadget[0],
                    state.gadget[1],
                    state.gadget[2],
                    state.gadget[3],
                    state.gadget[4],
                    state.gadget[5],
                    state.gadget[6],
                    state.gadget[7],
                    state.gadget[8],
                    state.gadget[9],
                    state.gadget[10],
                    state.gadget[11],
                    state.gadget[12],
                    state.gadget[13],
                    state.gadget[14],
                    state.gadget[15],
                    state.gadget[16],
                    state.gadget[17],
                    state.gadget[18],
                    state.gadget[19],

    ); //TODO
    file.write(s.as_bytes());
    println!("session {} created",key);
    return Ok(key);
}

///만료된 세션을 삭제합니다.
///
/// # Example
/// ```
/// DataManager::SessionIO::ClearSession();
/// ```
///
pub fn ClearSession()
{
    let paths = fs::read_dir("./DataBase/").unwrap();
    for path in paths
    {
        let s = path.unwrap().path();
        let mut file = File::open(&s).unwrap();
        let mut reader = BufReader::new(file);
        let mut tts :String = String::new();
        reader.read_line(&mut tts).unwrap();
        let tts = tts.replace("\n","");
        let mut tt = tts.parse::<i64>().unwrap();
        if(tt<= GetUnixTime())
        {
            fs::remove_file(&s);
            let mut path = format!("{}",s.display());
            let number = path.replace("./DataBase/","");
            println!("session {} removed",number);
        }
    }
}

///만료되지 않았더라도 모든 세션을 삭제합니다.
///
/// # Example
/// ```
/// DataManager::SessionIO::DeleteAllSessions();
/// ```
///
pub fn DeleteAllSessions()
{
    let paths = fs::read_dir("./DataBase/").unwrap();
    for path in paths
    {
        let s = path.unwrap().path();
        let mut file = File::open(&s).unwrap();
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_line(&mut contents);
        let mut tts = contents.replace("\n","");
        fs::remove_file(&s);
        let mut path = format!("{}",s.display());
        let number = path.replace("./DataBase/","");
        println!("session {} removed",number);
    }
}


///세션의 내용을 읽어 GameState로 반환합니다.
///세션의 만료시간은 읽지 않습니다.
///
/// # Example
/// ```
/// let mut state = DataManager::SessionIO::ReadSession(324234);
/// ```
///
pub fn ReadSession(session:u32) ->Result<GameState,io::Error>
{
    let file_name = format!("DataBase/{}", session);
    let mut filedata = File::open(file_name)?;
    let mut reader = BufReader::new(filedata);

    let mut state: GameState = GameState{page:0,condition:vec![false;20],gadget:vec![false;20]};

    let mut i = 0;
    for text in reader.lines()
    {
        if i == 0 {}
        else if i == 1
        {
            let mut input = text.unwrap();
            let v = input.parse::<i32>().unwrap();
            state.page = v;
        }
        else if 2<=i&&i<=21
        {
            let mut input = text.unwrap();
            let v = input.parse::<bool>().unwrap();
            state.condition[i-2]=v;
        }
        else if 22<=i&&i<=41
        {
            let mut input = text.unwrap();
            let v = input.parse::<bool>().unwrap();
            state.gadget[i-22]=v;
        }

        //TODO

        i = i+1;
    }

    return Ok(state);
}

///GameState를 받아 세션에 작성하며, 세션을 연장합니다.
///
/// # Example
/// ```
/// DataManager::SessionIO::WriteSession(342398,state);
/// ```
///
pub fn WriteSession(session:u32, state:&GameState) -> Result<i32,io::Error>
{
    let file_name = format!("DataBase/{}", session);
    let tt = GetUnixTime()+600;

    let mut mofile = fs::OpenOptions::new().write(true).truncate(true).open(file_name).unwrap();
    mofile.write(String::from(format!("{}\n{}\n\
                    {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n\
                    {}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    tt,
                    state.page,
                    state.condition[0],
                    state.condition[1],
                    state.condition[2],
                    state.condition[3],
                    state.condition[4],
                    state.condition[5],
                    state.condition[6],
                    state.condition[7],
                    state.condition[8],
                    state.condition[9],
                    state.condition[10],
                    state.condition[11],
                    state.condition[12],
                    state.condition[13],
                    state.condition[14],
                    state.condition[15],
                    state.condition[16],
                    state.condition[17],
                    state.condition[18],
                    state.condition[19],
                    state.gadget[0],
                    state.gadget[1],
                    state.gadget[2],
                    state.gadget[3],
                    state.gadget[4],
                    state.gadget[5],
                    state.gadget[6],
                    state.gadget[7],
                    state.gadget[8],
                    state.gadget[9],
                    state.gadget[10],
                    state.gadget[11],
                    state.gadget[12],
                    state.gadget[13],
                    state.gadget[14],
                    state.gadget[15],
                    state.gadget[16],
                    state.gadget[17],
                    state.gadget[18],
                    state.gadget[19])).as_bytes()); //TODO

    return Ok(0);
}