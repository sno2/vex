use nom::{
	branch::alt,
	bytes::complete::{tag, take_until},
	character::complete::multispace1,
	character::complete::{char as ch, multispace0},
	combinator::map,
	sequence::{delimited, pair, terminated, tuple},
	IResult,
};

use crate::Scope;

/// The at-rules available within CSS.
#[derive(Debug, PartialEq)]
pub enum AtRule<'a> {
	/// The `@charset` at-rule.
	/// ## Notes
	/// - Spec says that this should not have whitespace between the at-rule and
	///   charset along with the the charset string and the semi-colon. However,
	///   we'll just allow it happen to make it easier to use (the prettier can
	///   fix it).
	Charset(&'a str),
	/// The `@media` at-rule.
	Media {
		/// The condition within the media rule.
		condition: &'a str,
		/// The scope of styles within the rule.
		scope: Scope<'a>,
	},
}

impl<'a> AtRule<'a> {
	/// Parses a generic at-rule.
	pub fn parse(input: &'a str) -> IResult<&str, Self> {
		alt((Self::parse_media, Self::parse_charset))(input)
	}

	/// Parses into an [`AtRule::Charset`].
	pub fn parse_charset(input: &'a str) -> IResult<&str, Self> {
		map(
			delimited(
				tuple((tag("@charset"), multispace1, ch('"'))),
				take_until("\""),
				tuple((ch('"'), multispace0, ch(';'))),
			),
			|charset| Self::Charset(charset),
		)(input)
	}

	/// Parses into an [`AtRule::Media`].
	pub fn parse_media(input: &'a str) -> IResult<&str, Self> {
		map(
			pair(
				delimited(tag("@media"), take_until("{"), ch('{')),
				terminated(Scope::parse, ch('}')),
			),
			|(condition, scope)| Self::Media { condition, scope },
		)(input)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn charset() {
		let (_, rule) = AtRule::parse_charset("@charset    \"UTF-8\"  ;").unwrap();
		assert_eq!(rule, AtRule::Charset("UTF-8"));

		AtRule::parse_charset("@charset UTF-8")
			.expect_err("Quotes are required around the charset.");
		AtRule::parse_charset("\t@charset \"foo\";").expect_err("Whitespace before @ rule.");
		AtRule::parse_charset("@charset 'foo';")
			.expect_err("Using single quotes instead of double quotes.");
	}
}
