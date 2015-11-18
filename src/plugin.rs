use ::Format;
use regex::Regex;

pub trait DmlPlugin {

    fn get_named_block_processors() -> Vec<Box<NamedBlockProcessor>> {
        Vec::new()
    }

    fn get_anonymous_block_processors() -> Vec<Box<AnonymousBlockProcessor>> {
        Vec::new()
    }

    fn get_inline_processors() -> Vec<Box<InlineProcessor>> {
        Vec::new()
    }
}


pub trait AnonymousBlockProcessor {
    fn get_pattern(&self) -> String;
    fn process(&self, line: &str, to: Format) -> String;
}

pub trait NamedBlockProcessor {

}

pub trait InlineProcessor {
    fn process(&self, block: &str, to: Format) -> String;
}

