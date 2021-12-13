use nom::{
    IResult,
    combinator::*,
    sequence::*,
    character::complete::{self as character, *},
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
pub fn parse_command<'r>(input: &'r str) -> IResult<&'r str, GCode<'r>> {
    map_res(
        tuple((
            // Line Number
            opt(delimited(char('N'), character::u32, space1)),
            // Mnemonic
            one_of("GMPTO"),
            // Major Version
            character::u32,
            opt(preceded(char('.'), character::u32))
        )),
        |values| {
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
        },
    )(input)
}
