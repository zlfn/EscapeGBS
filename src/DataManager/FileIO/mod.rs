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


