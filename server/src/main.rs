mod db;
mod model;
mod schema;

use axum::extract::{FromRequestParts, Path, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post};
use axum::{async_trait, Json, Router};
use axum_auth::AuthBearer;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::DatabaseErrorKind;
use model::*;
use schema::*;
use std::error::Error;
use std::net::SocketAddr;

type ConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

sql_function!(
    fn last_insert_id() -> Unsigned<Integer>
);

fn get_connection(
    pool: &ConnectionPool,
) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, Response> {
    pool.get()
        .or(Err((StatusCode::INTERNAL_SERVER_ERROR).into_response()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let pool = db::get_connection_pool();

    let router = Router::new()
        .route("/", get(root))
        .route("/book/", post(create_book))
        .route("/book/:book_name", delete(delete_book).get(get_book))
        .route("/book/:book_name/currency/", post(create_currency))
        .route(
            "/book/:book_name/currency/:currency_symbol",
            get(get_currency).delete(delete_currency),
        )
        .route("/book/:book_name/account/", post(create_account))
        .route(
            "/book/:book_name/account/:account_name",
            delete(delete_account).get(get_account),
        )
        .route("/book/:book_name/transaction", post(create_transaction))
        .route("/book/:book_name/transaction/:transaction_id", delete(delete_transaction))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
    Ok(())
}

struct Claim {
    user: User,
}

#[async_trait]
impl FromRequestParts<ConnectionPool> for Claim {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        pool: &ConnectionPool,
    ) -> Result<Self, Self::Rejection> {
        if let Ok(token) = AuthBearer::from_request_parts(parts, pool).await {
            let conn = &mut get_connection(pool)?;
            let mut result = users::table
                .filter(users::dsl::bearer.eq(&token.0))
                .load::<User>(conn)
                .expect("Error loading Users");
            return if result.len() == 1 {
                let claim = Claim {
                    user: result.remove(0),
                };
                Ok(claim)
            } else {
                Err((StatusCode::FORBIDDEN, "Authentication failed.").into_response())
            };
        }
        Err((StatusCode::FORBIDDEN, "Not authenticated.").into_response())
    }
}

async fn root(claim: Claim) -> impl IntoResponse {
    format!("Hello {}!", claim.user.name)
}

async fn create_book(
    claim: Claim,
    State(pool): State<ConnectionPool>,
    Json(user_book): Json<finance_lib::Book>,
) -> Result<Response, Response> {
    let conn = &mut get_connection(&pool)?;
    let book = Book::from_user_struct(
        &user_book,
        AddedInformationForBook {
            user_name: &claim.user.name,
        },
    );
    let result = diesel::insert_into(books::table)
        .values(&book)
        .execute(conn);
    if let Err(e) = result {
        if let diesel::result::Error::DatabaseError(database_error, _) = e {
            if let DatabaseErrorKind::UniqueViolation = database_error {
                return Err((
                    StatusCode::CONFLICT,
                    format!("Book '{}' already exists.", book.name),
                )
                    .into_response());
            }
        }
        Err((StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response())
    } else {
        Ok(format!("Book '{}' created.", book.name).into_response())
    }
}

async fn delete_book(
    claim: Claim,
    Path(book_name): Path<String>,
    State(pool): State<ConnectionPool>,
) -> Result<Response, Response> {
    let conn = &mut get_connection(&pool)?;
    let result = diesel::delete(books::table)
        .filter(
            books::dsl::name
                .eq(&book_name)
                .and(books::dsl::user_name.eq(&claim.user.name)),
        )
        .execute(conn);
    if let Ok(amount) = result {
        if amount > 0 {
            Ok(format!("Book {} deleted", &book_name).into_response())
        } else {
            Err((
                StatusCode::NOT_FOUND,
                format!("Book {} does not exist.", &book_name),
            )
                .into_response())
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response())
    }
}

async fn get_book(
    claim: Claim,
    Path(book_name): Path<String>,
    State(pool): State<ConnectionPool>,
) -> Result<Response, Response> {
    let conn = &mut get_connection(&pool)?;
    let result = books::table
        .filter(
            books::dsl::name
                .eq(&book_name)
                .and(books::dsl::user_name.eq(&claim.user.name)),
        )
        .load::<Book>(conn);
    if let Ok(queried) = result {
        if queried.len() == 1 {
            Ok(Json(queried[0].to_user_struct()).into_response())
        } else {
            Err((StatusCode::NOT_FOUND).into_response())
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR).into_response())
    }
}

async fn create_currency(
    claim: Claim,
    State(pool): State<ConnectionPool>,
    Path(book_name): Path<String>,
    Json(user_currency): Json<finance_lib::Currency>,
) -> Result<Response, Response> {
    let server_currency = Currency::from_user_struct(
        &user_currency,
        UserAndBookInfo {
            user_name: &claim.user.name,
            book_name: &book_name,
        },
    );
    let conn = &mut get_connection(&pool)?;
    let result = diesel::insert_into(currencies::table)
        .values(server_currency)
        .execute(conn);
    if let Err(e) = result {
        if let diesel::result::Error::DatabaseError(database_error_kind, _) = e {
            match database_error_kind {
                DatabaseErrorKind::ForeignKeyViolation => {
                    return Err((StatusCode::BAD_REQUEST, "Book does not exist.").into_response())
                }
                DatabaseErrorKind::UniqueViolation => {
                    return Err((StatusCode::CONFLICT, "Currency alread exists.").into_response())
                }
                _ => {}
            }
        }
        Err((StatusCode::INTERNAL_SERVER_ERROR).into_response())
    } else {
        Ok(().into_response())
    }
}

async fn get_currency(
    claim: Claim,
    Path((book_name, currency_symbol)): Path<(String, String)>,
    State(pool): State<ConnectionPool>,
) -> Result<Response, Response> {
    let conn = &mut get_connection(&pool)?;
    let result = currencies::table
        .filter(
            currencies::dsl::symbol.eq(&currency_symbol).and(
                currencies::dsl::book_name
                    .eq(&book_name)
                    .and(currencies::dsl::user_name.eq(&claim.user.name)),
            ),
        )
        .load::<Currency>(conn);
    if let Ok(list) = result {
        if list.len() == 1 {
            let currency = &list[0];
            Ok(Json(currency.to_user_struct()).into_response())
        } else {
            Err((StatusCode::NOT_FOUND).into_response())
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR).into_response())
    }
}

