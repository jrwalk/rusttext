pub mod loader;
pub mod vocabulary;
pub mod word;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
