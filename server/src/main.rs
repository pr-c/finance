mod db;
mod model;
mod schema;

use axum::extract::{FromRequestParts, Path, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{async_trait, Router};
use axum_auth::AuthBearer;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::DatabaseErrorKind;
use model::*;
use schema::*;
use std::error::Error;
use std::net::SocketAddr;

type ConnectionPool = Pool<ConnectionManager<MysqlConnection>>;

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
        .route("/books/:book_name", post(create_book).delete(delete_book))
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
    Path(book_name): Path<String>,
    State(pool): State<ConnectionPool>,
) -> Result<Response, Response> {
    let conn = &mut get_connection(&pool)?;
    let book = Book {
        name: book_name.clone(),
        user_name: claim.user.name,
    };
    let result = diesel::insert_into(books::table)
        .values(&book)
        .execute(conn);
    if let Err(e) = result {
        if let diesel::result::Error::DatabaseError(database_error, _) = e {
            if let DatabaseErrorKind::UniqueViolation = database_error {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Book '{}' already exists.", book_name),
                )
                    .into_response());
            }
        }
        Err((StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response())
    } else {
        Ok(format!("Book '{}' created.", book_name).into_response())
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
                .and(books::dsl::user_name.eq(claim.user.name)),
        )
        .execute(conn);
    if let Ok(amount) = result {
        if amount > 0 {
            Ok(format!("Book {} deleted", &book_name).into_response())
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                format!("Book {} does not exist.", &book_name),
            )
                .into_response())
        }
    } else {
        Err((StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response())
    }
}
