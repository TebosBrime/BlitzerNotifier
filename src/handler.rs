use std::collections::HashMap;
use crate::blitzer_api_client::get_blitzer_api_result;
use crate::model::{BlitzerClientRequestParams, LocationType, Poi};
use crate::telegram::TelegramBot;
use crate::{configuration, database};
use strum::IntoEnumIterator;
use crate::database::KnownPoi;

pub(crate) async fn handle(telegram_bot: &TelegramBot) -> Result<(), anyhow::Error> {
    println!("Start BlitzerNotifier!");

    let location_box = configuration::get_location_box().await;
    println!("Working with locationBox: {}", location_box);

    println!("Init database connection...");
    let mut database = database::Repository::try_new().await?;

    let request_params = BlitzerClientRequestParams {
        zoom_level: 5,
        types: LocationType::iter()
            .filter(|location_type: &LocationType| location_type.is_default())
            .collect(),
        location_box,
    };

    let api_response = get_blitzer_api_result(&request_params).await?;
    println!("Found {} pois in the given area", api_response.pois.len());

    let mut known_pois: HashMap<String, KnownPoi> = database.get_known_pois().into_iter()        
        .map(|known_poi| (known_poi.backend_id.clone(), known_poi)) 
        .collect();         
    println!("There are {} active pois in the database", known_pois.len());

    let mut new_pois = Vec::new();
    for poi in api_response.pois {
        match poi {
            Poi::Detailed(detailed_poi) => {
                if let Some(known_poi) = known_pois.remove(&detailed_poi.backend) {
                    println!(
                        "Found poi in database: {}, {}. Chat: {}: {}, {}",
                        detailed_poi.id, detailed_poi.backend, known_poi.chat_id, known_poi.message_id_info, known_poi.message_id_location
                    );
                    continue;

                }

                new_pois.push(detailed_poi);
            }
            Poi::Cluster(cluster_poi) => {
                println!("Found cluster poi.. but skipped: {:?}", cluster_poi)
            }
        }
    }

    for poi in new_pois.iter().clone() {
        println!("Found new poi: {:?}.. sending telegram message", poi);
        let info_message = telegram_bot.send_message(poi.to_telegram_message()).await;

        let latitude = poi.lat.parse::<f64>().expect("Failed to parse latitude");
        let longitude = poi.lng.parse::<f64>().expect("Failed to parse longitude");
        let location_message = telegram_bot.send_location(latitude, longitude).await;

        database
            .add_poi(
                poi.clone(),
                info_message.chat.id,
                info_message.id,
                location_message.id,
            );
    }
    
    for known_poi in known_pois.values().clone() {
        println!("Poi {:?} is now inactive.. going to delete messages", known_poi.backend_id);

        telegram_bot.delete_message(known_poi.chat_id, known_poi.message_id_info, known_poi.message_id_location).await;
        database.update_last_seen(known_poi.id.clone());
    }

    Ok(())
}
