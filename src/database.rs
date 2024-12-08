use crate::configuration;
use crate::model::DetailedPoi;
use mysql::prelude::Queryable;
use mysql::{params, Pool, PooledConn};

pub struct Repository {
    connection: PooledConn,
}
impl Repository {
    pub async fn try_new() -> anyhow::Result<Self> {
        let connection_uri = configuration::get_mysql_connection_uri().await;
        let pool = Pool::new(connection_uri.as_str())?;

        let mut conn = pool.get_conn()?;

        conn.query_drop(
            "CREATE TABLE IF NOT EXISTS known_blitzer (
                id VARCHAR(255) PRIMARY KEY,
                lat VARCHAR(255) NOT NULL,
                lng VARCHAR(255) NOT NULL,
                address_country VARCHAR(255) NOT NULL,
                address_state VARCHAR(255) NOT NULL,
                address_zip_code VARCHAR(255) NOT NULL,
                address_city VARCHAR(255) NOT NULL,
                address_city_district VARCHAR(255) NOT NULL,
                address_street VARCHAR(255) NOT NULL,
                content TEXT NOT NULL,
                backend VARCHAR(255) NOT NULL,
                poi_type VARCHAR(255) NOT NULL,
                vmax VARCHAR(255) NOT NULL,
                create_date VARCHAR(255) NOT NULL,
                confirm_date VARCHAR(255) NOT NULL,
                info_desc TEXT,
                first_found DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
        )?;

        Ok(Self { connection: conn })
    }

    pub fn add_poi(&mut self, pois: Vec<DetailedPoi>) {
        self.connection.exec_batch(
            r"INSERT INTO known_blitzer (
                    id, lat, lng, address_country, address_state, address_zip_code, address_city,
                    address_city_district, address_street, content, backend, poi_type, vmax,
                    create_date, confirm_date, info_desc
                ) VALUES (
                    :id, :lat, :lng, :address_country, :address_state, :address_zip_code, :address_city,
                    :address_city_district, :address_street, :content, :backend, :poi_type, :vmax,
                    :create_date, :confirm_date, :info_desc
                )",
            pois.into_iter().map(|poi| params! {
                "id" => poi.id,
                "lat" => poi.lat,
                "lng" => poi.lng,
                "address_country" => poi.address.country,
                "address_state" => poi.address.state,
                "address_zip_code" => poi.address.zip_code,
                "address_city" => poi.address.city,
                "address_city_district" => poi.address.city_district,
                "address_street" => poi.address.street,
                "content" => poi.content,
                "backend" => poi.backend,
                "poi_type" => poi.poi_type,
                "vmax" => poi.vmax,
                "create_date" => poi.create_date,
                "confirm_date" => poi.confirm_date,
                "info_desc" => poi.info.desc.as_deref(),
            })
        ).expect("Should write poi to database");
    }

    pub fn get_known_poi_backend_ids(&mut self) -> Vec<String> {
        let known_blitzer: Vec<String> = self
            .connection
            .query_map("SELECT backend from known_blitzer", |backend| backend)
            .expect("Should get backend id of poi from database");

        known_blitzer
    }
}
