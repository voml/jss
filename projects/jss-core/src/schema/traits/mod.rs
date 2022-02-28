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
    pub fn definition() -> Self {
        Self { kind: JssKind::Definition, typing: JssType::Undefined, ..Default::default() }
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
    pub fn is_ref_type(&self) -> bool {
        matches!(self.typing, JssType::Reference(_))
    }
}

impl JssSchema {
    pub fn get_name(&self) -> &str {
        debug_assert!(self.name.is_some());
        match &self.name {
            Some(s) => s.as_str(),
            None => "",
        }
    }
    pub fn set_name<S>(&mut self, name: S)
    where
        S: Into<String>,
    {
        self.name = Some(name.into())
    }

    pub fn get_type(&self) -> &JssType {
        &self.typing
    }
    pub fn set_type<S>(&mut self, typing: S)
    where
        S: Into<JssType>,
    {
        self.typing = typing.into()
    }

    pub fn has_description(&self) -> bool {
        !self.description.is_empty()
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn set_description<S>(&mut self, description: S)
    where
        S: Into<String>,
    {
        self.description = description.into()
    }

    pub fn properties(&self) -> Iter<'_, String, JssSchema> {
        self.property.iter()
    }
    pub fn insert_property<K, V>(&mut self, key: K, value: V) -> Option<JssSchema>
    where
        K: Into<String>,
        V: Into<JssSchema>,
    {
        self.property.insert(key.into(), value.into())
    }

    pub fn attributes(&self) -> Iter<'_, String, JssValue> {
        self.attribute.iter()
    }
    pub fn insert_attribute<K, V>(&mut self, key: K, value: V) -> Option<JssValue>
    where
        K: Into<String>,
        V: Into<JssValue>,
    {
        self.attribute.insert(key.into(), value.into())
    }

    pub fn definitions(&self) -> Iter<'_, String, JssSchema> {
        self.definition.iter()
    }
    pub fn insert_definition<K, V>(&mut self, key: K, value: V) -> Option<JssSchema>
    where
        K: Into<String>,
        V: Into<JssSchema>,
    {
        self.definition.insert(key.into(), value.into())
    }
}
