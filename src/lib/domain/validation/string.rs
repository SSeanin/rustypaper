use crate::domain::validation::ValidationError;

#[derive(Debug, Clone)]
pub struct Check(String);

impl Check {
    pub fn new<T>(value: T) -> Self where T: Into<String> {
        Self(value.into().trim().to_owned())
    }

    pub fn is_min_length(&self, min: usize) -> super::Result<Self> {
        if self.0.len() < min {
            Err(ValidationError::MinLength { min, got: self.0.len() })
        } else {
            Ok(self.clone())
        }
    }

    pub fn is_max_length(&self, max: usize) -> super::Result<Self> {
        if self.0.len() > max {
            Err(ValidationError::MaxLength { max, got: self.0.len() })
        } else {
            Ok(self.clone())
        }
    }

    pub fn is_not_empty(&self) -> super::Result<Self> {
        (!self.0.is_empty())
            .then(|| self.clone())
            .ok_or(ValidationError::EmptyContent)
    }

    pub fn into_inner(self) -> String {
        self.0.escape_default().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{Check, ValidationError};

    #[test]
    fn length_min() {
        let check = Check::new("string with 25 characters");
        let ok = check.is_min_length(20);
        let err = check.is_min_length(30);

        assert!(ok.is_ok());

        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), ValidationError::MinLength { min: 30, got: 25 });
    }

    #[test]
    fn length_max() {
        let check = Check::new("string with 25 characters");
        let ok = check.is_max_length(30);
        let err = check.is_max_length(20);

        assert!(ok.is_ok());

        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), ValidationError::MaxLength { max: 20, got: 25 });
    }

    #[test]
    fn empty() {
        let not_empty_string = Check::new("Some test");
        let empty_string = Check::new("");

        let not_empty_result = not_empty_string.is_not_empty();
        let empty_result = empty_string.is_not_empty();

        assert!(not_empty_result.is_ok());

        assert!(empty_result.is_err());
        assert_eq!(empty_result.unwrap_err(), ValidationError::EmptyContent);
    }

    #[test]
    fn escape_unicode_values() {
        let check = Check::new("🥰❤\t\n test");
        let value = check.into_inner();

        assert_eq!(value, r#"\u{1f970}\u{2764}\t\n test"#)
    }
}
