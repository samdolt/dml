use ::OnelineProcessor;
use ::DmlPlugin;

pub struct TitlePlugin;

impl DmlPlugin for TitlePlugin {
    fn get_oneline_processors() -> Vec<Box<OnelineProcessor>> {
        vec!(
            Box::new(TitleProcessing::new(1)),
            Box::new(TitleProcessing::new(2)),
            Box::new(TitleProcessing::new(3)),
            Box::new(TitleProcessing::new(4)),
            Box::new(TitleProcessing::new(5)),
            Box::new(TitleProcessing::new(6)),
        )
    }
}

#[derive(Copy, Clone)]
pub struct TitleProcessing {
    level: usize,
}

impl TitleProcessing {

    pub fn new(level: usize) -> TitleProcessing {
        TitleProcessing {
            level: level,
        }
    }

}

impl OnelineProcessor for TitleProcessing {

    fn get_pattern(&self) -> String {
        let mut pattern = String::with_capacity(self.level + 1);
        for _ in 0..self.level {
            pattern.push('#');
        }

        pattern.push(' ');
        pattern
    }

    fn process(&self, line: &str) -> String {
        format!("<h{}>{}</h{}>", self.level, line, self.level)
    }

}


#[test]
fn test_title() {
    let h1 = TitleProcessing::new(1);
    let h2 = TitleProcessing::new(2);
    let h3 = TitleProcessing::new(3);

    assert_eq!("# ", h1.get_pattern());
    assert_eq!("## ", h2.get_pattern());
    assert_eq!("### ", h3.get_pattern());

    assert_eq!("<h1>Hello</h1>", h1.process("Hello"));
    assert_eq!("<h2>Hi World!</h2>", h2.process("Hi World!"));
    assert_eq!("<h3>Hallo</h3>", h3.process("Hallo"));
}
