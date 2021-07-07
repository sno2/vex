use nom::{
	character::complete::char as ch, character::complete::multispace0, combinator::map,
	multi::separated_list0, sequence::delimited, IResult,
};

use crate::Declaration;

/// A collection of declarations.
#[derive(Debug, PartialEq)]
pub struct DeclarationBlock<'a> {
	/// The declarations within the block.
	pub declarations: Vec<Declaration<'a>>,
}

impl<'a> DeclarationBlock<'a> {
	/// Parses a declaration block.
	pub fn parse(input: &'a str) -> IResult<&str, Self> {
		map(
			separated_list0(
				delimited(multispace0, ch(';'), multispace0),
				Declaration::parse,
			),
			|declarations| Self { declarations },
		)(input)
	}
}
