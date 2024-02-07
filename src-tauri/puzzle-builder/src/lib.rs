pub mod error;
pub mod game;
pub mod image_splitter;
pub mod plugin;
pub(crate) mod utils;

pub(crate) type Result<T, E = error::Error> = std::result::Result<T, E>;

pub use plugin::get_plugin;
