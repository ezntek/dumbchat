use chrono::{DateTime, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Uid {
    pub id: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub id: Uid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message<Tz: TimeZone> {
    pub text: String,
    pub sent_by: User,
    pub time_sent: DateTime<Tz>,
}

impl Uid {
    pub fn with_id(id: u16) -> Uid {
        Uid { id }
    }
}

impl Display for Uid {}

impl ToString for Uid {
    fn to_string(&self) -> String {
        self.id.to_string()
    }
}


impl<Tz> Message<Tz>
where
    Tz: TimeZone,
{
    pub fn new(text: String, sent_by: User, time_sent: DateTime<Tz>) -> Self {
        Message {
            text,
            sent_by,
            time_sent,
        }
    }
}