async fn delete_currency(
    claim: Claim,
    Path((book_name, currency_symbol)): Path<(String, String)>,
    State(pool): State<ConnectionPool>,
) -> Result<Response, Response> {
    let conn = &mut get_connection(&pool)?;
    let result = diesel::delete(currencies::table)
        .filter(
            currencies::dsl::user_name.eq(&claim.user.name).and(
                currencies::dsl::book_name
                    .eq(&book_name)
                    .and(currencies::dsl::symbol.eq(&currency_symbol)),
            ),
        )
        .execute(conn);
    if let Ok(deleted_amount) = result {
        if deleted_amount >= 1 {
            Ok(().into_response())
        } else {
            Err((StatusCode::NOT_FOUND).into_response())
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR).into_response())
    }
}

async fn create_account(
    claim: Claim,
    Path(book_name): Path<String>,
    State(pool): State<ConnectionPool>,
    Json(user_account): Json<finance_lib::Account>,
) -> Result<Response, Response> {
    let conn = &mut get_connection(&pool)?;
    let account = Account::from_user_struct(
        &user_account,
        UserAndBookInfo {
            book_name: &book_name,
            user_name: &claim.user.name,
        },
    );
    let result = diesel::insert_into(accounts::table)
        .values(&account)
        .execute(conn);
    match result {
        Ok(1) => Ok(().into_response()),
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Err((StatusCode::CONFLICT).into_response())
        }
        _ => Err((StatusCode::INTERNAL_SERVER_ERROR).into_response()),
    }
}
async fn delete_account(
    claim: Claim,
    Path((book_name, account_name)): Path<(String, String)>,
    State(pool): State<ConnectionPool>,
) -> Result<Response, Response> {
    let conn = &mut get_connection(&pool)?;
    let result = diesel::delete(accounts::table)
        .filter(
            accounts::dsl::user_name.eq(claim.user.name).and(
                accounts::dsl::name
                    .eq(account_name)
                    .and(accounts::dsl::book_name.eq(book_name)),
            ),
        )
        .execute(conn);
    match result {
        Ok(1) => Ok(().into_response()),
        Ok(_) => Err((StatusCode::NOT_FOUND).into_response()),
        _ => Err((StatusCode::INTERNAL_SERVER_ERROR).into_response()),
    }
}

async fn get_account(
    claim: Claim,
    Path((book_name, account_name)): Path<(String, String)>,
    State(pool): State<ConnectionPool>,
) -> Result<Response, Response> {
    let conn = &mut get_connection(&pool)?;
    let result = accounts::table
        .filter(
            accounts::dsl::user_name.eq(claim.user.name).and(
                accounts::dsl::name
                    .eq(account_name)
                    .and(accounts::dsl::book_name.eq(book_name)),
            ),
        )
        .load::<Account>(conn);

    match result {
        Ok(accounts) => {
            if accounts.len() > 0 {
                Ok(Json(accounts[0].to_user_struct()).into_response())
            } else {
                Err((StatusCode::NOT_FOUND).into_response())
            }
        }
        _ => Err((StatusCode::INTERNAL_SERVER_ERROR).into_response()),
    }
}

async fn create_transaction(
    claim: Claim,
    Path(book_name): Path<String>,
    State(pool): State<ConnectionPool>,
    Json(user_transaction): Json<finance_lib::NewTransaction>,
) -> Result<Response, Response> {
    let mut conn = get_connection(&pool)?;
    let transaction = NewTransaction::from_user_struct(
        &user_transaction,
        UserAndBookInfo {
            book_name: &book_name,
            user_name: &claim.user.name,
        },
    );
    let result = diesel::insert_into(transactions::table)
        .values(&transaction)
        .execute(&mut conn);
    match result {
        Ok(1) => {
            let id_result = diesel::select(last_insert_id()).get_result::<u32>(&mut conn);
            match id_result {
                Ok(id) => Ok((Json(id)).into_response()),
                _ => Err((StatusCode::IM_A_TEAPOT).into_response()),
            }
        }
        Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, e)) => {
            Err((StatusCode::NOT_FOUND, e.message().to_string()).into_response())
        }
        _ => Err((StatusCode::INTERNAL_SERVER_ERROR).into_response()),
    }
}

async fn delete_transaction(
    claim: Claim,
    Path((book_name, transaction_id)): Path<(String, u32)>,
    State(pool): State<ConnectionPool>,
) -> Result<Response, Response> {
    let mut conn = get_connection(&pool)?;
    let result = diesel::delete(transactions::table).filter(
        transactions::dsl::user_name.eq(claim.user.name).and(
            transactions::dsl::book_name
                .eq(book_name)
                .and(transactions::dsl::id.eq(transaction_id)),
        ),
    ).execute(&mut conn);
    match result {
        Ok(1) => Ok(().into_response()),
        Ok(_) => Err((StatusCode::NOT_FOUND).into_response()),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR).into_response())
    }
}
