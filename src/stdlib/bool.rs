use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Bool {
    True,
    False,
}

impl Display for Bool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Bool::True => true.to_string(),
                Bool::False => false.to_string(),
            }
        ))
    }
}
