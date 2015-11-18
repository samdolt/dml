//! # DML library
//!
//! This library is used to parse Dml text file or to write new plugin for the
//! DML text processor.

#[macro_use]
extern crate log;

extern crate regex;


use std::io::BufReader;
use std::io::BufWriter;
use std::io;
use std::io::prelude::*;

mod plugin;
pub use plugin::AnonymousBlockProcessor;
pub use plugin::InlineProcessor;
pub use plugin::DmlPlugin;

mod title;
use title::TitlePlugin;

mod emphasis;
use emphasis::EmphasisPlugin;

mod templates;

mod config;
pub use config::Config;
pub use config::Format;


/// Process multiple block of text
pub fn process<R: Read, W: Write>(input: R, output: W, config: Config) -> Result<(), io::Error> {

    let in_buff = BufReader::new(input);
    let mut out_buff = BufWriter::new(output);

    // Check if we have to write some html header or latex header
    // and write it
    if config.with_header_and_footer {
        match config.format {
            Format::Html    => try!(write!(out_buff, "{}", templates::HTML_HEADER)),
            Format::Latex   => unimplemented!(),
        }
    }


    // Parse input file in a serie of block. Each bloc is passed directly
    // to the function process_block, so when the last bloc has been parsed,
    // the work is almost done
    let mut txt_block = String::with_capacity(1024);
    let mut in_named_block: bool = false;
    let mut in_anonymous_block: bool = false;
    let max_line_lenght = 80;

    for line in in_buff.lines() {

        let line: String = try!(line);
        let trimed_line = line.trim();

        trace!("PROCESS_LINE: {}", &line);

        // Spec says that the number of char in a line is max 80.
        if line.chars().count() > max_line_lenght {
            warn!("More than 80 chars in a line detected");
        }

        // We can't be in a anonymous block and a named block in the same time
        debug_assert!(!(in_named_block && in_anonymous_block));

        // Named block begin with --- and end with ---
        // This type of block must be checked first, because empty line is
        // allowed on a named bloc. (Used as a block separator otherwise)
        if in_named_block {
            if trimed_line == "---" {
                // We are on the end of a named block -> process_block
                // and clear in_named_block state
                in_named_block = false;
                try!(process_block(&txt_block, &mut out_buff, config));
                txt_block.clear();
            } else {
                // We are on the middle of a named block. Text are appended to
                // txt_block.
                txt_block.push_str(&line);
                txt_block.push_str("\n");
            }
        } else if in_anonymous_block {
            if trimed_line == "" {
                // We reached the end of the anonymous block
                in_anonymous_block = false;
                try!(process_block(&txt_block, &mut out_buff, config));
                txt_block.clear();
            } else {
                // We are in the middle of a anonymous block
                txt_block.push_str(&line);
                txt_block.push_str("\n");
            }
        } else {
            // We are not in a block, we need to check for a new block

            // txt_block should be empty here.
            debug_assert!(txt_block == "");

            if trimed_line.starts_with("--- ") {
                in_named_block = true;
            } else if trimed_line != "" {
                in_anonymous_block = true;
                txt_block.push_str(&line);
                txt_block.push_str("\n");
            }
        }
    }

    // Reading text file has finished, check for the last block
    if txt_block.trim() != "" {
        try!(process_block(&txt_block, &mut out_buff, config));
    }

    // Write footer if needed
    if config.with_header_and_footer {
        match config.format {
            Format::Html    => try!(write!(out_buff, "{}", templates::HTML_FOOTER)),
            Format::Latex   => unimplemented!(),
        }
    }

    Ok(())
}

/// Process a single block of text
///
/// # Examples
///
/// ```
/// use dml::Config;
/// use dml::process_block;
///
/// let input = "# Hello World";
/// let mut output = std::io::stdout();
/// let config = Config::default();
/// process_block(&input, &mut output, config);
/// ```
///
/// # Panics
///
/// In debug mode, a empty string input, or a string input with only space
/// cause a debug_assert panic.
///
/// # Failure
///
/// Return a failure if a IO read or write as failed
pub fn process_block<W: Write>(txt_block: &str, output: &mut W, config: Config) -> Result<(),io::Error> {

    let mut txt_block = txt_block.to_string();

    trace!("PROCESS_BLOCK: ---{}---", &txt_block);

    // This function only allow legal bloc
    debug_assert!(txt_block.trim() != "");

    if txt_block.lines().count() == 1 {
        let anonymous_processors = TitlePlugin::get_anonymous_block_processors();

        for plug in anonymous_processors.iter() {

            if txt_block.starts_with(&plug.get_pattern()){
                for _ in 0..plug.get_pattern().len() {
                    txt_block.remove(0);
                }
                try!(write!(output, "{}\n\n", plug.process(txt_block.trim(), config.format)));
                return Ok(());
            }
        }
    }

    // At least, it's a paragraph
    let inline_processor = EmphasisPlugin::get_inline_processors();

    let mut processed_block = txt_block.to_string();
    for plug in inline_processor {
        processed_block = plug.process(&processed_block, config.format);
    }

    match config.format {
        Format::Html  => try!(write!(output,"<p>\n{}\n</p>\n\n", processed_block.trim())),
        Format::Latex => try!(write!(output, "{}\n\n", processed_block.trim())),
    }

    Ok(())
}

