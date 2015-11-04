
#[derive(Clone, Copy, Debug)]
pub enum Format {
    Html,
    Latex,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub format: Format,
    pub with_header_and_footer: bool,
}

