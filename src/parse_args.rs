use nom::{
    IResult,
    character::complete::*,
    bytes::complete::*,
};
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::multi::*;

use crate::{parentheses_comment, seimcolon_comment, Comment};

use super::{
    Arg,
    ArgOrComment,
};

type ArgOrCommentResult<'r> = IResult<&'r str, ArgOrComment<'r>>;
pub type ManyArgOrCommentsResult<'r> = IResult<&'r str, Option<Vec<ArgOrComment<'r>>>>;

/*
 * Conditionally parses a string arg if enabled is true.
 */
// #[inline(always)]
fn string_arg<'r>(input: &'r str) -> ArgOrCommentResult<'r> {
    map(
        preceded(
            space1,
            escaped(
                is_not("\n\r;("),
                '\\',
                one_of("(); \t\n\\"),
            ),
        ),
        |s| ArgOrComment::Arg(Arg::Text(s)),
    )(input)
}

// #[inline(always)]
fn key_value_arg<'r>(input: &'r str) -> ArgOrCommentResult<'r> {
    map(
        pair(
            verify(
                anychar,
                |c: &char| c.is_alphabetic(),
            ),
            opt(map_res(
                is_not(" \t\n\r;("),
                |s: &str| s.parse(),
            )),
        ),
        |(k, v): (char, Option<f32>)| {
            let arg = Arg::KeyValue((k.to_ascii_uppercase(), v));
            ArgOrComment::Arg(arg)
        },
    )(input)
}

fn combine_args_and_comments<'r>(
    parser_outputs: (Vec<ArgOrComment<'r>>, Option<&'r str>),
) -> Option<Vec<ArgOrComment<'r>>> {
    let (mut args_or_comments, final_comment) = parser_outputs;

    if let Some(final_comment) = final_comment {
        args_or_comments.push(
            ArgOrComment::Comment(Comment(final_comment)),
        );
    }

    if args_or_comments.is_empty() {
        None
    } else {
        Some(args_or_comments)
    }
}

// #[inline(always)]
pub fn parse_args<'r>(
    string_arg_mcode: bool,
    input: &'r str,
) -> ManyArgOrCommentsResult<'r> {
    if string_arg_mcode {
        // Add a Text argument for the string arg of certain MCodes (eg. M28 teg.gcode)
        map(
            tuple((
                many0(
                    alt((
                        map(parentheses_comment, |s| ArgOrComment::Comment(Comment(s))),
                        string_arg,
                    )),
                ),
                opt(seimcolon_comment),
            )),
            combine_args_and_comments,
        )(input)
    } else {
        map(
            tuple((
                many0(
                    alt((
                        map(parentheses_comment, |s| ArgOrComment::Comment(Comment(s))),
                        // Add the rest of the args and comments
                        preceded(
                            space1,
                            key_value_arg,
                        ),
                    )),
                ),
                opt(seimcolon_comment),
            )),
            combine_args_and_comments,
        )(input)
    }
}
