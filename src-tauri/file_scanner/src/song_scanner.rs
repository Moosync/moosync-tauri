use std::{path::PathBuf, sync::mpsc::Sender};

use crate::utils::{check_directory, get_files_recursively, scan_file};
use database::{database::Database, types::songs::Song};
use threadpool::ThreadPool;
use types::errors::errors::Result;

pub struct SongScanner<'a> {
    dir: PathBuf,
    pool: &'a mut ThreadPool,
    thumbnail_dir: PathBuf,
    artist_split: String,
}

impl<'a> SongScanner<'a> {
    pub fn new(
        dir: PathBuf,
        pool: &'a mut ThreadPool,
        thumbnail_dir: PathBuf,
        artist_split: String,
    ) -> Self {
        Self {
            dir,
            pool,
            thumbnail_dir,
            artist_split,
        }
    }

    fn check_dirs(&self) -> Result<()> {
        check_directory(self.thumbnail_dir.clone())?;

        Ok(())
    }

    pub fn scan_in_pool(
        &self,
        tx: Sender<(Option<String>, Result<Song>)>,
        size: f64,
        path: PathBuf,
        playlist_id: Option<String>,
    ) {
        let thumbnail_dir = self.thumbnail_dir.clone();
        let artist_split = self.artist_split.clone();
        self.pool.execute(move || {
            let mut metadata = scan_file(&path, &thumbnail_dir, size, false, &artist_split);
            if metadata.is_err() {
                metadata = scan_file(&path, &thumbnail_dir, size, true, &artist_split);
            }

            tx.send((playlist_id, metadata))
                .expect("channel will be there waiting for the pool");
        });
    }

    pub fn start(
        &self,
        db: &Database,
        tx_song: Sender<(Option<String>, Result<Song>)>,
        force: bool,
    ) -> Result<usize> {
        self.check_dirs()?;

        let file_list = get_files_recursively(self.dir.clone())?;

        let song_list = if !force {
            db.files_not_in_db(file_list.file_list)?
        } else {
            file_list.file_list
        };

        println!(
            "file list: {:?}",
            song_list
                .iter()
                .map(|v| v.0.clone())
                .collect::<Vec<PathBuf>>()
        );

        let len = song_list.len();

        for (file_path, size) in song_list {
            self.scan_in_pool(tx_song.clone(), size, file_path, None);
        }

        drop(tx_song);

        Ok(len)
    }
}
