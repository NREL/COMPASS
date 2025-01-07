
import geopandas

df = geopandas.read_file("ord_areas.gpkg")

states = df[["state_name", "geometry"]].dissolve(by="state_name")

states.merge(df[["state_name", "state_fips"]].drop_duplicates(), on="state_name")
county = df[["state_name", "state_fips", "county_name", "county_fips", "geometry"]].dropna()
subd = df[["state_name", "state_fips", "subd_name", "subd_fips", "geometry"]].dropna()


insert into bookkeeping (hash) values ('dummy_hash');
INSERT INTO jurisdiction (name, bookkeeping_lnk, fips, rank, geometry) VALUES ('Kentucky', 1, 7, 'county', 'POLYGON ((-89.181 37.046, -88.815 36.954, -89.053 36.932, -89.181 37.046))');
