#![doc = include_str!("./README.md")]

/// Extension trait for adding methods
pub trait StringCasesExt {
    /// '_' defines boundaries
    fn to_snake_case(&self) -> String;

    /// '-' defines boundaries
    fn to_kebab_case(&self) -> String;

    /// Differences in cases define boundaries
    fn to_camel_case(&self) -> String;

    /// Differences in cases define boundaries. First character is always uppercase
    fn to_pascal_case(&self) -> String;
}

impl<T> StringCasesExt for T
where
    T: std::ops::Deref<Target = str>,
{
    fn to_snake_case(&self) -> String {
        apply_divider_transform(self, '_')
    }

    fn to_kebab_case(&self) -> String {
        apply_divider_transform(self, '-')
    }

    fn to_camel_case(&self) -> String {
        apply_case_transform(self, false)
    }

    fn to_pascal_case(&self) -> String {
        apply_case_transform(self, true)
    }
}

fn apply_divider_transform(s: &str, divider: char) -> String {
    let mut peekable = s.chars().peekable();
    let mut string = String::with_capacity(s.len());
    let mut previous_was_uppercase = false;

    while let Some(character) = peekable.next() {
        if let '_' | '-' = character {
            string.push(divider);
            continue;
        }

        let upcoming = peekable.peek().copied();

        let uppercase_sequence_to_lower = previous_was_uppercase
            && character.is_uppercase()
            && upcoming.map(char::is_lowercase).unwrap_or(false);

        if uppercase_sequence_to_lower {
            string.push(divider);
            string.extend(character.to_lowercase());
            continue;
        }

        let lowercase_to_other = character.is_lowercase()
            && upcoming
                .map(|c| c.is_uppercase() || c.is_numeric())
                .unwrap_or(false);

        string.extend(character.to_lowercase());

        if lowercase_to_other {
            string.push(divider);
        }

        previous_was_uppercase = character.is_uppercase();
    }
    string
}

fn apply_case_transform(s: &str, uppercase_first: bool) -> String {
    let mut last_was_divider = uppercase_first;
    let mut string = String::new();
    for chr in s.chars() {
        if chr == '_' {
            last_was_divider = true;
        } else if last_was_divider {
            last_was_divider = false;
            string.extend(chr.to_uppercase());
        } else {
            string.push(chr);
        }
    }
    string
}

#[cfg(test)]
mod tests {
    use super::StringCasesExt;

    #[test]
    fn works_on_string() {
        let x: String = "test".into();
        let _ = x.to_snake_case();
    }

    #[test]
    fn snake_case() {
        assert_eq!("SomeStruct".to_snake_case(), "some_struct");
        assert_eq!("SomeTLA".to_snake_case(), "some_tla");
        assert_eq!(
            "Member_With_underscore".to_snake_case(),
            "member_with_underscore"
        );
        assert_eq!("JSXElement".to_snake_case(), "jsx_element");
        assert_eq!("Field6".to_snake_case(), "field_6");
    }

    #[test]
    fn kebab_case() {
        assert_eq!("SomeStruct".to_kebab_case(), "some-struct");
        assert_eq!("SomeTLA".to_kebab_case(), "some-tla");
        assert_eq!(
            "Member_With_underscore".to_kebab_case(),
            "member-with-underscore"
        );
        assert_eq!("Field6".to_kebab_case(), "field-6");
    }

    #[test]
    fn camel_case() {
        assert_eq!("Some_Struct".to_camel_case(), "SomeStruct");
        assert_eq!("some_thing".to_camel_case(), "someThing");
        assert_eq!(
            "Member_With_underscore".to_camel_case(),
            "MemberWithUnderscore"
        );
        assert_eq!("field6".to_camel_case(), "field6");
    }

    #[test]
    fn pascal_case() {
        assert_eq!("Some_Struct".to_pascal_case(), "SomeStruct");
        assert_eq!("some_thing".to_pascal_case(), "SomeThing");
        assert_eq!(
            "Member_With_underscore".to_pascal_case(),
            "MemberWithUnderscore"
        );
        assert_eq!("field6".to_pascal_case(), "Field6");
    }
}
