use std::str::FromStr;
use failure::Error;

#[derive(Debug, PartialEq, Clone, DbEnum)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl FromStr for Priority {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        if lower == "high" || lower == "h" {
            Ok(Priority::High)
        } else if lower == "medium" || lower == "m" {
            Ok(Priority::Medium)
        } else if lower == "low" || lower == "l" {
            Ok(Priority::Low)
        } else {
            Err(format_err!("cannot parse {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_all_lowercase() {
        let inputs = &[("high", Priority::High), ("low", Priority::Low)];

        for &(ref ip, ref expected) in inputs {
            let priority: Priority = ip.parse().unwrap();
            assert_eq!(&priority, expected);
        }
    }

    #[test]
    fn parse_mixed_case() {
        let inputs = &[("hiGh", Priority::High), ("MEDIUM", Priority::Medium)];

        for &(ref ip, ref expected) in inputs {
            let priority: Priority = ip.parse().unwrap();
            assert_eq!(&priority, expected);
        }
    }
    #[test]
    fn parse_short() {
        let inputs = &[("h", Priority::High), ("M", Priority::Medium)];

        for &(ref ip, ref expected) in inputs {
            let priority: Priority = ip.parse().unwrap();
            assert_eq!(&priority, expected);
        }
    }
}
