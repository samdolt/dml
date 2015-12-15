use AnonymousBlockProcessor;
use DmlPlugin;
use Format;

pub struct TitlePlugin;

impl DmlPlugin for TitlePlugin {
    fn get_anonymous_block_processors() -> Vec<Box<AnonymousBlockProcessor>> {
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
        TitleProcessing { level: level }
    }
}

impl AnonymousBlockProcessor for TitleProcessing {
    fn get_pattern(&self) -> String {
        let mut pattern = String::with_capacity(self.level + 1);
        for _ in 0..self.level {
            pattern.push('#');
        }

        pattern.push(' ');
        pattern
    }

    fn process(&self, line: &str, format: Format) -> String {
        match format {
            Format::Html => format!("<h{}>{}</h{}>", self.level, line, self.level),
            Format::Latex => {
                match self.level {

                    1 => format!("\\section{{{}}}", line),
                    2 => format!("\\subsection{{{}}}", line),
                    3 => format!("\\subsubsection{{{}}}", line),
                    4 => format!("\\paragraph{{{}}}", line),
                    5 => format!("\\subparagraph{{{}}}", line),
                    _ => line.to_string(),
                }
            }
        }
    }
}


#[test]
fn test_title_html() {
    let h1 = TitleProcessing::new(1);
    let h2 = TitleProcessing::new(2);
    let h3 = TitleProcessing::new(3);

    assert_eq!("# ", h1.get_pattern());
    assert_eq!("## ", h2.get_pattern());
    assert_eq!("### ", h3.get_pattern());

    assert_eq!("<h1>Hello</h1>", h1.process("Hello", Format::Html));
    assert_eq!("<h2>Hi World!</h2>", h2.process("Hi World!", Format::Html));
    assert_eq!("<h3>Hallo</h3>", h3.process("Hallo", Format::Html));
}


#[test]
fn test_title_latex() {
    let h1 = TitleProcessing::new(1);
    let h2 = TitleProcessing::new(2);
    let h3 = TitleProcessing::new(3);
    let h4 = TitleProcessing::new(4);
    let h5 = TitleProcessing::new(5);

    assert_eq!("# ", h1.get_pattern());
    assert_eq!("## ", h2.get_pattern());
    assert_eq!("### ", h3.get_pattern());

    assert_eq!("\\section{Hello}", h1.process("Hello", Format::Latex));
    assert_eq!("\\subsection{Hi World!}",
               h2.process("Hi World!", Format::Latex));
    assert_eq!("\\subsubsection{Hallo}", h3.process("Hallo", Format::Latex));
    assert_eq!("\\paragraph{h4}", h4.process("h4", Format::Latex));
    assert_eq!("\\subparagraph{h5}", h5.process("h5", Format::Latex));
}
