mod id;

#[derive(Debug)]
pub enum MainDeclaration {
    Id(String),
    Icon(String),
    Name(String),
    Version(String),
    Description(String),
    Link(String),
    Domain(String)
}