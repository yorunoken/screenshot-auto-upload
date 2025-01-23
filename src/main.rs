use clap::{Parser, ValueEnum};
use config::Config;
use dir_watcher::{watcher, UploadFn};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod config;
mod dir_watcher;
mod uploader;

#[derive(Clone, ValueEnum, Debug, Serialize, Deserialize)]
pub enum Provider {
    #[value(name = "s-ul")]
    SUl,
    #[value(name = "imgur")]
    Imgur,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Absolute path to the folder that will be watched for images
    #[clap(short = 'p', long = "path", aliases = ["screenshot", "image"])]
    screenshot_path: Option<String>,

    /// image service provider to use
    #[clap(short = 't', long = "provider", aliases = ["type", "service"])]
    provider: Option<Provider>,

    /// API key for the selected provider service
    #[clap(short = 'k', long = "key", aliases = ["api-key", "token"])]
    key: Option<String>,

    /// Whether to save the current configuration
    #[clap(short = 's', long = "save-config", aliases = ["save"], action = clap::ArgAction::SetTrue, default_value_t = false)]
    save_config: bool,

    /// Absolute path to the configuration file
    #[clap(short = 'c', long = "config", aliases = ["config-file"], default_value_t = config::get_default_config_path())]
    config_path: String,
}

#[tokio::main]
async fn main() {
    if let Err(err) = dotenv() {
        eprintln!("Failed to load .env file: {}", err);
    }

    let args = Args::parse();
    let (provider, key, screenshot_path) = if let Ok(config) = Config::load(&args.config_path) {
        (
            args.provider.unwrap_or(config.provider),
            args.key.unwrap_or(config.key),
            args.screenshot_path.unwrap_or(config.screenshot_watch_path),
        )
    } else {
        (
            args.provider
                .expect("Provider must be specified when no config exists"),
            args.key
                .expect("Key must be specified when no config exists"),
            args.screenshot_path
                .expect("Screenshot path must be specified when no config exists"),
        )
    };

    if args.save_config {
        let config = Config::new(key.clone(), provider.clone(), screenshot_path.clone());

        if let Err(err) = config.save(&args.config_path) {
            eprintln!("Failed to save config: {}", err);
        }
    }

    let upload_fn: UploadFn = match provider {
        Provider::SUl => {
            Box::new(move |file_path| Box::pin(uploader::sul_upload(file_path, key.clone())))
        }
        Provider::Imgur => {
            Box::new(move |file_path| Box::pin(uploader::imgur_upload(file_path, key.clone())))
        }
    };

    let path = Path::new(&screenshot_path);
    println!("Watching");
    if let Err(err) = watcher(path, upload_fn).await {
        println!("{:#?}", err);
    }
}
