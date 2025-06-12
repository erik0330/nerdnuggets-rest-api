use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use utils::env::Env;

pub struct DatabasePool {
    pool: Pool<Postgres>,
}

impl DatabasePool {
    pub async fn init(env: &Env) -> Result<Self, Error> {
        let pool = match PgPoolOptions::new()
            .max_connections(env.database_max_connections)
            .connect(&env.database_url)
            .await
        {
            Ok(pool) => {
                println!("->> ✅Connection to the database is successful!\n");
                pool
            }
            Err(err) => {
                println!("->> 🔥 Failed to connect to the database: {:?}\n", err);
                std::process::exit(1);
            }
        };

        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}
