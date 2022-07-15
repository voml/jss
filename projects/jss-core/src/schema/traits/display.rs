use std::fmt::Write;

use text_utils::indent;

use super::*;

impl Display for JssSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.write_description(f)?;
        match self.kind {
            JssKind::Scheme => f.write_str("schema "),
            JssKind::Property => f.write_str("."),
            JssKind::PropertyTop => f.write_str("prop "),
            JssKind::Definition => f.write_str("def "),
        }?;
        self.head(f)?;
        // for (key, value) in &self.keywords {
        //     writeln!(f, "{}", indent(format!("{}: {:#?}", key, value), 4))?;
        // }
        for (key, value) in &self.attribute {
            writeln!(f, "{}", indent(format!("{}: {:#?}", key, value), 4))?;
        }
        match self.kind {
            JssKind::Scheme => {}
            _ => {
                for (_, value) in &self.property {
                    writeln!(f, "{}", indent(format!("{:#}", value), 4))?;
                    writeln!(f)?;
                }
            }
        }
        f.write_str("}\n")?;
        if let JssKind::Scheme = self.kind {
            for (_, value) in &self.definition {
                Display::fmt(value, f)?;
                writeln!(f)?;
            }
            for (_, value) in &self.property {
                Display::fmt(value, f)?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl JssSchema {
    #[inline]
    fn head(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            None => f.write_char('_')?,
            Some(s) => f.write_str(s)?,
        }
        if self.typing != JssType::Undefined {
            f.write_str(&self.typing.to_string())?
        }
        f.write_str(" {\n")?;
        Ok(())
    }
    #[inline]
    fn write_description(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.has_description() {
            for line in self.description.lines() {
                writeln!(f, "/// {}", line)?
            }
        }
        Ok(())
    }
}

impl Display for JssValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => {
                unimplemented!()
            }
            Self::Boolean(_) => {
                unimplemented!()
            }
            Self::Number(_) => {
                unimplemented!()
            }
            Self::String(v) => Debug::fmt(v, f),
            Self::Url(_) => {
                unimplemented!()
            }
            Self::Regex(_) => {
                unimplemented!()
            }
            Self::Array(v) => Debug::fmt(v, f),
            Self::Object(_) => {
                unimplemented!()
            }
        }
    }
}

impl Display for JssType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined => f.write_str("undefined"),
            Self::Anything => f.write_str("anything"),
            Self::Nothing => f.write_str("nothing"),
            Self::Number => f.write_str("number"),
            JssType::Integer => f.write_str("integer"),
            Self::Object => f.write_str("object"),
            Self::Reference(v) => f.write_str(&v),
            JssType::String => f.write_str("string"),
            JssType::Array => f.write_str("array"),
            Self::Complex(v) => f.write_str(&v.to_string()),
        }
    }
}

impl Display for JssComplexType {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
