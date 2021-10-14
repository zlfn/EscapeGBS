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
    page:1
};
    let s = format!("{}\n{}\n",now+tt,state.page); //TODO
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
///만약 세션 만료시간이 얼마 남지 않았다면 세션을 연장합니다.
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

    let mut state: GameState = GameState{page:0};

    let mut i = 0;
    for text in reader.lines()
    {
        if i == 0
        {
            let mut input = text.unwrap();
            if GetUnixTime()+900 >= input.parse::<i64>().unwrap()
            {
                //TODO: 세션 연장
            }
        }
        else if i == 1
        {
            let mut input = text.unwrap();
            let v = input.parse::<i32>().unwrap();
            state.page = v;
        }

        //TODO


        i = i+1;
    }

    return Ok(state);
}

///GameState를 받아 세션에 작성합니다.
///세션 만료 시간은 변경되지 않습니다.
///
/// # Example
/// ```
/// DataManager::SessionIO::WriteSession(342398,state);
/// ```
///
pub fn WriteSession(session:u32, state:GameState) -> Result<i32,io::Error>
{
    let file_name = format!("DataBase/{}", session);
    let mut filedata = File::open(&file_name).unwrap();
    let mut reader = BufReader::new(&filedata);

    let mut s : String = String::new();
    reader.read_line(&mut s)?;
    let mut input = s.replace("\n","");
    let tt = input.parse::<i64>().unwrap();

    let mut mofile = fs::OpenOptions::new().write(true).truncate(true).open(file_name).unwrap();
    println!("{}\n{}\n",tt,state.page);
    mofile.write(String::from(format!("{}\n{}\n",tt,state.page)).as_bytes()); //TODO

    return Ok(0);
}