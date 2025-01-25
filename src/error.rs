use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ForumInputError {
    ForbiddenCharacter,
    ForbiddenString,
    EmptyParameter,
}

impl std::fmt::Display for ForumInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &ForumInputError::ForbiddenCharacter => {
                write!(f, "Forbidden Character Detected")
            }
            &ForumInputError::ForbiddenString => {
                write!(f, "Forbidden String Detected")
            }
            &ForumInputError::EmptyParameter => write!(f, "Parameter is Empty"),
        }
    }
}

impl std::error::Error for ForumInputError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ForumMailError {
    TemplateHeader,
    TemplateLackOfParameter,
    Send(String),
}

impl std::fmt::Display for ForumMailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForumMailError::TemplateHeader => write!(f, "Template Header is Wrong"),
            ForumMailError::TemplateLackOfParameter => {
                write!(f, "Template Parameters Are Not Enough")
            }
            ForumMailError::Send(err_val) => write!(f, "Sending | {}", err_val),
        }
    }
}

impl std::error::Error for ForumMailError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ForumAuthError {
    AuthenticationFailed(String),
    TokenRefreshTimeOver,
}

impl std::fmt::Display for ForumAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForumAuthError::AuthenticationFailed(err_val) => {
                write!(f, "Authentication Failed | {}", err_val)
            }
            ForumAuthError::TokenRefreshTimeOver => write!(f, "Token Refresh Time is Over"),
        }
    }
}

impl std::error::Error for ForumAuthError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}
