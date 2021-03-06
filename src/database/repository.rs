use super::{
    dotenv, env, info, lazy_static, ConnectionManager, Pool, PooledConnection, SqliteConnection,
    CONFIG,
};

lazy_static! {
    pub static ref DB: Pool<ConnectionManager<SqliteConnection>> = establish_connection();
}

fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("[DB] DATABASE_URL must be set");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url.to_owned());
    let pool = Pool::builder()
        .max_size(*CONFIG.db_pool())
        .build(manager)
        .expect("[DB] Failed to create pool. Check your db settings");

    info!("[DB] Connection to database established");

    pool
}

pub(crate) fn db_conn() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    DB.get().unwrap()
}
