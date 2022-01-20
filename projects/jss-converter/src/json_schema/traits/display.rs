use super::*;


impl Display for JssSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // if f.alternate() {
        //     write!(f, "f({})", self.0)
        // } else {
        //     write!(f, "{}", self.0)
        // }
        //                 writeln!(f, "/// des")?
        match self.kind {
            JssKind::Scheme => {
                writeln!(f, "{:indent$}schema _{tn} {{", "", indent = 0, tn = self.typing)?;
                for (key, value) in &self.annotation {
                    write!(f, "{:indent$}{key}: {{", "", indent = 4, key = key)?;
                    for line in format!("{}", value).lines() {
                        writeln!(f, "{:indent$}{rest}", "", indent = 4, rest = line)?;
                    }
                }

                writeln!(f, "{:indent$}}}", indent = 0)?;
            }
            JssKind::Property => {}
        }
        Ok(())
    }
}

impl Display for JssValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => { unimplemented!() }
            Self::Bool(_) => { unimplemented!() }
            Self::Number(_) => { unimplemented!() }
            Self::String(_) => { unimplemented!() }
            Self::Url(_) => { unimplemented!() }
            Self::Regex(_) => { unimplemented!() }
            Self::Array(v) => {
                match v.len() {
                    0 => write!(f, "[]"),
                    _ => write!(f, "Array"),
                }
            }
            Self::Object(_) => { unimplemented!() }
        }
    }
}

impl Display for JssType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined => { write!(f, "") }
            Self::Anything => { write!(f, ": anything") }
            Self::Nothing => { write!(f, ": nothing") }
            Self::String(_) => { write!(f, ": string") }
            Self::Number => { write!(f, ": number") }
            Self::Object => { write!(f, ": object") }
            Self::Reference(v) => { write!(f, "{}", v) }
        }
    }
}