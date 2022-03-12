use serde_json::Map;

/// Attempt to convert json value to specified type
pub trait JsonValueWrap
where
    Self: Sized,
{
    fn as_boolean(&self) -> Option<&bool>;
    fn as_array(&self) -> Option<&Vec<Self>>;
    fn as_string(&self) -> Option<&String>;
    fn as_object(&self) -> Option<&Map<String, Self>>;
    fn into_boolean(self) -> Option<bool>;
    fn into_string(self) -> Option<String>;
    fn into_array(self) -> Option<Vec<Self>>;
    fn into_object(self) -> Option<Map<String, Self>>;
}

/// Check the type of json value
pub trait JsonValueCheck: JsonValueWrap
where
    Self: Sized,
{
    fn is_null(&self) -> bool;
    fn is_empty(&self) -> bool;
    #[inline]
    fn is_boolean(&self) -> bool {
        self.as_boolean().is_some()
    }
    #[inline]
    fn is_true(&self) -> bool {
        matches!(self.as_boolean(), Some(true))
    }
    #[inline]
    fn is_false(&self) -> bool {
        matches!(self.as_boolean(), Some(false))
    }
    #[inline]
    fn is_string(&self) -> bool {
        self.as_string().is_some()
    }
    #[inline]
    fn is_array(&self) -> bool {
        self.as_array().is_some()
    }
}

/// Treat json value as array and get data from it according to index
pub trait JsonMaybeArray: JsonValueWrap
where
    Self: Sized,
{
    fn get_index(&self, index: usize) -> Option<&Self>;
    fn mut_index(&mut self, index: usize) -> Option<&mut Self>;
}

/// Treat json value as object and get data from it according to key
pub trait JsonMaybeObject: JsonValueWrap
where
    Self: Sized,
{
    fn get_key(&self, key: &str) -> Option<&Self>;
    fn mut_key(&mut self, key: &str) -> Option<&mut Self>;
    fn extract_key(&mut self, key: &str) -> Option<Self>;
    #[inline]
    fn get_key_as_boolean(&self, key: &str) -> Option<&bool> {
        self.get_key(key).and_then(|f| f.as_boolean())
    }
    #[inline]
    fn get_key_as_string(&self, key: &str) -> Option<&String> {
        self.get_key(key).and_then(|f| f.as_string())
    }
    #[inline]
    fn get_key_as_array(&self, key: &str) -> Option<&Vec<Self>> {
        self.get_key(key).and_then(|f| f.as_array())
    }
    #[inline]
    fn get_key_as_object(&self, key: &str) -> Option<&Map<String, Self>> {
        self.get_key(key).and_then(|f| f.as_object())
    }
    #[inline]
    fn extract_key_as_boolean(&mut self, key: &str) -> Option<bool> {
        self.extract_key(key).and_then(|f| f.into_boolean())
    }
    #[inline]
    fn extract_key_as_string(&mut self, key: &str) -> Option<String> {
        self.extract_key(key).and_then(|f| f.into_string())
    }
    #[inline]
    fn extract_key_as_array(&mut self, key: &str) -> Option<Vec<Self>> {
        self.extract_key(key).and_then(|f| f.into_array())
    }
    #[inline]
    fn extract_key_as_object(&mut self, key: &str) -> Option<Map<String, Self>> {
        self.extract_key(key).and_then(|f| f.into_object())
    }
}
