#[derive(Debug)]
pub struct Section {
    pub ident: String,
    pub items: Vec<Item>,
    pub ignore: bool,   // true if ignore
}

#[derive(Debug)]
pub enum Item {
    Content(String),
    SubSection(Section),
}
