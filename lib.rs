pub trait StringCasesExt {
    fn to_snake_case(&self) -> String;

    fn to_kebab_case(&self) -> String;

    fn to_pascal_case(&self) -> String;
}

impl<'a> StringCasesExt for &'a str {
    fn to_snake_case(&self) -> String {
        apply_camel_transform(self, '_')
    }

    fn to_kebab_case(&self) -> String {
        apply_camel_transform(self, '-')
    }

    fn to_pascal_case(&self) -> String {
        let mut last_was_underscore_or_start = true;
        let mut string = String::new();
        for chr in self.chars() {
            if chr == '_' {
                last_was_underscore_or_start = true;
            } else if last_was_underscore_or_start {
                last_was_underscore_or_start = false;
                string.extend(chr.to_uppercase());
            } else {
                string.push(chr);
            }
        }
        string
    }
}

pub(crate) fn apply_camel_transform(s: &str, divider: char) -> String {
    let mut peekable = s.chars().peekable();
    let mut string = String::new();
    while let Some(character) = peekable.next() {
        if let '_' | '-' = character {
            string.push(divider);
            continue;
        }

        string.extend(character.to_lowercase());
        if character.is_lowercase()
            && peekable
                .peek()
                .copied()
                .map(|c| c.is_uppercase() || c.is_numeric())
                .unwrap_or(false)
        {
            string.push(divider);
        }
    }
    string
}

#[cfg(test)]
mod tests {
    use super::StringCasesExt;

    #[test]
    fn snake_case() {
        assert_eq!("SomeStruct".to_snake_case(), "some_struct");
        assert_eq!("SomeTLA".to_snake_case(), "some_tla");
        assert_eq!(
            "Member_With_underscore".to_snake_case(),
            "member_with_underscore"
        );
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
    fn pascal_case() {
        assert_eq!("Some_Struct".to_pascal_case(), "SomeStruct");
        assert_eq!("some_thing".to_pascal_case(), "SomeThing");
        assert_eq!(
            "Member_With_underscore".to_pascal_case(),
            "MemberWithUnderscore"
        );
        assert_eq!("Field6".to_pascal_case(), "Field6");
    }
}
