use ::InlineProcessor;
use ::DmlPlugin;
use ::Format;
use regex::Regex;

pub struct EmphasisPlugin;

impl DmlPlugin for EmphasisPlugin {
    fn get_inline_processors() -> Vec<Box<InlineProcessor>> {
        vec!(
            Box::new(EmphasisProcessing::new()),
        )
    }
}

#[derive(Copy, Clone)]
pub struct EmphasisProcessing;

impl EmphasisProcessing {

    pub fn new() -> EmphasisProcessing {
        EmphasisProcessing
    }

}

impl InlineProcessor for EmphasisProcessing {

    fn process(&self, block: &str, format: Format) -> String {

        let strong_re = Regex::new(r"\*{2}(?P<t>[^\*]+)\*{2}").unwrap();
        let weak_re = Regex::new(r"\*(?P<t>[^\*]+)\*").unwrap();

        let mut result: String;
        match format {
            Format::Html    =>  {
                    result = strong_re.replace_all(&block, "<strong>$t</strong>");
                    result = weak_re.replace_all(&result, "<em>$t</em>");
                },
            Format::Latex   => unimplemented!()
        }
        result.to_string()
    }
}


#[test]
fn test_emphasis_html() {
    let em = EmphasisProcessing::new();

    assert_eq!("<em>Hello</em>", em.process("*Hello*", Format::Html));
    assert_eq!("<strong>Hallo</strong>", em.process("**Hallo**", Format::Html));

    assert_eq!("<em>Hello <strong>World</strong> !</em>",
               em.process("*Hello **World** !*", Format::Html)
    );
}
