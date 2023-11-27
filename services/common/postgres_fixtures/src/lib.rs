use sqlx::{postgres::PgConnectOptions, PgPool};
use std::{path::Path, sync::Arc};

pub async fn set_db(pg_pool: &PgPool, url: &str, db_name: &str, table_migrations_path: &Path) {
    // let pool = PgPool::connect(&url)
    //     .await
    //     .expect("Failed to connect to the database");
    // // Create Database
    // sqlx::query(r#"CREATE DATABASE $1;"#)
    //     .bind(db_name)
    //     .execute(&pool)
    //     .await
    //     .expect("Failed to create database");

    // drop(pool);
    // let pool = PgPool::connect(&format!("{url}/{db_name}"))
    //     .await
    //     .expect("Failed to connect to the database");

    // Read the contents of the migrations directory
    let entries =
        std::fs::read_dir(table_migrations_path).expect("Failed to read migrations directory");

    for entry in entries {
        let entry = entry.expect("Error reading entry in migrations directory");
        let path = entry.path();

        // Check if the entry is a file (not a subdirectory)
        if path.is_file() {
            // Run migration for each file in the directory
            let migration_content =
                std::fs::read_to_string(&path).expect("Failed to read migration file");

            sqlx::query(&migration_content)
                .execute(pg_pool)
                .await
                .unwrap();
        }
    }
}

// #[async_trait]
// pub trait Fixture {
//
// }

// pub async fn apply_fixtures<T: Fixture>
