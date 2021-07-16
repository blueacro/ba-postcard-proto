#![no_std]

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TimeIsNow {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SetTime {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Command {
    SetTime(SetTime),
    QueryTime,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Response {
    TimeIsNow(TimeIsNow)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
