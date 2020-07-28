extern crate nom;

use thiserror::Error;

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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub struct Comment<'r>(&'r str);
// pub type Line = (Option<GCode>, Option<Vec<Comment>>);

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


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mnemonic {
    /// Preparatory commands, often telling the controller what kind of motion
    /// or offset is desired.
    General,
    /// Auxilliary commands.
    Miscellaneous,
    /// Used to give the current program a unique "name".
    ProgramNumber,
    /// Tool selection.
    ToolChange,
    /// O-Code: http://linuxcnc.org/docs/html/gcode/o-code.html
    Subroutine,
}

pub static G: Mnemonic = Mnemonic::General;
pub static M: Mnemonic = Mnemonic::Miscellaneous;
pub static P: Mnemonic = Mnemonic::ProgramNumber;
pub static T: Mnemonic = Mnemonic::ToolChange;
pub static O: Mnemonic = Mnemonic::Subroutine;

#[derive(Debug, PartialEq, Clone)]
pub enum ArgOrComment<'r> {
    Arg(Arg<'r>),
    Comment(Comment<'r>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arg<'r> {
    KeyValue(KeyValue),
    Flag(char),
    Text(&'r str),
}

#[derive(Debug, PartialEq, Clone)]
pub struct KeyValue {
    key: char,
    value: f32,
}

impl<'r> GCode<'r> {
    pub fn arguments(&self) -> impl Iterator<Item = &Arg> {
        use std::convert::identity;

        self.args_or_comments
            .iter()
            .flat_map(identity)
            .filter_map(|ac| {
                if let ArgOrComment::Arg(arg)  = ac {
                    Some(arg)
                } else {
                    None
                }
            })
    }
}
