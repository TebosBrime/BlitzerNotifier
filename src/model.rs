use serde::{Deserialize, Serialize};
use std::fmt::Display;
use serde_variant::to_variant_name;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, EnumIter, Debug)]
pub enum LocationType {
    #[serde(rename="0")]
    BlitzerMobile0,
    #[serde(rename="1")]
    BlitzerMobile1,
    #[serde(rename="2")]
    BlitzerMobile2,
    #[serde(rename="3")]
    BlitzerMobile3,
    #[serde(rename="4")]
    BlitzerMobile4,
    #[serde(rename="5")]
    BlitzerMobile5,
    #[serde(rename="6")]
    BlitzerMobile6,
    #[serde(rename="20")]
    Stauende,
    #[serde(rename="21")]
    Gefahrenstelle21,
    #[serde(rename="22")]
    Baustelle22,
    #[serde(rename="23")]
    Gefahrenstelle23,
    #[serde(rename="24")]
    Gefahrenstelle24,
    #[serde(rename="25")]
    Gefahrenstelle25,
    #[serde(rename="26")]
    Baustelle26,
    #[serde(rename="29")]
    Gefahrenstelle29,
    #[serde(rename="101")]
    Blitzer101,
    #[serde(rename="102")]
    Blitzer102,
    #[serde(rename="103")]
    Blitzer103,
    #[serde(rename="104")]
    Blitzer104,
    #[serde(rename="105")]
    Blitzer105,
    #[serde(rename="106")]
    Blitzer106,
    #[serde(rename="107")]
    Blitzer107,
    #[serde(rename="108")]
    Blitzer108,
    #[serde(rename="109")]
    Blitzer109,
    #[serde(rename="110")]
    Blitzer110,
    #[serde(rename="111")]
    Blitzer111,
    #[serde(rename="112")]
    Blitzer112,
    #[serde(rename="113")]
    Blitzer113,
    #[serde(rename="114")]
    Tunnel,
    #[serde(rename="115")]
    Blitzer115,
    #[serde(rename="117")]
    Blitzer117,
    #[serde(rename="1015")]
    Kulturguide1015,
    #[serde(rename="1016")]
    Kulturguide1016,
    #[serde(rename="2015")]
    Hotspot2015,
    #[serde(rename="ts")]
    BlitzerTeilstat,
    #[serde(rename="vwd")]
    Polizeimeldung1,
    #[serde(rename="vwda")]
    Polizeimeldung2,
    #[serde(rename="traffic")]
    PolylineTraffic,
    #[serde(rename="pics")]
    BlitzerBilder,
}
impl LocationType {
    pub fn is_default(&self) -> bool {
        matches!(self, 
            | LocationType::Blitzer101
            | LocationType::Blitzer102
            | LocationType::Blitzer103
            | LocationType::Blitzer104
            | LocationType::Blitzer105
            | LocationType::Blitzer106
            | LocationType::Blitzer107
            | LocationType::Blitzer108
            | LocationType::Blitzer109
            | LocationType::Blitzer110
            | LocationType::Blitzer111
            | LocationType::Blitzer112
            | LocationType::Blitzer113
            | LocationType::Tunnel
            | LocationType::Blitzer115
            | LocationType::Blitzer117
            | LocationType::BlitzerTeilstat
            | LocationType::BlitzerMobile0
            | LocationType::BlitzerMobile1
            | LocationType::BlitzerMobile2
            | LocationType::BlitzerMobile3
            | LocationType::BlitzerMobile4
            | LocationType::BlitzerMobile5
            | LocationType::BlitzerMobile6
            | LocationType::Polizeimeldung1)
    }
}

pub struct BlitzerClientRequestParams {
    pub zoom_level: i32,
    pub types: Vec<LocationType>,
    pub location_box: LocationBox,
}

impl BlitzerClientRequestParams {
    pub fn get_request_params(&self) -> String {
        format!(
            "?z={}&type={}&box={:#}",
            self.zoom_level,
            self.types.iter()
                .map(|t| to_variant_name(t).expect("Should be rename enum value"))
                .collect::<Vec<_>>()
                .join(","),
            self.location_box
        )
    }
}

pub struct LocationBox {
    pub lat_min: f64,
    pub lng_min: f64,
    pub lat_max: f64,
    pub lng_max: f64,
}

impl Display for LocationBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.lat_min, self.lng_min, self.lat_max, self.lng_max
        )
    }
}


// Response
#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub pois: Vec<Poi>,
    // pub grid: Vec<Grid>,
    // pub infos: Vec<Info>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)] // Allow POIs to take on different forms without explicit tags in the JSON
pub enum Poi {
    Detailed(DetailedPoi),
    Cluster(ClusterPoi),
}

#[derive(Debug, Deserialize, Clone)]
pub struct DetailedPoi {
    pub id: String,
    pub lat: String,
    pub lng: String,
    pub address: Address,
    pub content: String,
    pub backend: String,
    #[serde(rename = "type")]
    pub poi_type: String,
    pub vmax: String,
    pub create_date: String,
    pub confirm_date: String,
    pub info: Info,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ClusterPoi {
    pub style: Option<u32>,
    pub counter: Option<String>, 
    pub lat: f64,
    pub lng: f64,
    #[serde(rename = "type")]
    pub cluster_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Address {
    pub country: String,
    pub state: String,
    pub zip_code: String,
    pub city: String,
    pub city_district: String,
    pub street: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Info {
    pub desc: Option<String>,
}

impl DetailedPoi {
    pub fn to_telegram_message(&self) -> String {
        let cloned_poi = self.to_owned();
        
        let mut city = format!("{} {}", cloned_poi.address.zip_code, cloned_poi.address.city);
        if !&cloned_poi.address.city_district.is_empty() {
            city = format!("{city} ({})", cloned_poi.address.city_district);
        }
        
        let poi_type: LocationType = serde_json::from_str(&format!("\"{}\"", cloned_poi.poi_type))
            .expect("Invalid value for LocationType");
        
        let mut base_message = format!("Attention: A new point of interest found at {city}: \n\nAddress: {}\nType: {:?}\nMax speed: {}",
                                       cloned_poi.address.street,
                                       poi_type,
                                       cloned_poi.vmax
        );
        
        if cloned_poi.info.desc.is_some() {
            base_message = format!("{} \n\nAdditional info: {}", base_message, cloned_poi.info.desc.unwrap());
        }
        
        base_message
    }
}