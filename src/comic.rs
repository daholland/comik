use std::{fs::File, path::PathBuf, sync::Arc};

use anyhow::Result;
use image::DynamicImage;
use thiserror::Error;
use unrar::Archive as RarArchive;
use zip::{result::ZipError, ZipArchive};

#[derive(Error, Debug, Clone)]
pub enum ComicError {
    #[error("invalid archive type")]
    InvalidArchiveType,
    #[error("unable to parse rar file")]
    RarParseError,
    #[error("unknown comic error")]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Page {
    file_name: String,
    path: PathBuf,
    image: Option<DynamicImage>,
}

impl Page {}

#[derive(Clone, Debug)]
pub struct Comic {
    pub title: String,
    pub folder_path: PathBuf,
    pub pages: Vec<Arc<Page>>,
}

impl Comic {
    pub async fn from_archive_path(path: PathBuf) -> Result<Self, ComicError> {
        println!("got archive {:?}", path);

        return match path.extension() {
            Some(ext) if ext == "zip" || ext == "cbz" => Comic::from_zip(path),
            Some(ext) if ext == "rar" || ext == "cbr" => Comic::from_rar(path),
            _ => Err(ComicError::InvalidArchiveType),
        };
    }

    fn from_zip(path: PathBuf) -> Result<Self, ComicError> {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let temp_directory = tempfile::tempdir().unwrap().into_path();

        let file = File::open(&path).unwrap();
        let reader = std::io::BufReader::new(file);

        let mut zip_archive = ZipArchive::new(reader).unwrap();

        let pages: Vec<Arc<Page>> = zip_archive
            .file_names()
            .map(|name| {
                let path = path.join(name);

                Arc::new(Page {
                    file_name: name.to_string(),
                    image: None,
                    path,
                })
            })
            .collect::<Vec<Arc<Page>>>();

        zip_archive.extract(&temp_directory).unwrap();

        Ok(Self {
            title: file_name,
            folder_path: temp_directory,
            pages,
        })
    }

    fn from_rar(path: PathBuf) -> Result<Self, ComicError> {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let path_string = path.to_str().unwrap().to_string();

        let temp_directory = tempfile::tempdir().unwrap().into_path();

        let pages: Vec<Arc<Page>> = RarArchive::new(path_string.clone())
            .list()
            .unwrap()
            .process()
            .unwrap()
            .into_iter()
            .map(|entry| {
                let filename = entry.filename;
                let path = temp_directory.join(&filename);

                Arc::new(Page {
                    file_name: filename,
                    image: None,
                    path,
                })
            })
            .collect::<Vec<Arc<Page>>>();

        RarArchive::new(path_string.clone())
            .extract_to(temp_directory.to_str().unwrap_or_default().to_string())
            .unwrap()
            .process()
            .unwrap();

        Ok(Self {
            title: file_name,
            folder_path: temp_directory,
            pages,
        })
    }
}
