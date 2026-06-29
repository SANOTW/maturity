use maturity_macro::maturity;

#[maturity]
#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("[ I/O error ]: {0}")]
    Io(#[from] std::io::Error),
    #[error("[ Tracing Subscriber error ]: {0}")]
    TracingSubscriber(#[from] tracing_subscriber::filter::ParseError),
}
