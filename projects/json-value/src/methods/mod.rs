use serde_json::Map;
pub trait JsonValue
where
    Self: Sized,
{
    fn get_key(&self, key: &str) -> Option<&Self>;
    fn extract_key(&mut self, key: &str) -> Option<Self>;

    fn as_boolean(&self) -> Option<&bool>;
    fn as_array(&self) -> Option<&Vec<Self>>;
    fn as_string(&self) -> Option<&String>;
    fn as_object(&self) -> Option<&Map<String, Self>>;
    fn into_object(self) -> Option<Map<String, Self>>;
    #[inline]
    fn get_key_string(&self, key: &str) -> Option<&String> {
        self.get_key(key).and_then(|f| f.as_string())
    }

    fn get_key_as_boolean(&self, key: &str) -> Option<&bool> {
        self.get_key(key).and_then(|f| f.as_boolean())
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
    fn extract_key_as_object(&mut self, key: &str) -> Option<Map<String, Self>> {
        self.extract_key(key).and_then(|f| f.into_object())
    }
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
