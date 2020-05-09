use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Elem {
    pub tag: String,
    pub id: Option<String>,
    pub classes: Option<Vec<String>>,
    pub attributes: Option<Vec<Attr>>,
    pub cont: Option<Cont>,
    pub children: Option<Vec<Elem>>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Attr {
    pub name: String,
    pub value: String,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Cont {
    LINE(String),
    BLOCK(Vec<String>),
}

impl<'a> Elem {

    ///Creates an empty element 
    pub fn default() -> Elem {
        Elem {
            tag: String::from(""),
            cont: None,
            id: None,
            classes: None,
            attributes: None,
            children: None,
        }
    }

    ///Creates an element from a tag
    pub fn from_ta(tag: &str) -> Elem {
        Elem {
            tag: String::from(tag),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag and a content line
    pub fn from_ta_col(tag: &str, cont: &str) -> Elem {
        Elem {
            tag: String::from(tag),
            cont: Some(Cont::LINE(cont.to_string())),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag and a content block
    pub fn from_ta_cob(tag: &str, cont: Vec<String>) -> Elem {
        Elem {
            tag: String::from(tag),
            cont: Some(Cont::BLOCK(cont)),
            ..Elem::default()
        }
    }
    ///Creates an element from a tag and child elements
    pub fn from_ta_ch(tag: &str, children: Vec<Elem>) -> Elem {
        Elem {
            tag: String::from(tag),
            children: Some(children),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag and classes
    pub fn from_ta_cl(tag: &str, classes: Vec<String>) -> Elem {
        Elem {
            tag: String::from(tag),
            classes: Some(classes),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag and an id
    pub fn from_ta_id(tag: &str, id: &str) -> Elem {
        Elem {
            tag: String::from(tag),
            id: Some(String::from(id)),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag, ids, and classes
    pub fn from_ta_id_cl(tag: &str, id: &str, classes: Vec<String>) -> Elem {
        Elem {
            tag: String::from(tag),
            classes: Some(classes),
            id: Some(String::from(id)),
            ..Elem::default()
        }
    }
    ///Creates an element from a tag, ids, classes, and content block
    pub fn from_ta_id_cl_cob(tag: &str, id: &str, classes: Vec<String>, content: Vec<String>) -> Elem {
        Elem {
            tag: String::from(tag),
            cont: Some(Cont::BLOCK(content)),
            classes: Some(classes),
            id: Some(String::from(id)),
            ..Elem::default()
        }
    }

    ///Creates an element from a tag, and attributes
    pub fn from_ta_at(tag: &str,  attributes: Vec<Attr>) -> Elem {
        Elem {
            tag: String::from(tag),
            attributes: Some(attributes),
            ..Elem::default()
        }
    }
}

impl Attr {
    pub fn default() -> Attr {
        Attr {name: String::new(), value: String::new()}
    }
}

impl fmt::Display for Cont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cont::LINE(text) => write!(f, "{}", text),
            Cont::BLOCK(texts) => {let mut res = String::new();
                                  for t in texts {
                                      res.push_str(t);
                                      res.push('\n');
                                  }
                                  write!(f, "{}", res)
            }
        }
    }
}
