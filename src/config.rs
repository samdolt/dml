
#[derive(Clone, Copy, Debug, RustcDecodable)]
pub enum Format {
    Html,
    Latex,
}

impl Default for Format {
    fn default() -> Format { Format::Html }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Config {
    pub format: Format,
    pub with_header_and_footer: bool,
}

