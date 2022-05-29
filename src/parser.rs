#[derive(Debug)]
pub struct Section {
    pub ident: String,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Content(String),
    SubSection(Section),
}
