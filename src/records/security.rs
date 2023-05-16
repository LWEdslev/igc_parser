use crate::error::IGCError;
use crate::error::IGCError::SecurityInitError;
use crate::Result;
#[derive(Debug, Clone)]
pub struct Security {
    pub security_code: String,
}

impl Security {
    pub(crate) fn parse(line: &str) -> Result<Self> {
        if line.len() < 2 { return Err(SecurityInitError(format!("'{line}' is too short to be a security tag")))};
        let security_code = line[1..].to_string();
        Ok(Self {security_code})
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn security_parsed_correctly() {
        let sec = Security::parse("GREJNGJERJKNJKRE31895478537H43982FJN9248F942389T433T").unwrap();
        assert_eq!(sec.security_code, "REJNGJERJKNJKRE31895478537H43982FJN9248F942389T433T".to_string())
    }
}