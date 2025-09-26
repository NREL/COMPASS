//! Support to GeoPackage format

use tracing::{debug, trace};

use crate::Result;

pub(super) fn init_db(conn: &duckdb::Transaction) -> Result<()> {
    debug!("Initializing jurisdiction database");

    trace!("Creating gpkg_spatial_ref_sys table");
    conn.execute_batch(
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
    )?;

    trace!("Creating gpkg_contents table");
    // last_change DATETIME NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now')),
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS gpkg_contents (
          table_name TEXT NOT NULL PRIMARY KEY,
          data_type TEXT NOT NULL,
          identifier TEXT UNIQUE,
          description TEXT DEFAULT '',
          last_change TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
          min_x DOUBLE,
          min_y DOUBLE,
          max_x DOUBLE,
          max_y DOUBLE,
          srs_id INTEGER,
          CONSTRAINT fk_gc_r_srs_id
            FOREIGN KEY (srs_id) REFERENCES gpkg_spatial_ref_sys(srs_id)
        );
        "#,
    )?;

    trace!("Creating gpkg_ogr_contents table");
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS gpkg_ogr_contents (
          table_name TEXT NOT NULL PRIMARY KEY,
          feature_count INTEGER DEFAULT NULL,
        );
        "#,
    )?;

    trace!("Creating gpkg_geometry_columns table");
    conn.execute_batch(
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
    )?;

    trace!("Creating gpkg_extensions table");
    conn.execute_batch(
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
    )?;

    trace!("Creating ord_areas table");
    conn.execute_batch(
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
    )?;

    Ok(())
}
