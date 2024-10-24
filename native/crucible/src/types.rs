// pub type Error = Box<dyn std::error::Error + Send + Sync>;

use crate::errors::CrucibleError;
pub type Error = CrucibleError;
pub type Result<T> = std::result::Result<T, Error> ;
