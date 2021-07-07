use nom::IResult;

/// A wrapper for the content of a selector. **Currently not implemented for
/// parsing selectors.**
#[derive(Debug, PartialEq)]
pub struct Selector<'a>(pub &'a str);

impl<'a> Selector<'a> {
	/// Parses the given selector.
	pub fn parse(input: &'a str) -> IResult<&str, Self> {
		Ok(("", Self(input)))
	}
}
