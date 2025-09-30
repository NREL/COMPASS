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

    // table_name shall contain the name of a sqlite table
    trace!("Creating gpkg_contents table (required)");
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

    // 1.3.0, sections 1.1.2, required content
    trace!("Inserting required records into gpkg_spatial_ref_sys");
    query(
    r#"
      INSERT
        INTO gpkg_spatial_ref_sys (
          srs_name, srs_id, organization,
          organization_coordsys_id, definition, description
          )
        VALUES (
          'Undefined Cartesian SRS', -1, 'NONE', -1,
          'undefined',
          'undefined Cartesian coordinate reference system'
          );

        INSERT
          INTO gpkg_spatial_ref_sys (
            srs_name, srs_id, organization,
            organization_coordsys_id, definition, description
            )
          VALUES (
            'Undefined geographic SRS', 0, 'NONE', 0,
            'undefined',
            'undefined geographic coordinate reference system'
            );

        INSERT
          INTO gpkg_spatial_ref_sys (
            srs_name, srs_id, organization,
            organization_coordsys_id, definition, description
            )
          VALUES (
            'WGS 84 geodetic', 4326, 'EPSG', 4326,
            'GEOGCS["WGS 84",DATUM["WGS_1984",SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],AUTHORITY["EPSG","6326"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AXIS["Latitude",NORTH],AXIS["Longitude",EAST],AUTHORITY["EPSG","4326"]]',
            'longitude/latitude coordinates in decimal degrees on the WGS 84 spheroid'
            );
        "#,
    )
    .execute(&pool)
    .await?;

    // Should max/min values be computed from the data instead?
    trace!("Inserting reference of feature table `ord_areas`");
    query(
        r#"
      INSERT
        INTO gpkg_contents (
          table_name, data_type, identifier, description,
          last_change, min_x, min_y, max_x, max_y, srs_id)
        VALUES (
          'ord_areas', 'features', 'ord_areas', '',
          strftime('%Y-%m-%dT%H:%M:%fZ','now'),
          -179.148909, 18.910361,
          179.778470112501, 71.365162, 4326);
        "#,
    )
    .execute(&pool)
    .await?;

    //trace!("Inserting reference to geometry column in `o");
    trace!("Linking geometry column `geom` in table `ord_areas`");
    query(
        r#"
        INSERT
          INTO gpkg_geometry_columns (
            table_name, column_name, geometry_type_name,
            srs_id, z, m
            )
          VALUES (
            'ord_areas', 'geom', 'GEOMETRY', 4326, 0, 0);
            "#,
    )
    .execute(&pool)
    .await?;

    query("PRAGMA integrity_check").execute(&pool).await?; // -> ok
    query("PRAGMA foreign_key_check").execute(&pool).await?; // -> empty

    Ok(())
}
