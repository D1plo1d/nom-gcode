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
    Comment,
};

pub fn parse_parentheses_comment<'r>() -> impl FnMut(&'r str,) -> IResult<&'r str, &'r str> {
    let parser = preceded(
        char('('),
        is_not("\n\r)"),
    );

    terminated(
        parser,
        char(')'),
    )
}

pub struct WithComments<'r, O> {
    pub comments: Option<Vec<&'r str>>,
    pub value: O,
}

pub fn with_parentheses_comments<
    'r,
    T: FnMut(&'r str,) -> IResult<&'r str, O>,
    O,
>(
    parser: T,
) -> impl FnMut(&'r str,) -> IResult<&'r str, WithComments<'r, O>> {
    let parser = pair(
        parser,
        opt(many1(parse_parentheses_comment())),
    );

    let parser = pair(
        opt(many1(parse_parentheses_comment())),
        parser,
    );

    map(
        parser,
        |(mut comments, (value, more_comments))| {
            // Merge all comments into one optional vec
            if let Some(more_comments) = more_comments {
                comments = Some([
                    comments.unwrap_or_else(|| vec![]),
                    more_comments,
                ].concat())
            }

            WithComments {
                comments,
                value,
            }
        }
    )
}

pub fn parse_seimcolon_comment<'r>() -> impl FnMut(&'r str,) -> IResult<&'r str, &'r str> {
    preceded(
        char(';'),
        not_line_ending,
    )
}

pub fn parse_comments<'r>() -> impl FnMut(&'r str,) -> IResult<&'r str, Option<Vec<Comment>>> {
    opt(many1(map(
        alt((parse_seimcolon_comment(), parse_parentheses_comment())),
        |s: &str| Comment(s),
    )))
}

pub fn any_comment<'r>() -> impl FnMut(&'r str,) -> IResult<&'r str, Comment<'r>> {
    map(
        alt((parse_seimcolon_comment(), parse_parentheses_comment())),
        |comment| Comment(comment),
    )
}
