//! URI: Parser
//! Parse URI values from strings

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::{pair, tuple};
use nom::IResult;
use std::str::FromStr;

use super::error::{Error, ErrorParse};
use super::{Path, PathSegment, Scheme, URI};

/// Parse a URI
pub fn uri_from_string(uri_string: &str) -> Result<URI, Error> {
    let result = tuple((scheme, tag("://"), path))(uri_string);

    match result {
        Err(err) => {
            return Err(Error::Parse(ErrorParse {
                parse_error: err.to_string(),
            }))
        }
        Ok((_rest, parts)) => {
            return Ok(URI {
                scheme: parts.0,
                path: parts.2,
            })
        }
    }
}

/// Schema from string
fn scheme(input: &str) -> IResult<&str, Scheme> {
    let (rest, scheme_str) = alt((tag("file"), tag("http")))(input)?;
    return Ok((rest, Scheme::from_str(scheme_str).unwrap()));
}

/// Parse a PathSegment
fn path_segment(input: &str) -> IResult<&str, PathSegment> {
    let (rest, (_, segment_str)) = pair(char('/'), take_while(|ch| ch != '/'))(input)?;
    return Ok((rest, PathSegment(segment_str.to_string())));
}

/// Parse a Path
fn path(input: &str) -> IResult<&str, Path> {
    let (rest, segments) = many0(path_segment)(input)?;
    return Ok((rest, Path { segments: segments }));
}
