use parser::Scope;

pub trait Plugin {
	fn transform_ast(&self, scope: &mut Scope);
	fn serialize(&self, scope: &Scope) -> Option<String>;
}

pub fn vex<T: AsRef<str>>(input: T, plugins: Vec<impl Plugin>) -> Option<String> {
	let (_, mut scope) = parser::Scope::parse(input.as_ref()).unwrap();
	for plugin in plugins.iter() {
		plugin.transform_ast(&mut scope);
	}

	for plugin in plugins.iter() {
		let serialized = plugin.serialize(&scope);
		if let Some(serialized) = serialized {
			return Some(serialized);
		}
	}

	None
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
