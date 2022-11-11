use std::time::{SystemTime, Duration};

use serde::{Deserialize,Serialize};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct TotpParameters{
    pub service_name: String,
    pub pretty_name: String,
    pub algorithm: String,
    pub reftime: SystemTime,
    pub period: Duration,
    pub secret: Vec<u8>,
    pub digits: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Note{
    pub title: String,
    pub content: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum StoreRow{
    Totp(TotpParameters),
    Note(Note)
} 

