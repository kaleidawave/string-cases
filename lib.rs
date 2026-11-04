#![doc = include_str!("./README.md")]
#![warn(clippy::pedantic)]

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

use std::borrow::Cow;

fn apply_divider_transform(s: &str, divider: char) -> String {
    fn is_break(c: char) -> bool {
        c.is_uppercase() || matches!(c, '_' | '-' | ' ') || c.is_ascii_digit()
    }

    let mut upto = 0;
    let mut acc = Cow::Borrowed("");
    let mut last_divider = false;

    for (idx, matched) in s.match_indices(is_break) {
        let before = &s[upto..idx];
        acc.to_mut().push_str(before);

        if let "-" | "_" | " " = matched {
            acc.to_mut().push(divider);
            upto = idx + matched.len();
            last_divider = true;
        } else {
            let after = &s[idx..];
            let mut chars = after.chars();
            let first_char = chars.next().unwrap();
            let second_char = chars.next();

            // TODO this condition is a bit of a mess
            if !last_divider
                && upto > 0
                && (!before.is_empty() || second_char.is_some_and(|c| !is_break(c)))
            {
                acc.to_mut().push(divider);
            }

            acc.to_mut().extend(first_char.to_lowercase());

            upto = idx + first_char.len_utf8();
            last_divider = false;
        }
    }

    acc += &s[upto..];

    acc.into_owned()
}

/// TODO should `Abc 123` be `Abc123` (remove spaces)?
fn apply_case_transform(s: &str, uppercase_first: bool) -> String {
    let mut acc = Cow::Borrowed("");
    let mut upto = 0;
    if uppercase_first {
        if let Some(chr) = s.chars().next() {
            acc.to_mut().extend(chr.to_uppercase());
            upto += chr.len_utf8();
        }
    }
    for (idx, _) in s.match_indices('_') {
        acc += &s[upto..idx];
        let after = &s[idx + 1..];
        upto = idx + 1;
        if let Some(chr) = after.chars().next() {
            acc.to_mut().extend(chr.to_uppercase());
            upto += chr.len_utf8();
        }
    }
    acc += &s[upto..];
    acc.into_owned()
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
        assert_eq!("Field 6".to_snake_case(), "field_6");
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
