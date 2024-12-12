use crate::configuration;
use crate::model::DetailedPoi;
use mysql::prelude::Queryable;
use mysql::{params, Pool, PooledConn};
use teloxide::types::{ChatId, MessageId};

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
                first_seen DATETIME DEFAULT CURRENT_TIMESTAMP,
                last_seen DATETIME DEFAULT NULL,
                chat_id BIGINT NOT NULL,
                message_id_info INT NOT NULL,
                message_id_location INT NOT NULL
            )",
        )?;

        Ok(Self { connection: conn })
    }

    pub fn add_poi(
        &mut self,
        poi: DetailedPoi,
        chat_id: ChatId,
        info_message_id: MessageId,
        location_message_id: MessageId,
    ) {
        self.connection.exec_drop(
            r"INSERT INTO known_blitzer (
                    id, lat, lng, address_country, address_state, address_zip_code, address_city,
                    address_city_district, address_street, content, backend, poi_type, vmax,
                    create_date, confirm_date, info_desc, chat_id, message_id_info, message_id_location
                ) VALUES (
                    :id, :lat, :lng, :address_country, :address_state, :address_zip_code, :address_city,
                    :address_city_district, :address_street, :content, :backend, :poi_type, :vmax,
                    :create_date, :confirm_date, :info_desc, :chat_id, :message_id_info, :message_id_location
                )",
            params! {
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
                "chat_id" => chat_id.0,
                "message_id_info" => info_message_id.0,
                "message_id_location" => location_message_id.0,
            }
        ).expect("Should write poi to database");
    }

    pub fn get_known_pois(&mut self) -> Vec<KnownPoi> {
        let known_blitzer: Vec<KnownPoi> = self
            .connection
            .query_map(
                "SELECT id,backend,chat_id,message_id_info,message_id_location from known_blitzer WHERE last_seen IS NULL",
                |(id, backend_id, chat_id, message_id_info, message_id_location)| KnownPoi {
                    id,
                    backend_id,
                    chat_id,
                    message_id_info,
                    message_id_location,
                },
            )
            .expect("Should get backend id of poi from database");

        known_blitzer
    }

    pub fn update_last_seen(&mut self, poi_id: String) {
        self.connection.exec_drop(
            r"UPDATE known_blitzer SET last_seen = CURRENT_TIMESTAMP() WHERE id = :id",
            params! {
                "id" => poi_id,
            }
        ).expect("Should write poi to database");

    }
}

pub struct KnownPoi {
    pub id: String,
    pub backend_id: String,
    pub chat_id: i64,
    pub message_id_info: i32,
    pub message_id_location: i32,
}
