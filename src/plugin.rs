use ::Format;

pub trait DmlPlugin {
    fn get_oneline_processors() -> Vec<Box<OnelineProcessor>> {
        Vec::new()
    }

    fn get_block_processors()   -> Vec<Box<BlockProcessor>> {
        Vec::new()
    }

    fn get_inline_processors()  -> Vec<Box<InlineProcessor>> {
        Vec::new()
    }
}


pub trait OnelineProcessor {
    fn get_pattern(&self) -> String;
    fn process(&self, line: &str, to: Format) -> String;
}

pub trait BlockProcessor {

}

pub trait InlineProcessor {

}
