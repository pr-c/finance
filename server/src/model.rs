use crate::schema::*;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use std::error::Error;

#[derive(Queryable)]
pub struct User {
    pub name: String,
    pub bearer: Option<String>,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = books)]
pub struct Book {
    pub name: String,
    pub user_name: String,
    pub description: Option<String>,
}

impl ToUserStruct for Book {
    type UserStruct = finance_lib::Book;

    fn to_user_struct(&self) -> Self::UserStruct {
        finance_lib::Book {
            name: self.name.clone(),
            description: self.description.clone(),
        }
    }
}

pub struct AddedInformationForBook<'a> {
    pub user_name: &'a String,
}

impl<'a> FromUserStruct<'a> for Book {
    type UserStruct = finance_lib::Book;
    type AddedInformation = AddedInformationForBook<'a>;
    fn from_user_struct(
        user_struct: &Self::UserStruct,
        added_information: Self::AddedInformation,
    ) -> Self {
        Self {
            name: user_struct.name.clone(),
            user_name: added_information.user_name.clone(),
            description: user_struct.description.clone(),
        }
    }
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = currencies)]
pub struct Currency {
    symbol: String,
    decimal_points: i32,
    description: Option<String>,
    user_name: String,
    book_name: String,
}

impl ToUserStruct for Currency {
    type UserStruct = finance_lib::Currency;
    fn to_user_struct(&self) -> Self::UserStruct {
        finance_lib::Currency {
            symbol: self.symbol.clone(),
            description: self.description.clone(),
            decimal_points: self.decimal_points,
        }
    }
}

pub struct UserAndBookInfo<'a> {
    pub user_name: &'a String,
    pub book_name: &'a String,
}

impl<'a> FromUserStruct<'a> for Currency {
    type UserStruct = finance_lib::Currency;
    type AddedInformation = UserAndBookInfo<'a>;

    fn from_user_struct(
        user_struct: &finance_lib::Currency,
        added_information: Self::AddedInformation,
    ) -> Self {
        Self {
            user_name: added_information.user_name.clone(),
            book_name: added_information.book_name.clone(),
            decimal_points: user_struct.decimal_points,
            description: user_struct.description.clone(),
            symbol: user_struct.symbol.clone(),
        }
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub name: String,
    pub description: Option<String>,
    pub user_name: String,
    pub book_name: String,
}

impl ToUserStruct for Account {
    type UserStruct = finance_lib::Account;
    fn to_user_struct(&self) -> Self::UserStruct {
        Self::UserStruct {
            description: self.description.clone(),
            name: self.name.clone(),
        }
    }
}

impl<'a> FromUserStruct<'a> for Account {
    type AddedInformation = UserAndBookInfo<'a>;
    type UserStruct = finance_lib::Account;
    fn from_user_struct(
        user_struct: &Self::UserStruct,
        added_information: Self::AddedInformation,
    ) -> Self {
        Self {
            name: user_struct.name.clone(),
            description: user_struct.description.clone(),
            book_name: added_information.book_name.clone(),
            user_name: added_information.user_name.clone(),
        }
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i64,
    time: NaiveDateTime,
    pub description: Option<String>,
    pub book_name: String,
    pub user_name: String,
}

impl ToUserStruct for Transaction {
    type UserStruct = finance_lib::Transaction;
    fn to_user_struct(&self) -> Self::UserStruct {
        Self::UserStruct {
            id: self.id,
            time: Some(self.time),
            description: self.description.clone(),
        }
    }
}

impl<'a> FromUserStruct<'a> for Transaction {
    type AddedInformation = UserAndBookInfo<'a>;
    type UserStruct = finance_lib::Transaction;

