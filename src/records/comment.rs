use crate::error::IGCError;
use crate::error::IGCError::CommentInitError;

#[derive(Debug, Clone)]
pub struct Comment {
    pub content: String
}

impl Comment {
    pub(crate) fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        if line.len() < 1 { return Err(CommentInitError(format!("'{line}' is too short to be a comment")))}
        let content = line[1..].to_string();
        Ok(Self {content})
    }
}