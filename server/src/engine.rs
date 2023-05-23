use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use finance_lib::Book;

pub static engine: Lazy<Mutex<Engine>> = Lazy::new(|| {
    Mutex::new(Engine::new())
});

#[derive(Serialize, Deserialize, Default)]
pub struct Engine {

}

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    books: HashMap<String, Book>
}

impl Engine {
    fn new() -> Self {
        let path = Path::new("finance.toml");
        if !path.exists() {
            Self::default()
        } else {
            let mut content = String::new();
            File::open(path).unwrap().read_to_string(&mut content).unwrap();
            toml::from_str::<Self>(&content).unwrap()
        }
    }

    pub fn authenticate(&self, user_name: &str, bearer_token: &String) -> Result<(), Box<dyn Error>> {
        Err("".into())
    }
}