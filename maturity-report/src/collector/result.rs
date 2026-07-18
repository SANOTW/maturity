use maturity_core::annotation::maturity_attributes::MaturityAttributes;
use maturity_macro::maturity;
use tracing::instrument;

#[maturity(experimental)]
pub enum MaturityAnnotation {
    Missing,
    Parsed(MaturityAttributes),
    Invalid(syn::Error),
}

impl MaturityAnnotation {
    #[instrument(level = "trace", skip_all)]
    pub fn exists(&self) -> bool {
        matches!(self, Self::Parsed(_) | Self::Invalid(_))
    }

    #[instrument(level = "trace", skip_all)]
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Parsed(_))
    }

    #[instrument(level = "trace", skip_all)]
    pub fn attributes(&self) -> Option<&MaturityAttributes> {
        match self {
            Self::Parsed(value) => Some(value),
            _ => None,
        }
    }

    #[instrument(level = "trace", skip_all)]
    pub fn error(&self) -> Option<&syn::Error> {
        match self {
            Self::Invalid(error) => {
                println!("{error}");
                Some(error)
            }
            _ => None,
        }
    }
}
