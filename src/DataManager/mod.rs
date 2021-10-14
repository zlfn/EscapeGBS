use std::fs::File;
use std::io::BufReader;
use chrono::Utc;

pub mod FileIO;
pub mod SessionIO;

pub struct GameState
{
    pub page:i32
    //TODO
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