use crate::blitzer_api_client::get_blitzer_api_result;
use crate::model::{BlitzerClientRequestParams, LocationType, Poi};
use strum::IntoEnumIterator;
use crate::telegram::TelegramBot;

mod blitzer_api_client;
mod database;
mod model;
mod configuration;
mod telegram;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let telegram_bot = TelegramBot::try_new().await;

    if let Err(error) = handler(&telegram_bot).await {
        eprintln!("Error: {}", error);
        telegram_bot.send_message(format!("Failed to execute bot... see log for more information. {}", error)).await;
    }
    
    Ok(())
}

async fn handler(telegram_bot: &TelegramBot) -> Result<(), anyhow::Error> {
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

    let api_response = get_blitzer_api_result(request_params).await?;
    println!("Found {} pois in the given area", api_response.pois.len());

    let known_pois = database.get_known_poi_ids();
    let mut new_pois = Vec::new();
    for poi in api_response.pois {
        match poi {
            Poi::Detailed(detailed_poi) => {
                if known_pois.contains(&detailed_poi.id) {
                    println!("Found poi in database: {:?}", detailed_poi.id);
                    continue
                }

                new_pois.push(detailed_poi);
            }
            Poi::Cluster(cluster_poi) => {
                println!("Found cluster poi.. but skipped: {:?}", cluster_poi)
            }
        }
    }

    if new_pois.is_empty() {
        return Ok(());
    }

    for poi in new_pois.iter().clone() {
        println!("Found new poi: {:?}", poi);
        telegram_bot.send_message(poi.to_telegram_message()).await;
    }

    database.add_poi(new_pois);

    Ok(())
}
