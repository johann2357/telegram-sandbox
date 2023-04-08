use dotenv::dotenv;
use std::env;
use tokio;

mod telegram;

#[tokio::main]
async fn main() -> Result<(), telegram::errors::TelegramError> {
    // Load the environment variables from the .env file
    dotenv().ok();
    // Test telegram workflow
    let telegram_token =
        env::var("TELEGRAM_TOKEN").expect("Environment variable TELEGRAM_TOKEN not set");

    let updates = telegram::updates::get_updates(784250033, 50, 60, &telegram_token)
        .await
        .expect("Couldn't get updates from Telegram.");
    for update in updates.result {
        println!("");
        println!("\tUpdate ID: {:?}", update.update_id);
        println!(
            "\tUpdate Message Username: {:?}",
            update.message.from.username
        );
        println!(
            "\tUpdate Message Chat type: {:?}",
            update.message.chat.r#type
        );
        println!(
            "\tUpdate Message Chat Group: {:?}",
            update.message.chat.title
        );
        if update.message.text.is_some() {
            println!("\tUpdate Message Text: {:?}", update.message.text.unwrap());
        }
        if update.message.photo.is_some() {
            let all_photos = update.message.photo.unwrap_or_default();

            let best_photo = all_photos.iter().max_by_key(|photo| photo.width.clone());

            if best_photo.is_some() {
                let photo = best_photo.unwrap();
                println!(
                    "\tUpdate Message Best Photo: {:?}x{:?}",
                    photo.width, photo.height
                );
                let file_id = photo.file_id.clone();
                let file = telegram::file::get_file(file_id, &telegram_token).await?;
                println!("\t\tFile Path: {:?}", file.result.file_path);
            }
        }
        println!("");
    }

    Ok(())
}
