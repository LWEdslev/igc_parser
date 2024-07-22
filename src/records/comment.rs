use crate::error::IGCError::CommentInitError;
use crate::{Result, StrWrapper};

#[cfg(feature = "serde")] use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct Comment {
    pub content: StrWrapper,
}

impl Comment {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.is_empty() { return Err(CommentInitError(format!("'{line}' is too short to be a comment")))}
        let content = line[1..].to_string().into();
        Ok(Self {content})
    }
}