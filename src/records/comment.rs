use crate::records::error::IGCError;
use crate::records::error::IGCError::CommentInitError;

#[derive(Debug, Clone)]
pub struct Comment {
    pub content: String
}

impl Comment {
    fn parse(line: &str) -> Result<Self, IGCError> where Self: Sized {
        if line.len() < 1 { return Err(CommentInitError(format!("'{line}' is too short to be a comment")))}
        let content = line[1..].to_string();
        Ok(Self {content})
    }
}