    fn from_user_struct(
        user_struct: &Self::UserStruct,
        added_information: Self::AddedInformation,
    ) -> Self {
        Self {
            id: user_struct.id,
            time: user_struct.time.unwrap_or_else(|| Utc::now().naive_utc()),
            description: user_struct.description.clone(),
            book_name: added_information.book_name.clone(),
            user_name: added_information.user_name.clone(),
        }
    }
}

impl<'a> FromNewUserStruct<'a> for Transaction {
    type AddedInformation = UserAndBookInfo<'a>;
    type NewUserStruct = finance_lib::NewTransaction;

    fn from_new_user_struct(
        new_user_struct: &Self::NewUserStruct,
        added_information: Self::AddedInformation,
    ) -> Result<Self, Box<dyn Error>> {
        let id = crate::SNOWFLAKE_GENERATOR.lock()?.real_time_generate();
        Ok(Self {
            id,
            time: new_user_struct
                .time
                .unwrap_or_else(|| Utc::now().naive_utc()),
            description: new_user_struct.description.clone(),
            book_name: added_information.book_name.clone(),
            user_name: added_information.user_name.clone(),
        })
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = postings)]
pub struct Posting {
    pub id: i64,
    pub transaction_id: i64,
    pub valuta: Option<NaiveDateTime>,
    pub book_name: String,
    pub user_name: String,
    pub account_name: String,
    pub currency: String,
    pub amount: i32,
}

pub struct AddedInformationForPosting<'a> {
    pub user_name: &'a String,
    pub book_name: &'a String,
    pub transaction_id: &'a i64,
}

impl<'a> FromUserStruct<'a> for Posting {
    type AddedInformation = AddedInformationForPosting<'a>;
    type UserStruct = finance_lib::Posting;
    fn from_user_struct(
        user_struct: &Self::UserStruct,
        added_information: Self::AddedInformation,
    ) -> Self {
        Self {
            account_name: user_struct.account_name.clone(),
            book_name: added_information.book_name.clone(),
            amount: user_struct.amount,
            currency: user_struct.currency.clone(),
            id: user_struct.id,
            transaction_id: *added_information.transaction_id,
            user_name: added_information.user_name.clone(),
            valuta: user_struct.valuta,
        }
    }
}

impl<'a> FromNewUserStruct<'a> for Posting {
    type AddedInformation = AddedInformationForPosting<'a>;
    type NewUserStruct = finance_lib::NewPosting;

    fn from_new_user_struct(
        new_user_struct: &Self::NewUserStruct,
        added_information: Self::AddedInformation,
    ) -> Result<Self, Box<dyn Error>> {
        let id = crate::SNOWFLAKE_GENERATOR.lock()?.real_time_generate();
        Ok(Self {
            id,
            amount: new_user_struct.amount,
            account_name: new_user_struct.account_name.clone(),
            currency: new_user_struct.currency.clone(),
            transaction_id: *added_information.transaction_id,
            valuta: new_user_struct.valuta,
            book_name: added_information.book_name.clone(),
            user_name: added_information.user_name.clone(),
        })
    }
}

impl ToUserStruct for Posting {
    type UserStruct = finance_lib::Posting;
    fn to_user_struct(&self) -> Self::UserStruct {
        Self::UserStruct {
            account_name: self.account_name.clone(),
            amount: self.amount,
            currency: self.currency.clone(),
            id: self.id,
            valuta: self.valuta,
        }
    }
}

pub trait ToUserStruct {
    type UserStruct;

    fn to_user_struct(&self) -> Self::UserStruct;
}

pub trait FromUserStruct<'a> {
    type UserStruct;
    type AddedInformation;

    fn from_user_struct(
        user_struct: &Self::UserStruct,
        added_information: Self::AddedInformation,
    ) -> Self;
}

pub trait FromNewUserStruct<'a>
where
    Self: Sized,
{
    type NewUserStruct;
    type AddedInformation;

    fn from_new_user_struct(
        new_user_struct: &Self::NewUserStruct,
        added_information: Self::AddedInformation,
    ) -> Result<Self, Box<dyn Error>>;
}
