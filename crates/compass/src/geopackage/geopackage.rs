use geozero::{ToWkt, wkb};
use sqlx::query;
use tracing::{debug, trace};

use crate::Result;

pub(super) async fn init_geopackage<P: AsRef<std::path::Path>>(path: P) -> Result<()> {
    trace!("Initializing a new GeoPackage at {:?}", path.as_ref());

    //async fn init_geopackage() -> Result<()> {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
        // .filename("my_sqlite_database.db")
        .create_if_missing(true);

    let pool = sqlx::SqlitePool::connect_with(options).await?;

    // Application_id for GeoPackage 0x47504B47
    query("PRAGMA application_id = 0x47504B47;")
        .execute(&pool)
        .await?;
    // User_version for GeoPackage 1.3.0 is 10300
    query("PRAGMA user_version = 10300;").execute(&pool).await?;

    trace!("Creating gpkg_spatial_ref_sys table (required)");
    query(
        r#"
        CREATE TABLE IF NOT EXISTS gpkg_spatial_ref_sys (
          srs_name TEXT NOT NULL,
          srs_id INTEGER NOT NULL PRIMARY KEY,
          organization TEXT NOT NULL,
          organization_coordsys_id INTEGER NOT NULL,
          definition  TEXT NOT NULL,
          description TEXT
          );
        "#,
    )
    .execute(&pool)
    .await?;

    trace!("Creating gpkg_contents table (mandatory)");
    query(
        r#"
        CREATE TABLE IF NOT EXISTS gpkg_contents (

          table_name TEXT NOT NULL PRIMARY KEY,
          data_type TEXT NOT NULL,
          identifier TEXT UNIQUE,
          description TEXT DEFAULT '',
          last_change DATETIME NOT NULL
            DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
          min_x DOUBLE,
          min_y DOUBLE,
          max_x DOUBLE,
          max_y DOUBLE,
          srs_id INTEGER,
          CONSTRAINT
            fk_gc_r_srs_id FOREIGN KEY (srs_id)
              REFERENCES gpkg_spatial_ref_sys(srs_id)
          );
        "#,
    )
    .execute(&pool)
    .await?;

    trace!("Creating gpkg_geometry_columns table");
    query(
        r#"
      CREATE TABLE IF NOT EXISTS gpkg_geometry_columns (
        table_name TEXT NOT NULL,
        column_name TEXT NOT NULL,
        geometry_type_name TEXT NOT NULL,
        srs_id INTEGER NOT NULL,
        z TINYINT NOT NULL,
        m TINYINT NOT NULL,
        CONSTRAINT pk_geom_cols PRIMARY KEY (table_name, column_name),
        CONSTRAINT uk_gc_table_name UNIQUE (table_name),
        CONSTRAINT fk_gc_tn FOREIGN KEY (table_name) REFERENCES gpkg_contents(table_name),
        CONSTRAINT fk_gc_srs FOREIGN KEY (srs_id) REFERENCES gpkg_spatial_ref_sys (srs_id)
        );
        "#,
    )
    .execute(&pool)
    .await?;

    trace!("Creating gpkg_extensions table");
    query(
        r#"
      CREATE TABLE IF NOT EXISTS gpkg_extensions (
        table_name TEXT,
        column_name TEXT,
        extension_name TEXT NOT NULL,
        definition TEXT NOT NULL,
        scope TEXT NOT NULL,
        CONSTRAINT ge_tce UNIQUE (table_name, column_name, extension_name)
        );
        "#,
    )
    .execute(&pool)
    .await?;

    trace!("Creating ord_areas table");
    query(
        r#"
      CREATE TABLE IF NOT EXISTS ord_areas (
        fid INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        geom GEOMETRY,
        state_name TEXT,
        state_fips INTEGER,
        county_name TEXT,
        county_fips TEXT,
        subd_name TEXT,
        subd_fips TEXT
        );
        "#,
    )
    .execute(&pool)
    .await?;

    query("PRAGMA integrity_check").execute(&pool).await?; // -> ok
    query("PRAGMA foreign_key_check").execute(&pool).await?; // -> empty

    Ok(())
}
