use derive_more::with_trait::Display;

pub type Names = Vec<Name>;

#[derive(Debug, Eq, PartialEq, Clone, Display)]
#[display("{_0}")]
pub struct Name(String);

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum NameError {
    #[error("Name is empty")]
    EmptyName,
}

impl TryFrom<String> for Name {
    type Error = NameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.trim().to_string();
        if value.is_empty() {
            Err(NameError::EmptyName)
        } else {
            Ok(Name(value))
        }
    }
}

#[cfg(test)]
pub mod test_helpers {
    use crate::business::name::Name;

    impl From<&str> for Name {
        fn from(value: &str) -> Self {
            value
                .to_string()
                .try_into()
                .expect("Test values should be valid")
        }
    }
}

#[cfg(test)]
mod name_should {
    use super::*;
    use googletest::assert_that;
    use googletest::matchers::*;
    use rstest::rstest;

    #[rstest]
    fn build_from_a_string() {
        let name: Result<Name, _> = String::from("Xavier").try_into();
        assert_that!(name, ok(pat!(Name(eq("Xavier")))))
    }

    #[rstest]
    #[case::space_before(" Xavier")]
    #[case::space_after("Xavier ")]
    #[case::space_both_ends(" Xavier ")]
    fn trim_values(#[case] name: &str) {
        let name: Result<Name, _> = String::from(name).try_into();
        assert_that!(name, ok(pat!(Name(eq("Xavier")))))
    }

    #[rstest]
    #[case::full_empty("")]
    #[case::single_space(" ")]
    fn cannot_be_empty(#[case] name: &str) {
        let name: Result<Name, _> = String::from(name).try_into();
        assert_that!(name, err(eq(&NameError::EmptyName)))
    }
}
