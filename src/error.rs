use thiserror::Error;

#[derive(Error, Debug)]
pub enum PixelatorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Processing error: {0}")]
    Processing(String),
}

pub type Result<T> = std::result::Result<T, PixelatorError>;