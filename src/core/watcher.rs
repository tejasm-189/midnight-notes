use notify::Watcher;
use std::path::Path;
use std::sync::mpsc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WatcherError {
    #[error("notify error: {0}")]
    Notify(#[from] notify::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("receive error: {0}")]
    Recv(#[from] mpsc::RecvError),
}

pub enum FileEvent {
    Created(String),
    Modified(String),
    Deleted(String),
}

pub struct FileWatcher {
    _watcher: notify::RecommendedWatcher,
    rx: mpsc::Receiver<FileEvent>,
}

impl FileWatcher {
    pub fn watch(path: &Path) -> Result<Self, WatcherError> {
        let (tx, rx) = mpsc::channel();

        let mut watcher =
            notify::recommended_watcher(move |event: Result<notify::Event, notify::Error>| {
                if let Ok(event) = event {
                    let make_event: Option<fn(String) -> FileEvent> = match event.kind {
                        notify::EventKind::Create(_) => {
                            Some(FileEvent::Created as fn(String) -> FileEvent)
                        }
                        notify::EventKind::Modify(_) => {
                            Some(FileEvent::Modified as fn(String) -> FileEvent)
                        }
                        notify::EventKind::Remove(_) => {
                            Some(FileEvent::Deleted as fn(String) -> FileEvent)
                        }
                        _ => None,
                    };
                    if let Some(make) = make_event {
                        for path in event.paths {
                            if let Some(path_str) = path.to_str() {
                                let _ = tx.send(make(path_str.to_string()));
                            }
                        }
                    }
                }
            })?;

        watcher.watch(path, notify::RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: watcher,
            rx,
        })
    }

    pub fn next_event(&self) -> Result<FileEvent, WatcherError> {
        Ok(self.rx.recv()?)
    }

    pub fn try_next_event(&self) -> Result<Option<FileEvent>, WatcherError> {
        match self.rx.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(WatcherError::Recv(mpsc::RecvError)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_watch_directory() {
        let dir = tempdir().unwrap();
        let watcher = FileWatcher::watch(dir.path()).unwrap();

        // Create a file and check for event
        std::fs::write(dir.path().join("test.md"), "hello").unwrap();

        // Give the watcher time to pick up the event
        std::thread::sleep(std::time::Duration::from_millis(500));

        let event = watcher.try_next_event().unwrap();
        assert!(event.is_some());
        if let Some(ev) = event {
            match ev {
                FileEvent::Created(path) | FileEvent::Modified(path) => {
                    assert!(path.ends_with("test.md"));
                }
                _ => panic!("expected create or modify event"),
            }
        }
    }
}
