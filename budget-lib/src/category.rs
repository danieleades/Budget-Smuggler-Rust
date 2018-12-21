use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Clone, Debug, Default)]
pub struct Category {
    fields: Vec<String>,
}

impl Category {
    pub fn tokens(&self) -> std::slice::Iter<std::string::String> {
        self.fields.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub(crate) fn take_top_category(&mut self) -> Option<String> {
        match self.fields.len() {
            0 => None,
            _ => Some(self.fields.remove(0)),
        }
    }

    pub(crate) fn add_subcategory<S: Into<String>>(&mut self, s: S) {
        self.fields.push(s.into());
    }

    pub(crate) fn with_subcategory<S: Into<String>>(mut self, s: S) -> Self {
        self.add_subcategory(s);
        self
    }

    pub(crate) fn depth(&self) -> usize {
        self.fields.len()
    }
}

impl std::ops::Index<usize> for Category {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

impl FromStr for Category {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            0 => Err("string is empty!"),
            _ => {
                let fields: Vec<String> = s.split("::").map(str::to_owned).collect();
                Ok(Category { fields })
            }
        }
    }
}

impl From<Category> for Option<String> {
    fn from(c: Category) -> Option<String> {
        match c.fields.len() {
            0 => None,
            _ => Some(collect_tokens(c.fields.into_iter(), "::".to_string())),
        }
    }
}

fn collect_tokens<S: AsRef<str>>(mut tokens: impl Iterator<Item = S>, sep: S) -> String {
    let mut sentence = "".to_string();
    if let Some(x) = tokens.next() {
        sentence.push_str(x.as_ref());
    }
    for token in tokens {
        sentence += &format!("{}{}", sep.as_ref(), token.as_ref())
    }
    sentence
}
