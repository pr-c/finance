use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Book {
    currencies: Vec<Currency>,
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize)]
pub struct Currency {
    name: String,
    decimals: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    name: String,
    account_type: AccountType,
}

#[derive(Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Equity,
    Expense,
    Income,
    Liability,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    time: DateTime<Utc>,
    postings: Vec<Posting>,
}

#[derive(Serialize, Deserialize)]
pub struct Posting {
    acc: String,
    currency: String,
    amount: u64,
}
