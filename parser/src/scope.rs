use nom::{
	branch::alt, character::complete::multispace0, combinator::map, multi::many0,
	sequence::delimited, IResult,
};

use crate::{AtRule, RuleSet};

/// A child within the scope.
#[derive(Debug, PartialEq)]
pub enum ScopeChild<'a> {
	/// @ rules
	AtRule(AtRule<'a>),
	/// ruleset
	RuleSet(RuleSet<'a>),
}

impl<'a> ScopeChild<'a> {
	/// Parses the given text into a scope child.
	pub fn parse(input: &'a str) -> IResult<&str, Self> {
		alt((
			map(AtRule::parse, |itm| ScopeChild::AtRule(itm)),
			map(RuleSet::parse, |ruleset| ScopeChild::RuleSet(ruleset)),
		))(input)
	}
}

/// A scope of CSS code, usually just used for parsing an entire file.
#[derive(Debug, PartialEq)]
pub struct Scope<'a> {
	/// The [`ScopeChild`] nodes within the scope.
	pub children: Vec<ScopeChild<'a>>,
}

impl<'a> Scope<'a> {
	/// Parses the text into a [`Scope`] of children.
	pub fn parse(input: &'a str) -> IResult<&str, Self> {
		map(
			many0(delimited(multispace0, ScopeChild::parse, multispace0)),
			|children| Self { children },
		)(input)
	}
}
