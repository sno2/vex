use nom::{
	bytes::complete::take_until,
	character::complete::{char as ch, multispace0},
	combinator::{map, opt},
	sequence::{delimited, pair, terminated, tuple},
	IResult,
};

use crate::{DeclarationBlock, Selector};

/// A selector and the style declarations that should apply to elements that
/// qualify under that selector.
#[derive(Debug, PartialEq)]
pub struct RuleSet<'a> {
	/// The raw, unmodified contents of the selector.
	pub selector: Selector<'a>,
	/// The declaration block with the properties and values.
	pub block: DeclarationBlock<'a>,
}

impl<'a> RuleSet<'a> {
	/// Parses the given input into a [`RuleSet`] with the selector and
	/// declarations.
	pub fn parse(input: &'a str) -> IResult<&str, Self> {
		map(
			tuple((
				terminated(take_until("{"), ch('{')),
				delimited(
					multispace0,
					DeclarationBlock::parse,
					pair(opt(pair(ch(';'), multispace0)), ch('}')),
				),
			)),
			|(selector, block)| Self {
				selector: Selector(selector),
				block,
			},
		)(input)
	}
}

#[cfg(test)]
mod tests {
	use crate::Declaration;

	use super::*;

	#[test]
	fn trailing_semi_in_ruleset() {
		let (_, rule_set) = RuleSet::parse("h1 { color:blue;text-decoration: grey; }").unwrap();
		assert_eq!(rule_set.selector, Selector("h1 "));
		assert_eq!(
			rule_set.block.declarations,
			vec![
				Declaration {
					name: "color",
					value: "blue",
				},
				Declaration {
					name: "text-decoration",
					value: " grey",
				},
			]
		);
	}

	#[test]
	fn no_trailing_semi_in_ruleset() {
		const INPUT: &str = "h1 {
			color:blue;
			text-decoration: grey
		}";
		let (_, rule_set) = RuleSet::parse(INPUT).unwrap();
		assert_eq!(rule_set.selector, Selector("h1 "));
		assert_eq!(
			rule_set.block.declarations,
			vec![
				Declaration {
					name: "color",
					value: "blue",
				},
				Declaration {
					name: "text-decoration",
					value: " grey\n		",
				},
			]
		);
	}
}
