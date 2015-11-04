use std::io::BufReader;
use std::io::BufWriter;
use std::io;
use std::io::prelude::*;

mod plugin;
pub use plugin::OnelineProcessor;
pub use plugin::DmlPlugin;

mod title;
use title::TitlePlugin;

mod templates;

mod config;
pub use config::Config;
pub use config::Format;


pub fn process<R: Read, W: Write>(input: R, output: W, config: Config) -> Result<(), io::Error> {

    let in_buff = BufReader::new(input);
    let mut out_buff = BufWriter::new(output);

    if config.with_header_and_footer {
        match config.format {
            Format::Html    => try!(write!(out_buff, "{}", templates::HTML_HEADER)),
            Format::Latex   => unimplemented!(),
        }
    }

    let mut txt_block = String::new();

    for line in in_buff.lines() {

        let line = try!(line);

        if line.trim() == "" {
            try!(process_block(&txt_block, &mut out_buff, config));
            txt_block.clear();
        } else {
            txt_block.push_str(&line);
            txt_block.push_str("\n");
        }
    }

    try!(process_block(&txt_block, &mut out_buff, config));


    if config.with_header_and_footer {
        match config.format {
            Format::Html    => try!(write!(out_buff, "{}", templates::HTML_FOOTER)),
            Format::Latex   => unimplemented!(),
        }
    }

    Ok(())
}

fn process_block<W: Write>(txt_block: &str, output: &mut W, config: Config) -> Result<(),io::Error> {

    let mut txt_block = txt_block.to_string();
    if txt_block.trim() == "" {
        return Ok(());
    }

    if txt_block.lines().count() == 1 {
        let oneline_processors = TitlePlugin::get_oneline_processors();

        for plug in oneline_processors.iter() {

            if txt_block.starts_with(&plug.get_pattern()){
                for _ in 0..plug.get_pattern().len() {
                    txt_block.remove(0);
                }
                try!(write!(output, "{}\n\n", plug.process(txt_block.trim(), config.format)));
                return Ok(());
            }
        }
    }

    match config.format {
        Format::Html  => try!(write!(output,"<p>\n{}\n</p>\n\n", txt_block.trim())),
        Format::Latex => try!(write!(output, "{}\n\n", txt_block.trim())),
    }

    Ok(())
}

