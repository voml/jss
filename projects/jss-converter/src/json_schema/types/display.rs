use super::*;
use std::fmt::{Debug, Formatter};

impl Debug for JssSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let w = &mut match self.kind {
            JssKind::Scheme => f.debug_struct("JssScheme"),
            JssKind::Property => f.debug_struct("JssProperty"),
        };
        w.field("type", &self.typing);
        w.field("definitions", &self.definition);
        w.field("properties", &self.properties);
        w.finish()
    }
}
