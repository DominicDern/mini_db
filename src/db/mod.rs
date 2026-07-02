pub mod connection;
pub mod container;
pub mod id;
pub mod mini;
pub mod terrain;

use sqlx::{SqlitePool, query};

use mini::Mini;

use terrain::Terrain;

pub async fn insert_mini(
    pool: &SqlitePool,
    name: String,
    file_location: Option<String>,
    base_size: u16,
) -> Result<Mini, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO minis (name, file_location, number_printed, base_size) VALUES (?, ?, 0, ?)",
    )
    .bind(&name)
    .bind(&file_location)
    .bind(base_size)
    .execute(pool)
    .await?;

    Ok(Mini {
        id: result.last_insert_rowid(),
        name,
        file_location,
        number_printed: 0,
        base_size,
    })
}

pub async fn remove_all_matching_minis(pool: &SqlitePool, name: &str) -> Result<u64, sqlx::Error> {
    let affected_rows = query!("DELETE FROM minis WHERE name = ?", name)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(affected_rows)
}

pub async fn get_all_minis(pool: &SqlitePool) -> Result<Vec<Mini>, sqlx::Error> {
    let minis = sqlx::query_as("SELECT * FROM minis")
        .fetch_all(pool)
        .await?;
    Ok(minis)
}

pub async fn insert_terrain(
    pool: &SqlitePool,
    name: String,
    file_location: Option<String>,
) -> Result<Terrain, sqlx::Error> {
    let result =
        sqlx::query("INSERT INTO terrains (name, file_location, number_printed) VALUES (?, ?, 0)")
            .bind(&name)
            .execute(pool)
            .await?;

    Ok(Terrain {
        id: result.last_insert_rowid(),
        name,
        file_location,
        number_printed: 0,
    })
}

pub async fn remove_all_matching_terrain(
    pool: &SqlitePool,
    name: &str,
) -> Result<u64, sqlx::Error> {
    let affected_rows = query!("DELETE FROM terrains WHERE name = ?", name)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(affected_rows)
}

pub async fn get_all_terrain(pool: &SqlitePool) -> Result<Vec<Terrain>, sqlx::Error> {
    let terrain = sqlx::query_as("SELECT * FROM terrains")
        .fetch_all(pool)
        .await?;
    Ok(terrain)
}
