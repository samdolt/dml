//! # DML library
//!
//! This library is used to parse Dml text file or to write new plugin for the
//! DML text processor.

#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate regex;

extern crate docopt;


use std::io::BufReader;
use std::io::BufWriter;
use std::io;
use std::io::prelude::*;

mod plugin;
pub use plugin::AnonymousBlockProcessor;
pub use plugin::InlineProcessor;
pub use plugin::DmlPlugin;

use plugin::TitlePlugin;
use plugin::EmphasisPlugin;

mod templates;

mod config;
pub use config::Config;
pub use config::Format;

mod project;

pub mod command;
pub use command::Command;

pub mod helpers;
pub mod parser;


