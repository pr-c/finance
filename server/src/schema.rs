// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (name, book_name, user_name) {
        name -> Varchar,
        description -> Nullable<Varchar>,
        user_name -> Varchar,
        book_name -> Varchar,
    }
}

diesel::table! {
    books (name, user_name) {
        name -> Varchar,
        user_name -> Varchar,
    }
}

diesel::table! {
    currencies (symbol, book_name, user_name) {
        symbol -> Varchar,
        description -> Nullable<Varchar>,
        user_name -> Varchar,
        book_name -> Varchar,
    }
}

diesel::table! {
    postings (id, transaction_id, book_name, user_name) {
        id -> Integer,
        transaction_id -> Integer,
        book_name -> Varchar,
        user_name -> Varchar,
        account -> Varchar,
        currency -> Varchar,
        amount -> Integer,
    }
}

diesel::table! {
    transactions (id, book_name, user_name) {
        id -> Integer,
        book_name -> Varchar,
        user_name -> Varchar,
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (name) {
        name -> Varchar,
        bearer -> Nullable<Varchar>,
    }
}

diesel::joinable!(accounts -> users (user_name));
diesel::joinable!(books -> users (user_name));
diesel::joinable!(currencies -> users (user_name));
diesel::joinable!(postings -> users (user_name));
diesel::joinable!(transactions -> users (user_name));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    books,
    currencies,
    postings,
    transactions,
    users,
);