use clipboard::{ClipboardContext, ClipboardProvider};
use notify::event::{AccessKind, AccessMode, EventKind};
use notify::{recommended_watcher, Event, RecursiveMode, Result as NotifyResult, Watcher};

use std::{error::Error, future::Future, path::Path, pin::Pin, sync::mpsc};

pub type UploadFn = Box<
    dyn for<'a> Fn(
            &'a str,
        )
            -> Pin<Box<dyn Future<Output = Result<String, Box<dyn Error>>> + Send + 'a>>
        + Send
        + Sync,
>;

pub async fn watcher(screenshot_path: &Path, upload_fn: UploadFn) -> Result<(), Box<dyn Error>> {
    let mut c_context: ClipboardContext = ClipboardProvider::new()?;
    let (tx, rx) = mpsc::channel::<NotifyResult<Event>>();

    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(screenshot_path, RecursiveMode::NonRecursive)?;

    loop {
        match rx.recv() {
            Ok(res) => match res {
                Ok(event) => {
                    if let EventKind::Access(access) = event.kind {
                        if access == AccessKind::Close(AccessMode::Write) {
                            let file_path = event.paths[0]
                                .to_str()
                                .ok_or("Failed to convert path to string")?;

                            println!("Processing file: {}", file_path);

                            match upload_fn(file_path).await {
                                Ok(file_url) => {
                                    println!("Successfully uploaded: {}", file_path);
                                    c_context.set_contents(String::from(file_url))?;
                                }
                                Err(err) => eprintln!("Failed to upload {}: {}", file_path, err),
                            }
                        }
                    }
                }
                Err(err) => println!("err: {:#?}", err),
            },
            Err(err) => {
                eprintln!("Channel error: {}", err);
                break;
            }
        }
    }

    Ok(())
}
