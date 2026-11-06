fn main() {
	use string_cases::StringCasesExt;

	let input = std::env::args().nth(1).expect("expected an argument");

	let snake_case: &str = &input.to_snake_case();
	println!("snake_case: {snake_case}");

	let kebab_case: &str = &input.to_kebab_case();
	println!("kebab-case: {kebab_case}");

	let camel_case: &str = &input.to_camel_case();
	println!("CamelCase: {camel_case}");

	let pascal_case: &str = &input.to_pascal_case();
	println!("PascalCase: {pascal_case}");
}
