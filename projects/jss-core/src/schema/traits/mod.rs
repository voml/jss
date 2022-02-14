use super::*;

mod debug;
mod display;

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
    /// Property Node
    fn default() -> Self {
        Self {
            kind: JssKind::Property,
            name: None,
            description: "".to_string(),
            typing: Default::default(),
            property: Default::default(),
            definition: Default::default(),
            attribute: Default::default(),
            // keywords: Default::default(),
        }
    }
}

impl JssSchema {
    #[inline]
    pub fn property() -> Self {
        Self::default()
    }
    #[inline]
    pub fn anything() -> Self {
        Self { kind: JssKind::Scheme, typing: JssType::Anything, ..Default::default() }
    }
    #[inline]
    pub fn nothing() -> Self {
        Self { kind: JssKind::Scheme, typing: JssType::Nothing, ..Default::default() }
    }
    #[inline]
    pub fn top() -> Self {
        Self { kind: JssKind::Scheme, typing: JssType::Undefined, ..Default::default() }
    }
    #[inline]
    pub fn is_anything(&self) -> bool {
        matches!(self.typing, JssType::Anything)
    }
    #[inline]
    pub fn is_noting(&self) -> bool {
        matches!(self.typing, JssType::Nothing)
    }
    pub fn is_top(&self) -> bool {
        matches!(self.kind, JssKind::Scheme)
    }
}
