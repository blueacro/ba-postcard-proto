#![no_std]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct TimeIsNow {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct SetTime {
    seconds: u8,
    minutes: u8,
    hours: u8,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Command {
    SetTime(SetTime),
    QueryTime,
}

#[derive(Serialize, Deserialize, PartialEq)]
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
