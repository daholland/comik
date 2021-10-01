use std::{cmp::Ordering, fs::File, path::PathBuf};

use anyhow::Result;
use unrar::Archive as RarArchive;
use zip::ZipArchive;

use super::{CollectionProvider, ComicProvider, PageProvider, ProviderError};

#[derive(Debug)]
pub struct FileSystemCollectionProvider {
    collection_name: String,
    comics: Vec<FileSystemComicProvider>,
}

impl FileSystemCollectionProvider {
    pub fn new(collection_name: String, paths: Vec<PathBuf>) -> Result<Self> {
        let comics: Vec<_> = paths
            .iter()
            .filter_map(|path| FileSystemComicProvider::from_archive_path(path.clone()).ok())
            .collect();

        Ok(Self {
            collection_name,
            comics,
        })
    }
}

impl CollectionProvider for FileSystemCollectionProvider {
    fn get_name(&self) -> String {
        self.collection_name.clone()
    }

    fn get_comic(&self, index: usize) -> Option<&dyn ComicProvider> {
        Some(self.comics.get(index).unwrap())
    }

    fn get_size(&self) -> usize {
        self.comics.len()
    }
}

#[derive(Debug)]
pub struct FileSystemComicProvider {
    title: String,
    pages: Vec<FileSystemPageProvider>,
}

impl FileSystemComicProvider {
    fn from_archive_path(path: PathBuf) -> Result<Self, ProviderError> {
        return match path.extension() {
            Some(ext) if ext == "zip" || ext == "cbz" => Self::from_zip(path),
            Some(ext) if ext == "rar" || ext == "cbr" => Self::from_rar(path),
            _ => Err(ProviderError::InvalidArchiveType),
        };
    }

    fn from_zip(path: PathBuf) -> Result<Self, ProviderError> {
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

        zip_archive.extract(&temp_directory).unwrap();

        let mut pages: Vec<FileSystemPageProvider> = zip_archive
            .file_names()
            .map(|name| {
                let path = temp_directory.join(name);
                let img = image::io::Reader::open(path).unwrap().decode().unwrap();

                FileSystemPageProvider {
                    file_name: name.to_string(),
                    image_buffer: Box::new(img),
                }
            })
            .collect::<Vec<FileSystemPageProvider>>();

        pages.sort();

        Ok(Self {
            title: file_name,
            pages,
        })
    }

    fn from_rar(path: PathBuf) -> Result<Self, ProviderError> {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let path_string = path.to_str().unwrap().to_string();

        let temp_directory = tempfile::tempdir().unwrap().into_path();

        let mut pages: Vec<FileSystemPageProvider> = RarArchive::new(path_string.clone())
            .list()
            .unwrap()
            .process()
            .unwrap()
            .into_iter()
            .map(|entry| {
                let filename = entry.filename;
                let path = temp_directory.join(&filename);
                let img = image::io::Reader::open(path).unwrap().decode().unwrap();

                FileSystemPageProvider {
                    file_name: filename,
                    image_buffer: Box::new(img),
                }
            })
            .collect::<Vec<FileSystemPageProvider>>();

        pages.sort();

        RarArchive::new(path_string.clone())
            .extract_to(temp_directory.to_str().unwrap_or_default().to_string())
            .unwrap()
            .process()
            .unwrap();

        Ok(Self {
            title: file_name,
            pages,
        })
    }
}

impl ComicProvider for FileSystemComicProvider {
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_page(&self, index: usize) -> Option<&dyn PageProvider> {
        self.pages.get(index).map(|p| p as &dyn PageProvider)
    }

    fn get_length(&self) -> usize {
        self.pages.len()
    }
}

#[derive(Debug, Clone, Eq)]
pub struct FileSystemPageProvider {
    file_name: String,
    image_buffer: Box<image::DynamicImage>,
}

impl PageProvider for FileSystemPageProvider {
    fn get_image(&self) -> Option<&image::DynamicImage> {
        Some(&self.image_buffer)
    }

    fn get_file_name(&self) -> Result<String> {
        Ok(self.file_name.clone())
    }
}

impl Ord for FileSystemPageProvider {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_name.cmp(&other.file_name)
    }
}

impl PartialOrd for FileSystemPageProvider {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FileSystemPageProvider {
    fn eq(&self, other: &Self) -> bool {
        self.file_name == other.file_name
    }
}
