use nom::{
    IResult,
    combinator::*,
    sequence::*,
    character::complete::*,
    Err,
    error::ErrorKind,
};

use super::{
    GCode,
    G,
    M,
    P,
    T,
    O,
};

// #[inline(always)]
pub fn parse_command<'r>() -> impl FnMut(&'r str,) ->  IResult<&'r str, GCode<'r>> {
    let number = || {
        map_res(
            digit1,
            |s: &str| s.parse(),
        )
    };

    let parser = tuple((
        // Line Number
        opt(delimited(char('N'), number(), space1)),
        // Mnemonic
        one_of("GMPTO"),
        // Major Version
        number(),
        opt(preceded(char('.'), number()))
    ));

    let parser = map_res(parser, |values| {
        let (line_number, mnemonic, major, minor) = values;

        let mnemonic = match mnemonic.to_ascii_uppercase() {
            'G' => G,
            'M' => M,
            'P' => P,
            'T' => T,
            'O' => O,
            _ => {
                return Err(Err::Error(("Invalid Mnemonic", ErrorKind::MapRes)))
            },
        };

        let gcode = GCode {
            line_number,
            mnemonic,
            major,
            minor: minor.unwrap_or(0),
            args_or_comments: None,
        };

        Ok(gcode)
    });

    parser
}
