use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub symbol: String,
    pub description: Option<String>,
    pub decimal_points: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub id: u32,
    pub time: Option<NaiveDateTime>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewTransaction {
    pub description: Option<String>,
    pub time: Option<NaiveDateTime>,
}
