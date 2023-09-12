use std::error::Error;
//use sqlx::Connection;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error >> {
    let url = "postgres://postgres:URKEYPASS@172.17.0.1:15432/postgres";

    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let res = sqlx::query("Select 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;

    let sum: i32 = res.get("sum");
    println!("1 + 1 = {}", sum);

    Ok(())

}
