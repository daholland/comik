use std::{cmp::Ordering, fs::File, path::PathBuf};

use anyhow::Result;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageError, ImageOutputFormat};
use thiserror::Error;
use unrar::Archive as RarArchive;
use zip::ZipArchive;

#[derive(Error, Debug, Clone)]
pub enum ComicError {
    #[error("invalid archive type")]
    InvalidArchiveType,
}

#[derive(Debug, Clone, Eq)]
pub struct Page {
    file_name: String,
    path: PathBuf,
}

impl Page {
    pub fn as_image(&self) -> Result<DynamicImage, ImageError> {
        ImageReader::open(&self.path)?.decode()
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, ImageError> {
        let mut buffer: Vec<u8> = Vec::new();

        let _ = &self
            .as_image()
            .unwrap()
            .write_to(&mut buffer, ImageOutputFormat::Png)
            .unwrap();

        Ok(buffer)
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_name.cmp(&other.file_name)
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.file_name == other.file_name
    }
}

#[derive(Debug, Default)]
pub struct ComicCollection {
    pub title: Option<String>,
    pub source: Option<String>,
    pub paths: Vec<PathBuf>,
}

impl ComicCollection {
    pub fn new(paths: Vec<PathBuf>) -> Result<ComicCollection> {
        let filtered_paths = paths.iter().cloned().filter(|path| {
            if let Some(ext) = path.extension() {
                ext == "zip" || ext == "cbz" || ext == "rar" || ext == "cbr"
            } else {
                false
            }
        }).collect::<Vec<PathBuf>>();

        Ok(ComicCollection {
            paths: filtered_paths,
            ..Default::default()
        })
    }
}

#[derive(Clone, Debug)]
pub struct Comic {
    pub title: String,
    pub folder_path: PathBuf,
    pub pages: Vec<Page>,
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

        let mut pages: Vec<Page> = zip_archive
            .file_names()
            .map(|name| {
                let path = temp_directory.join(name);

                Page {
                    file_name: name.to_string(),
                    path,
                }
            })
            .collect::<Vec<Page>>();

        pages.sort();

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

        let mut pages: Vec<Page> = RarArchive::new(path_string.clone())
            .list()
            .unwrap()
            .process()
            .unwrap()
            .into_iter()
            .map(|entry| {
                let filename = entry.filename;
                let path = temp_directory.join(&filename);

                Page {
                    file_name: filename,
                    path,
                }
            })
            .collect::<Vec<Page>>();

        pages.sort();

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
