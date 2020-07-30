use notify::{Watcher, watcher, DebouncedEvent, RecursiveMode, RecommendedWatcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use crate::Error;
use std::thread;
use std::path::PathBuf;

pub struct WatcherHandle {
    watcher: RecommendedWatcher,
}

pub fn create_watcher(path: &PathBuf, debounce: u64) -> Result<WatcherHandle, Error> {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_millis(debounce))?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(event) => {

                    match event {
                        DebouncedEvent::Write(path) => {
                            if let Some(extension) = path.extension() {
                                let extension = extension.to_string_lossy();
                                if extension == "rs" {
                                    crate::wasm_pack::run_wasm_pack().unwrap()
                                }
                            }
                        },
                        DebouncedEvent::Create(path) => {
                            if let Some(extension) = path.extension() {
                                let extension = extension.to_string_lossy();
                                if extension == "rs" {
                                    crate::wasm_pack::run_wasm_pack().unwrap()
                                }
                            }
                        },
                        DebouncedEvent::Remove(path) => {
                            if let Some(extension) = path.extension() {
                                let extension = extension.to_string_lossy();
                                if extension == "rs" {
                                    crate::wasm_pack::run_wasm_pack().unwrap()
                                }
                            }
                        },
                        DebouncedEvent::Rename(from, to) => {
                        },
                        _ => {},
                    }

                },
                Err(e) => {
                    dbg!(e);
                },
            }
        }
    });

    Ok(WatcherHandle {
        watcher: watcher,
    })
}
