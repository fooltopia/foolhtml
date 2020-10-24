use std::{collections::BTreeMap};

///Holds a map from paths to html templates
///If you request a template via its path/name,
///and it's not in the map, the manager will look
///for a file at the path relative to the working directory. 
#[derive(Debug, Default)]
pub struct TemplateManager {
    templates: BTreeMap<String, String>
}

impl TemplateManager {
    pub fn add(self: &mut Self, path: String, template: String) {
        self.templates.insert(path.to_string(), template.to_string());
    }

    pub fn get(self: &mut Self, path: &str) -> Result<String> {
        match self.templates.get(path) {
            Some(t) => Ok(t.to_string()),
            None => read_template_file(path),
        }
    }
}

fn read_template_file(path: &str) -> Result<String> {
    use std::io::Read;
    match std::fs::File::open(path) {
        Ok(mut file) =>{ let mut contents = String::new();
                     file.read_to_string(&mut contents).unwrap();
                     Ok(contents) }
        Err(_) => Err(TemplateManagerError::TemplateNotFound)

    }
}

use thiserror::Error;

type Result<T, E = TemplateManagerError> = std::result::Result<T, E>;

/// WordCountError enumerates all possible errors returned by this library.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum TemplateManagerError {
    /// Represents an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    #[error("Could not find the requested template")]
    TemplateNotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_and_retrieve() {
        let mut manager = TemplateManager::default();
        manager.add("path".to_string(), "hello world".to_string());
        assert_eq!("hello world".to_string(), manager.get("path").unwrap());
    }

    #[test]
    fn returns_error_on_invalid_path() {
        let mut manager= TemplateManager::default();
        let expected = TemplateManagerError::TemplateNotFound;
        let result = manager.get("hello/").err().unwrap();
        assert_eq!(expected, result);
    }

}
