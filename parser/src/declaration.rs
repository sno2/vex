use nom::{
	bytes::complete::{tag, take_till, take_until},
	combinator::map,
	sequence::{pair, terminated},
	IResult,
};

// TODO(sno2): handle for semi-colons within the value of a declaration (string
// value)

/// A property and its value within a ruleset.
#[derive(Debug, PartialEq)]
pub struct Declaration<'a> {
	/// The name of the declaration.
	pub name: &'a str,
	/// The value of the declaration without modification (ex. whitespace
	/// trimming).
	pub value: &'a str,
}

impl<'a> Declaration<'a> {
	/// Parses the given text into a declaration.
	/// ## Notes
	/// - This does not account for preceding and trailing whitespace.
	///   Therefore, you should wrap this function within a [`delimited`] with
	///   something like [`multispace0`] around it.
	pub fn parse(input: &'a str) -> IResult<&str, Self> {
		let (i, decl) = map(
			pair(
				terminated(take_until(":"), tag(":")),
				take_till(|c| c == ';' || c == '}'),
			),
			|(name, value)| Self { name, value },
		)(input)?;

		if decl.name.contains("}") {
			return Err(nom::Err::Error(nom::error::Error {
				input: decl.name,
				code: nom::error::ErrorKind::IsNot,
			}));
		}

		Ok((i, decl))
	}
}
