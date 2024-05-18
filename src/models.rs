use serde::{Deserialize, Serialize};
use uuod::Uuid;

#[derive(Serialize, Deserialize)]

pub struct User {
    pub id: Uuid,
    pub long_url: String,
    pub short_url: String,
}