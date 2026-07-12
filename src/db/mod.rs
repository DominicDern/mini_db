pub mod connection;
pub mod container;
pub mod id;
pub mod mini;
pub mod terrain;

use iced::window;
use sqlx::{SqlitePool, query};

use mini::Mini;

use terrain::Terrain;

use crate::db::container::Container;

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

#[derive(Debug, Clone)]
pub struct ContainerContents {
    pub container: Container,
    pub child_containers: Vec<Container>,
    pub minis: Vec<Mini>,
    pub terrain: Vec<Terrain>,
}

pub async fn get_container_contents(
    pool: &SqlitePool,
    container_id: i64,
) -> Result<ContainerContents, sqlx::Error> {
    let container = sqlx::query_as!(
        Container,
        "SELECT id, name, parent_id FROM containers WHERE id = ?",
        container_id
    )
    .fetch_one(pool)
    .await?;

    let child_containers = sqlx::query_as!(
        Container,
        "SELECT id, name, parent_id FROM containers WHERE parent_id = ?",
        container_id
    )
    .fetch_all(pool)
    .await?;

    let minis = sqlx::query_as!(
        Mini,
        r#"SELECT m.id, m.name, m.file_location,
        m.number_printed as "number_printed: u16",
        m.base_size as "base_size: u16"
        FROM minis m
        JOIN mini_locations ml ON ml.mini_id = m.id
        WHERE ml.container_id = ?"#,
        container_id
    )
    .fetch_all(pool)
    .await?;

    let terrain = sqlx::query_as!(
        Terrain,
        r#"SELECT t.id, t.name, t.file_location,
        t.number_printed as "number_printed: u16"
        FROM terrains t
        JOIN terrain_locations tl ON tl.terrain_id = t.id
        WHERE tl.container_id = ?"#,
        container_id
    )
    .fetch_all(pool)
    .await?;

    Ok(ContainerContents {
        container,
        child_containers,
        minis,
        terrain,
    })
}

pub async fn insert_container(
    pool: &SqlitePool,
    name: String,
    parent_id: Option<i64>,
) -> Result<Container, sqlx::Error> {
    let id = sqlx::query!(
        "INSERT INTO containers (name, parent_id) VALUES (?, ?)",
        name,
        parent_id
    )
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(Container {
        id,
        name,
        parent_id,
    })
}

pub async fn get_root_containers(pool: &SqlitePool) -> Result<Vec<Container>, sqlx::Error> {
    sqlx::query_as!(
        Container,
        "SELECT id, name, parent_id FROM containers WHERE parent_id IS NULL"
    )
    .fetch_all(pool)
    .await
}

/// Fetches every container in the DB in one shot. Pair this with
/// `container::build_forest` to reconstruct the full tree client-side.
pub async fn get_all_containers(pool: &SqlitePool) -> Result<Vec<Container>, sqlx::Error> {
    sqlx::query_as!(Container, "SELECT id, name, parent_id FROM containers")
        .fetch_all(pool)
        .await
}

/// Deletes a single container row. Note: this does not cascade — if the
/// container still has child containers, minis, or terrain referencing it,
/// the delete will fail with a foreign key error rather than silently
/// orphaning or cascading. Remove/move descendants first if needed.
pub async fn remove_container(pool: &SqlitePool, container_id: i64) -> Result<u64, sqlx::Error> {
    let affected_rows = sqlx::query!("DELETE FROM containers WHERE id = ?", container_id)
        .execute(pool)
        .await?
        .rows_affected();
    Ok(affected_rows)
}

pub async fn add_mini_to_container(
    pool: &SqlitePool,
    mini_id: i64,
    container_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT OR IGNORE INTO mini_locations (mini_id, container_id) VALUES (?, ?)",
        mini_id,
        container_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn add_terrain_to_container(
    pool: &SqlitePool,
    terrain_id: i64,
    container_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT OR IGNORE INTO terrain_locations (terrain_id, container_id) VALUES (?, ?)",
        terrain_id,
        container_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
