use std::fmt::{Debug, Display};
use ariadne::{Cache, Source};

pub mod ir;
pub mod parser;
mod parser2;

pub struct FileHolder {
    pub(crate) files: Vec<Source>,
}

impl FileHolder {
    pub fn from(file: String) -> Self {
        Self {
            files: vec![
                Source::from(file.as_str())
            ]
        }
    }
}

impl Cache<usize> for FileHolder {
    fn fetch(&mut self, id: &usize) -> Result<&Source, Box<dyn Debug + '_>> {
        let file = self.files.get(*id).unwrap();
        Ok(file)
    }

    fn display<'a>(&self, id: &'a usize) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(format!("file_id: {}", *id)))
    }
}