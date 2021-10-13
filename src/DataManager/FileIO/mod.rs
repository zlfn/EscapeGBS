use std::fs::File;
use std::io::prelude::*;
use rocket::response::content;
use std::{fs, io};
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Utc;

pub struct GameState
{
    test:bool
}

///입력된 주소에서 파일을 찾아 읽어서 String으로 반환합니다.
///
/// # Example
/// ```
/// let mut s = DataManager::FileIO::ReadFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadFile(path: &str) -> Result<String, io::Error>
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(String::from(contents));
}

///입력된 주소에서 HTML파일을 찾아 읽어서 content::Html로 반환합니다.
///
/// # Example
/// ```
/// return DataManager::FileIO::ReadHTMLFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadHTMLFile(path: &str) -> Result<content::Html<String>, io::Error>
{
    let s = ReadFile(path)?;
    return Ok(content::Html(s));
}

///입력된 주소에서 CSS 파일을 찾아 읽어서 content::Html로 반환합니다.
///
/// # Example
/// ```
/// return DataManager::FileIO::ReadCSSFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadCSSFile(path: &str) -> Result<content::Css<String>, io::Error>
{
    let s = ReadFile(path)?;
    return Ok(content::Css(s));
}

///입력된 주소에서 JS 파일을 찾아 읽어서 content::Html로 반환합니다.
///
/// # Example
/// ```
/// return DataManager::FileIO::ReadJSFile("HTML/index/index.html").unwrap();
/// ```
///
pub fn ReadJSFile(path: &str) -> Result<content::JavaScript<String>, io::Error>
{
    let s = ReadFile(path)?;
    return Ok(content::JavaScript(s));
}

///입력된 시간(초) 후에 만료되는 세션을 생성합니다.
///
/// # Example
/// ```
/// return DataManager::FileIO::GenerateSession(3600).unwrap();
/// ```
///
pub fn GenerateSession(tt: i64) -> Result<u32,io::Error>
{
    let key = rand::random::<u32>();
    let file_name = format!("DataBase/{}",key);
    let mut file = File::create(&file_name)?;
    let now = GetUnixTime();
    let s = format!("{}\n",now+tt);
    file.write(s.as_bytes());
    println!("session {} created",key);
    return Ok(key);
}

///만료된 세션을 삭제합니다.
///
/// # Example
/// ```
/// DataManager::FileIO::ClearSession();
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
        let mut contents = String::new();
        reader.read_line(&mut contents);
        let mut tts = contents.replace("\n","");
        let tt = tts.parse::<i64>().unwrap();
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
/// DataManager::FileIO::DeleteAllSessions();
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

///현재 유닉스 시간을 64비트 정수로 반환합니다.
///
/// # Example
/// ```
/// let time = DataManager::FileIO::GetUnixTime();
/// ```
///
pub fn GetUnixTime() -> i64 {
    let now = Utc::now();
    return now.timestamp();
}