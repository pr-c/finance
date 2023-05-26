use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub name: String,
    pub bearer: Option<String>,
}

#[derive(Queryable, Insertable)]
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

#[derive(Queryable, Insertable)]
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

pub struct AddedInformationForCurrency<'a> {
    pub user_name: &'a String,
    pub book_name: &'a String,
}

impl<'a> FromUserStruct<'a> for Currency {
    type UserStruct = finance_lib::Currency;
    type AddedInformation = AddedInformationForCurrency<'a>;

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
    name: String,
    description: Option<String>,
    user_name: String,
    book_name: String,
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

pub struct AddedInformationForAccount<'a> {
    pub user_name: &'a String,
    pub book_name: &'a String,
}

impl<'a> FromUserStruct<'a> for Account {
    type AddedInformation = AddedInformationForAccount<'a>;
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
