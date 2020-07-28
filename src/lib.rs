extern crate nom;

use std::fmt;
use thiserror::Error;

mod mnemonic;
pub use mnemonic::*;

mod parse_command;
pub use parse_command::parse_command;

mod parse_args;
pub use parse_args::parse_args;

mod parse_comments;
pub use parse_comments::*;

mod parse_gcode;
pub use parse_gcode::parse_gcode;

#[derive(Error, Debug)]
pub enum GCodeParseError {
    #[error("Invalid GCode. GCodes must start with a letter, a number and a space. Got: {0}")]
    InvalidGCode(String),
    #[error("Badly formatted GCode arguments. Got: {0}")]
    InvalidArguments(String),
    #[error("Badly formatted GCode comment. Got: {0}")]
    InvalidComment(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Comment<'r>(&'r str);

#[derive(Debug, PartialEq, Clone)]
pub enum GCodeLine<'r> {
    //// The first non-blank line of a file may contain nothing but a percent sign, %, possibly 
    /// surrounded by white space, and later in the file (normally at the end of the file) there 
    /// may be a similar line.
    /// 
    /// http://linuxcnc.org/docs/html/gcode/overview.html
    FileDemarcator,
    GCode(GCode<'r>),
    Comment(Comment<'r>),
}

impl<'r> From<Comment<'r>> for GCodeLine<'r> {
    fn from(comment: Comment<'r>) -> Self {
        GCodeLine::Comment(comment)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GCode<'r> {
    pub line_number: Option<u32>,
    pub mnemonic: Mnemonic,
    pub major: u32,
    pub minor: u32,
    args_or_comments: Option<Vec<ArgOrComment<'r>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArgOrComment<'r> {
    Arg(Arg<'r>),
    Comment(Comment<'r>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arg<'r> {
    KeyValue(KeyValue),
    Text(&'r str),
}

pub type KeyValue = (char, Option<f32>);

impl<'r> fmt::Display for GCode<'r> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let gcode = format!("{}{}.{}", self.mnemonic, self.major, self.minor);

        let mut words = vec![gcode];

        let arg_words = self.arguments()
            .map(|(k, v)| {
                format!("{}{}", k, v.map(|f| f.to_string()).unwrap_or("".to_string()))
            });

        words.extend(arg_words);

        if let Some(text) = self.text() {
            words.push(text.to_string());
        };

        write!(f, "{}", words.join(" "))
    }
}

impl<'r> GCode<'r> {
    fn args_or_comments_iter(&self) -> impl Iterator<Item = &ArgOrComment<'r>> {
        use std::convert::identity;

        self.args_or_comments
            .iter()
            .flat_map(identity)
    }

    pub fn text(&self) -> Option<&'r str> {
        self.args_or_comments_iter()
            .find_map(|ac| {
                if let ArgOrComment::Arg(Arg::Text(text)) = ac {
                    Some(*text)
                } else {
                    None
                }
            })
    }

    pub fn arguments(&self) -> impl Iterator<Item = &KeyValue> {
        self.args_or_comments_iter()
            .filter_map(|ac| {
                if let ArgOrComment::Arg(Arg::KeyValue(arg))  = ac {
                    Some(arg)
                } else {
                    None
                }
            })
    }
}
