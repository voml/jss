use std::fmt::{Display, Formatter};
use super::*;


impl Display for JssSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "f({})", self.0)
        } else {
            write!(f, "{}", self.0)
        }

        match self.kind {
            JssKind::Scheme => {
                f.write_str("/// des")
            }
            JssKind::Property => {}
        }
    }
}