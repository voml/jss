pub trait JsonValue {
    fn get_key(&self, key: &str) -> Option<&Self>;

    fn as_boolean(&self) -> Option<&bool>;
    fn as_array(&self) -> Option<&dyn JsonArray<ValueType = Self>>;
    fn as_object(&self) -> Option<&dyn JsonObject<ValueType = Self>>;
    fn as_string(&self) -> Option<&String>;
    #[inline]
    fn get_key_string(&self, key: &str) -> Option<&String> {
        self.get_key(key).and_then(|f| f.as_string())
    }

    fn get_key_boolean(&self, key: &str) -> Option<&bool> {
        self.get_key(key).and_then(|f| f.as_boolean())
    }
    #[inline]
    fn get_key_array(&self, key: &str) -> Option<&dyn JsonArray<ValueType = Self>> {
        self.get_key(key).and_then(|f| f.as_array())
    }
    #[inline]
    fn get_key_object(&self, key: &str) -> Option<&dyn JsonObject<ValueType = Self>> {
        self.get_key(key).and_then(|f| f.as_object())
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

pub trait JsonArray {
    type ValueType;
}

pub trait JsonObject {
    type ValueType;
}
