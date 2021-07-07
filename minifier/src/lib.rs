//! A minifier plugin for [`vex`].

#![deny(missing_docs)]

use itertools::Itertools;
use parser::{AtRule, Declaration, DeclarationBlock, RuleSet, Scope, ScopeChild, Selector};
use vex::Plugin;

/// The minifier struct.
pub struct Minifier;

impl Plugin for Minifier {
	fn serialize(&self, scope: &Scope) -> Option<String> {
		Some(scope.minify())
	}
}

/// A trait for minifying CSS.
pub trait Minify {
	/// Minifies the current struct into CSS.
	fn minify(&self) -> String;
}

impl Minify for Declaration<'_> {
	fn minify(&self) -> String {
		format!("{}:{}", self.name, self.value.trim())
	}
}

impl Minify for DeclarationBlock<'_> {
	fn minify(&self) -> String {
		self.declarations.iter().map(|decl| decl.minify()).join(";")
	}
}

impl Minify for Selector<'_> {
	fn minify(&self) -> String {
		let Self(selector) = self;
		String::from(selector.trim().replace("\n", " "))
	}
}

impl Minify for RuleSet<'_> {
	fn minify(&self) -> String {
		format!("{}{{{}}}", self.selector.minify(), self.block.minify())
	}
}

impl Minify for AtRule<'_> {
	fn minify(&self) -> String {
		match self {
			Self::Charset(charset) => format!("@charset \"{}\";", charset),
			Self::Media { condition, scope } => {
				format!("@media {}{{{}}}", condition.trim(), scope.minify())
			}
		}
	}
}

impl Minify for ScopeChild<'_> {
	fn minify(&self) -> String {
		match self {
			Self::AtRule(rule) => rule.minify(),
			Self::RuleSet(rule) => rule.minify(),
		}
	}
}

impl Minify for Scope<'_> {
	fn minify(&self) -> String {
		self.children.iter().map(|child| child.minify()).join("")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ruleset() {
		let (_, ruleset) = RuleSet::parse(
			"h1 {
			color: blue;
		}",
		)
		.unwrap();

		assert_eq!(ruleset.minify(), "h1{color:blue}");
	}
}
