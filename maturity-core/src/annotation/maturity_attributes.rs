use std::fmt;

use maturity_macro::maturity;
use quote::ToTokens;
use syn::{Expr, Ident, LitStr, Token};

// --------------------------------------

#[maturity(experimental)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MaturityState {
    Planned,
    #[default]
    Experimental,
    Developing,
    Stable,
    Deprecated,
}

impl std::str::FromStr for MaturityState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "planned" => Ok(Self::Planned),
            "experimental" => Ok(Self::Experimental),
            "developing" => Ok(Self::Developing),
            "stable" => Ok(Self::Stable),
            "deprecated" => Ok(Self::Deprecated),
            _ => Err(()),
        }
    }
}

impl fmt::Display for MaturityState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self {
            MaturityState::Planned => "planned",
            MaturityState::Experimental => "experimental",
            MaturityState::Developing => "developing",
            MaturityState::Stable => "stable",
            MaturityState::Deprecated => "deprecated",
        };

        f.write_str(state)
    }
}

// --------------------------------------

#[maturity(experimental)]
pub enum KnownAttribute {
    State,
    Todo,
    Refactor,
}

// --------------------------------------

#[maturity(experimental)]
#[derive(Debug, Clone)]
pub struct UnknownAttribute {
    pub key: String,
    pub value: Option<syn::Expr>,
}

// --------------------------------------

#[maturity(experimental)]
#[derive(Debug, Default, Clone)]
pub struct MaturityAttributes {
    pub state: Option<MaturityState>,
    pub todo: Option<String>,
    pub refactor: Option<String>,
    pub unknown: Vec<UnknownAttribute>,
}

impl syn::parse::Parse for MaturityAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attributes = Self::default();

        while !input.is_empty() {
            let indent: Ident = input.parse()?;
            let name = indent.to_string();

            if let Ok(state) = name.parse::<MaturityState>() {
                attributes.state = Some(state);
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                }
                continue;
            }

            match name.as_str() {
                "todo" => {
                    input.parse::<Token![=]>()?;
                    attributes.todo = Some(input.parse::<LitStr>()?.value());
                    // println!("{}", attributes.todo.clone().unwrap().to_string())
                }

                "refactor" => {
                    input.parse::<Token![=]>()?;
                    attributes.refactor = Some(input.parse::<LitStr>()?.value());
                    // println!("{}", attributes.refactor.clone().unwrap().to_string())
                }

                _ => {
                    let value = if input.peek(Token![=]) {
                        input.parse::<Token![=]>()?;
                        Some(input.parse::<Expr>()?)
                    } else {
                        None
                    };

                    attributes
                        .unknown
                        .push(UnknownAttribute { key: name, value });
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(attributes)
    }
}

impl fmt::Display for MaturityAttributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(state) = &self.state {
            write!(f, "[{state}]")?;
        }

        if let Some(todo) = &self.todo {
            write!(f, " todo=\"{todo}\"")?;
        }

        if let Some(refactor) = &self.refactor {
            write!(f, " refactor=\"{refactor}\"")?;
        }

        for unknown in &self.unknown {
            write!(f, " {}", unknown.key)?;

            if let Some(value) = &unknown.value {
                // This line utilises the `quote` crate
                write!(f, "={}", value.to_token_stream())?;
            }
        }

        Ok(())
    }
}
