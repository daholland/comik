pub mod file_system;

use std::sync::Arc;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum ProviderError {
    #[error("invalid archive type")]
    InvalidArchiveType,
}

pub trait CollectionProvider {
    fn get_name(&self) -> String;
    fn get_comic(&self, index: usize) -> Arc<Option<&dyn ComicProvider>>;
    fn get_size(&self) -> usize;
}

pub trait ComicProvider {
    fn open(&self) -> Result<()>;
    fn get_title(&self) -> String;
    fn get_page(&self, index: usize) -> Option<Box<dyn PageProvider>>;
    fn get_length(&self) -> usize;
}

pub trait PageProvider {
    fn get_image(&self) -> image::DynamicImage;
    fn get_file_name(&self) -> Result<String>;
}

pub trait ThumbnailProvider {
    fn get_thumbnail(&self, index: usize) -> image::DynamicImage;
    
}
