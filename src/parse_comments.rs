use nom::{
    IResult,
    character::complete::*,
    bytes::complete::*,
};
use nom::branch::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::multi::*;
use std::time::Duration;

use super::{
    Comment,
    DocComment,
};

// #[inline(always)]
pub fn parentheses_comment<'r>(input: &'r str) -> IResult<&'r str, &'r str> {
    let parser = preceded(
        char('('),
        is_not("\n\r)"),
    );

    terminated(
        parser,
        char(')'),
    )(input)
}

pub struct WithComments<'r, O> {
    pub comments: Option<Vec<&'r str>>,
    pub value: O,
}

// #[inline(always)]
pub fn with_parentheses_comments<
    'r,
    T: FnMut(&'r str,) -> IResult<&'r str, O>,
    O,
>(
    parser: T,
) -> impl FnMut(&'r str,) -> IResult<&'r str, WithComments<'r, O>> {
    let parser = pair(
        parser,
        opt(many1(parentheses_comment)),
    );

    let parser = pair(
        opt(many1(parentheses_comment)),
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

// #[inline(always)]
pub fn seimcolon_comment<'r>(input: &'r str,) -> IResult<&'r str, &'r str> {
    preceded(
        char(';'),
        not_line_ending,
    )(input)
}

// #[inline(always)]
pub fn comment<'r>(input: &'r str) -> IResult<&'r str, Comment<'r>> {
    map(
        alt((seimcolon_comment, parentheses_comment)),
        |comment| Comment(comment),
    )(input)
}

// #[inline(always)]
pub fn doc_comment<'r>(input: &'r str) -> IResult<&'r str, DocComment<'r>> {
    map_opt(
        preceded(
            char(';'),
            separated_pair(
                take_until(":"),
                char(':'),
                preceded(
                    space0,
                    not_line_ending,
                ),
            ),
        ),
        |(key, value)| {
            let doc = match key {
                "FLAVOR" => DocComment::GCodeFlavor(value),
                "TIME" => DocComment::PrintTime(Duration::from_secs(value.parse().ok()?)),
                "Filament used" => filament_used(value).ok()?.1,
                "Layer height" => DocComment::LayerHeight { millis: value.parse().ok()? },
                _ => return None
            };

            Some(doc)
        }
    )(input)
}

// #[inline(always)]
pub fn filament_used<'r>(input: &'r str,) -> IResult<&'r str, DocComment<'r>> {
    map_opt(
        terminated(
            take_until("m"),
            char('m'),
        ),
        |s: &'r str| {
            let meters = s.parse().ok()?;
            Some(DocComment::FilamentUsed { meters })
        }
    )(input)
}
