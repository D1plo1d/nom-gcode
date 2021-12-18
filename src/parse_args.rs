use nom::{
    IResult,
    character::complete::*,
    bytes::complete::*,
};
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::multi::*;
use nom::AsChar;

use crate::{seimcolon_comment, Comment, comment};

use super::{
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
        ArgOrComment::TextArg
    )(input)
}

#[inline(always)]
fn key_value_arg<'r>(input: &'r str) -> ArgOrCommentResult<'r> {
    map(
        pair(
            satisfy(AsChar::is_alpha),
            opt(map_res(
                is_not(" \t\n\r;("),
                |s: &str| s.parse(),
            )),
        ),
        ArgOrComment::KeyValue,
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
                    opt(map(comment, |c| ArgOrComment::Comment(c))),
                    opt(string_arg),
                    opt(map(comment, |c| ArgOrComment::Comment(c))),
                ),
            ),
            |(c1, arg, c2)| Some(vec![c1, arg, c2].into_iter().flatten().collect()),
        )(input)
    } else {
        map(
            tuple((
                many0(
                    alt((
                        // Add the rest of the args and comments
                        preceded(
                            space1,
                            key_value_arg,
                        ),
                        map(comment, |c| ArgOrComment::Comment(c)),
                    )),
                ),
                opt(seimcolon_comment),
            )),
            combine_args_and_comments,
        )(input)
    }
}


// #[inline(always)]
pub fn parse_kv_arg<'r>(
    input: &'r str,
) -> ArgOrCommentResult<'r> {
    // if string_arg_mcode {
    //     // Add a Text argument for the string arg of certain MCodes (eg. M28 teg.gcode)
    //     alt((
    //         string_arg,
    //         map(comment, |c| ArgOrComment::Comment(c)),
    //     ))(input)
    // } else {
    alt((
        // Add the rest of the args and comments
        preceded(
            space1,
            key_value_arg,
        ),
        map(comment, ArgOrComment::Comment),
    ))(input)
}
