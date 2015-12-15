//! # DML library
//!
//! This library is used to parse Dml text file or to write new plugin for the
//! DML text processor.


use std::io::BufReader;
use std::io::BufWriter;
use std::io::Lines;
use std::io;
use std::io::prelude::*;
use std::iter;

use plugin::AnonymousBlockProcessor;
use plugin::InlineProcessor;
use plugin::DmlPlugin;

use plugin::TitlePlugin;
use plugin::EmphasisPlugin;
use templates;

use Config;
use Format;

struct Parser {
    out_format: Format,
}

struct BlockIterator<R>
    where R: Read
{
    lines: Lines<BufReader<R>>,
}

#[derive(Debug)]
enum BlockType {
    Anonymous,
    Named,
    Empty,
}

#[derive(Debug)]
pub struct Block {
    type_: BlockType,
    name: String,
    content: String,
    args: Vec<String>,
}

impl Block {
    fn new() -> Block {
        let content = String::with_capacity(1024);

        Block {
            type_: BlockType::Empty,
            name: String::with_capacity(25),
            content: content,
            args: Vec::new(),
        }
    }
}

impl<R> BlockIterator<R> where R: Read
{
    pub fn new(input: R) -> BlockIterator<R> {
        let buf = BufReader::new(input);
        let lines = buf.lines();
        BlockIterator { lines: lines }
    }
}

impl<R> Iterator for BlockIterator<R> where R: Read
{
    type Item = Block;
    fn next(&mut self) -> Option<Block> {
        trace!("BlockIterator.next called");

        let mut block = Block::new();
        let mut line = String::with_capacity(80);
        loop {
            let line = match self.lines.next() {
                Some(R) => R.expect("IO ERROR"),
                None => break,
            };

            trace!("Get new line {:?}", &line);

            match block.type_ {
                BlockType::Empty => {
                    if line.starts_with("--- ") {
                        // New Named block
                        block.type_ = BlockType::Named;
                        let args: Vec<&str> = line.split_whitespace().collect();

                        // args[0] is "---"
                        block.name = args[1].to_string();


                    } else if line.trim() != "" {
                        // New anonymous block
                        block.type_ = BlockType::Anonymous;
                        block.content.push_str(&line);

                        block.name = line.split_whitespace()
                                         .next()
                                         .expect("Anoynomous name")
                                         .to_string();
                    } else {
                        // Blank line, not in a block -> Do nothing
                    }
                }
                BlockType::Anonymous => {
                    if line.trim() == "" {
                        // End of block
                        break;
                    } else {
                        block.content.push_str(&line);
                    }
                }
                BlockType::Named => {
                    if line.starts_with("---") {
                        // End of block
                        break;
                    } else {
                        block.content.push_str(&line);
                    }
                }
            }
        }

        if block.content.trim() == "" {
            None
        } else {
            Some(block)
        }
    }
}


impl Parser {
    pub fn new(format: Format) -> Parser {
        Parser { out_format: format }
    }

    pub fn parse<R: Read, W: Write>(&self, input: R, output: W) -> io::Result<()> {
        Ok(())
    }
}
/// Process multiple block of text
pub fn process<R: Read, W: Write>(input: R, output: W, config: Config) -> Result<(), io::Error> {
    let mut out_buff = BufWriter::new(output);

    // Check if we have to write some html header or latex header
    // and write it
    if config.with_header_and_footer {
        match config.format {
            Format::Html => try!(write!(out_buff, "{}", templates::HTML_HEADER)),
            Format::Latex => unimplemented!(),
        }
    }

    let block_parser = BlockIterator::new(input);

    for block in block_parser {
        println!("======================");
        println!("{:?}", block);
    }

    // Write footer if needed
    if config.with_header_and_footer {
        match config.format {
            Format::Html => try!(write!(out_buff, "{}", templates::HTML_FOOTER)),
            Format::Latex => unimplemented!(),
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
pub fn process_block<W: Write>(txt_block: &str,
                               output: &mut W,
                               config: Config)
                               -> Result<(), io::Error> {

    let mut txt_block = txt_block.to_string();

    trace!("PROCESS_BLOCK: ---{}---", &txt_block);

    // This function only allow legal bloc
    debug_assert!(txt_block.trim() != "");

    if txt_block.lines().count() == 1 {
        let anonymous_processors = TitlePlugin::get_anonymous_block_processors();

        for plug in anonymous_processors.iter() {

            if txt_block.starts_with(&plug.get_pattern()) {
                for _ in 0..plug.get_pattern().len() {
                    txt_block.remove(0);
                }
                try!(write!(output,
                            "{}\n\n",
                            plug.process(txt_block.trim(), config.format)));
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
        Format::Html => try!(write!(output, "<p>\n{}\n</p>\n\n", processed_block.trim())),
        Format::Latex => try!(write!(output, "{}\n\n", processed_block.trim())),
    }

    Ok(())
}
