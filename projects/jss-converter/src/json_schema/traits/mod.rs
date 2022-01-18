use super::*;

impl Default for JssKind {
    fn default() -> Self {
        Self::Property
    }
}

impl Default for JssType {
    fn default() -> Self {
        Self::Undefined
    }
}

impl Default for JssSchema {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            typing: Default::default(),
            properties: Default::default(),
            definition: Default::default(),
        }
    }
}
