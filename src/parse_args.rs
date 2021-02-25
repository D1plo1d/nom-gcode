use nom::{
    IResult,
    character::complete::*,
    bytes::complete::*,
};
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::multi::*;


use super::{
    Arg,
    ArgOrComment,
    comment,
};

type ArgOrCommentResult<'r> = IResult<&'r str, ArgOrComment<'r>>;
pub type ManyArgOrCommentsResult<'r> = IResult<&'r str, Option<Vec<ArgOrComment<'r>>>>;

fn arg_comment<'r>(
) -> impl FnMut(&'r str,) -> ArgOrCommentResult<'r> {
    preceded(
        space0,
        map(
            comment,
            |c| ArgOrComment::Comment(c)
        )
    )
}

fn many_arg_comments<'r>(
) -> impl FnMut(&'r str,) -> ManyArgOrCommentsResult<'r> {
    opt(many1(arg_comment()))
}

/*
 * Conditionally parses a string arg if enabled is true.
 */
fn opt_string_arg<'r>(
    enabled: bool,
) -> impl FnMut(&'r str,) -> ManyArgOrCommentsResult<'r> {
    let parser = cond(
        enabled,
        opt(preceded(
            space1,
            escaped(
                is_not("\n\r;("),
                '\\',
                one_of("(); \t\n\\"),
            ),
        ))
    );

    let parser = map(parser, |filename| {
        filename.flatten().map(|f| {
            vec![ArgOrComment::Arg(Arg::Text(f))]
        })
    });

    parser
}

fn key_value_arg<'r>(
) -> impl FnMut(&'r str,) -> ArgOrCommentResult<'r> {
    let alphabetical_char = verify(
        anychar,
        |c: &char| c.is_alphabetic(),
    );
    let ascii_f32 = map_res(
        is_not(" \t\n\r;("),
        |s: &str| s.parse(),
    );

    map(
        pair(alphabetical_char, ascii_f32),
        |(k, v): (char, f32)| {
            let arg = Arg::KeyValue((k.to_ascii_uppercase(), Some(v)));
            ArgOrComment::Arg(arg)
        },
    )
}

fn flag_arg<'r>(
) -> impl FnMut(&'r str,) -> ArgOrCommentResult<'r> {
    let alphabetical_char = verify(
        anychar,
        |c: &char| c.is_alphabetic(),
    );

    map(
        alphabetical_char,
        |k| {
            let arg = Arg::KeyValue((k.to_ascii_uppercase(), None));
            ArgOrComment::Arg(arg)
        },
    )
}

pub fn parse_args<'r>(
    string_arg_mcode: bool,
) -> impl FnMut(&'r str,) -> ManyArgOrCommentsResult<'r> {
    let parser = tuple((
        many_arg_comments(),
        // Add the rest of the args and comments
        opt(cond(
            !string_arg_mcode,
            many1(alt((
                preceded(
                    space1,
                    alt((key_value_arg(), flag_arg())),
                ),
                arg_comment(),
            ))),
        )),
        // Add a Text argument for the string arg of certain MCodes (eg. M28 teg.gcode)
        opt_string_arg(string_arg_mcode),
        many_arg_comments(),
    ));

    let parser = map(
        parser,
        |(t0, t1, t2, t3)| {
            let args_or_comments = vec![t0, t1.flatten(), t2, t3];
            let mut iter = args_or_comments
                .into_iter()
                .filter_map(|ac| ac)
                .flatten()
                .peekable();

            if iter.peek().is_none() {
                None
            } else {
                Some(iter.collect())
            }
        },
    );

    parser
}
