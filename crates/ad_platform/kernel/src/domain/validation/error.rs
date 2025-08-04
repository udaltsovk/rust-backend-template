use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub struct ValidationErrors(Vec<(String, String)>);
impl From<Vec<(String, String)>> for ValidationErrors {
    fn from(errors: Vec<(String, String)>) -> Self {
        Self(errors)
    }
}
impl From<Vec<(&str, &str)>> for ValidationErrors {
    fn from(errors: Vec<(&str, &str)>) -> Self {
        Self(
            errors
                .into_iter()
                .map(|(path, err)| (path.to_string(), err.to_string()))
                .collect(),
        )
    }
}
impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errors = self
            .0
            .iter()
            .map(|(path, err)| format!("{path}: {err}"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("Validation errors: [\n{errors}\n]").fmt(f)
    }
}
impl Error for ValidationErrors {}
impl From<Vec<ValidationErrors>> for ValidationErrors {
    fn from(errors: Vec<ValidationErrors>) -> Self {
        Self(errors.into_iter().flat_map(|err| err.0).collect())
    }
}
impl ValidationErrors {
    pub fn into_inner(self) -> Vec<(String, String)> {
        self.0
    }
}
