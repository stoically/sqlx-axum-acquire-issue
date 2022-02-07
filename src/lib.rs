use axum::{routing::get, Router};
use sqlx::{Acquire, PgPool, Postgres};

#[tokio::main]
async fn main() -> Result<(), sqlx::error::BoxDynError> {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    let app = Router::new().route(
        "/",
        get(|| async move {
            let pool = pool.clone();
            let mut transaction = pool.begin().await.unwrap();
            query(&mut transaction).await.unwrap();
        }),
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn query<'a, A>(conn: A) -> Result<(), sqlx::error::BoxDynError>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut conn = conn.acquire().await?;

    sqlx::query!("SELECT 1 as v").fetch_one(&mut *conn).await?;
    sqlx::query!("SELECT 2 as v").fetch_one(&mut *conn).await?;

    Ok(())
}
