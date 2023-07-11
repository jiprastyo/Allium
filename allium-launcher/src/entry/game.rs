use std::{
    ffi::OsStr,
    fs, mem,
    path::{Path, PathBuf},
};

use anyhow::Result;
use common::constants::ALLIUM_GAMES_DIR;
use log::info;
use serde::{Deserialize, Serialize};

use crate::entry::{lazy_image::LazyImage, short_name};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Game {
    pub name: String,
    pub full_name: String,
    pub path: PathBuf,
    pub image: LazyImage,
    pub extension: String,
}

impl Game {
    pub fn new(path: PathBuf) -> Game {
        let full_name = path
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("")
            .to_string();
        let name = short_name(&full_name);
        let extension = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("")
            .to_string();
        let image = LazyImage::Unknown(path.clone());
        Game {
            name,
            full_name,
            path,
            image,
            extension,
        }
    }

    pub fn image(&mut self) -> Option<&Path> {
        self.image.image()
    }

    /// Attempts to resync the game path with the games directory. Returns the old path if it changed.
    pub fn resync(&mut self) -> Result<Option<PathBuf>> {
        Ok(if self.path.exists() {
            None
        } else if let Some(name) = self.path.file_name() {
            if let Some(game) = find(&ALLIUM_GAMES_DIR, name)? {
                info!("Resynced game path: {:?}", game);
                Some(mem::replace(&mut self.path, game))
            } else {
                None
            }
        } else {
            None
        })
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.full_name.cmp(&other.full_name)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find(path: &Path, name: &OsStr) -> Result<Option<PathBuf>> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let game = find(&path, name)?;
                if game.is_some() {
                    return Ok(game);
                }
            } else if path.file_name() == Some(name) {
                return Ok(Some(path));
            }
        }
    }
    Ok(None)
}
