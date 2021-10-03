use std::{cmp::Ordering, fs::File, io::Cursor, path::PathBuf, sync::Arc};

use anyhow::Result;
use compress_tools::{list_archive_files, uncompress_archive_file};
use image::io::Reader as ImageReader;

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
            .map(|path| FileSystemComicProvider::new(path.clone()).unwrap())
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

    fn get_comic(&self, index: usize) -> Arc<Option<&dyn ComicProvider>> {
        let comic = self.comics.get(index).map(|p| p as &dyn ComicProvider);

        Arc::new(comic)
    }

    fn get_size(&self) -> usize {
        self.comics.len()
    }
}

#[derive(Debug)]
pub struct FileSystemComicProvider {
    title: String,
    path: PathBuf,
    archive: File,
    file_list: Vec<String>,
}

impl FileSystemComicProvider {
    fn new(path: PathBuf) -> Result<Self, ProviderError> {
        match path.extension() {
            Some(ext) if ext == "zip" || ext == "cbz" || ext == "rar" || ext == "cbr" => {
                let title = path.file_name().unwrap().to_str().unwrap().to_string();

                let archive = File::open(path.clone()).unwrap();

                let mut file_list = list_archive_files(&archive).unwrap();
                file_list.sort();

                Ok(Self {
                    title,
                    path,
                    archive,
                    file_list,
                })
            }
            _ => Err(ProviderError::InvalidArchiveType),
        }
    }
}

impl ComicProvider for FileSystemComicProvider {
    fn open(&self) -> Result<()> {
        Ok(())
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_page(&self, index: usize) -> Option<Box<dyn PageProvider>> {
        let file_name = self.file_list.get(index).unwrap();

        let mut img_buffer = Vec::default();

        uncompress_archive_file(&self.archive, &mut img_buffer, file_name).unwrap();

        let image = ImageReader::new(Cursor::new(img_buffer))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap();

        let page_provider = FileSystemPageProvider {
            file_name: file_name.to_string(),
            image_buffer: image,
        };

        Some(Box::new(page_provider))
    }

    fn get_length(&self) -> usize {
        self.file_list.len()
    }
}

#[derive(Debug, Clone, Eq)]
pub struct FileSystemPageProvider {
    file_name: String,
    image_buffer: image::DynamicImage,
}

impl PageProvider for FileSystemPageProvider {
    fn get_image(&self) -> image::DynamicImage {
        self.image_buffer.clone()
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
