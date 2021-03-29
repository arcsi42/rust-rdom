//! Representation of a [NamedNodeMap](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)
//! and associated metadata.

use std::sync::{Arc, Weak};

use crate::node::raw::{AnyRawNode, AttrNode, element::AnyRawElement, private::PrivateAnyRawNode};
use crate::sandbox::Sandbox;

/// A [NamedNodeMap](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap) structure
pub struct NamedNodeMap {
    /// Reference to the sandbox to which this NamedNodeMap belongs
    pub context: Weak<Sandbox>,

    /// Reference back up to the raw element
    pub element: Weak<AnyRawElement>,

    /// The attribute nodes
    pub attribute_list: Vec<Arc<AttrNode>>,
}

impl NamedNodeMap {
    fn new(context: Weak<Sandbox>, element: Weak<AnyRawElement>) -> Arc<NamedNodeMap> {
        Arc::new(NamedNodeMap {
            context,
            element,
            attribute_list: Vec::new()
        })
    }

    fn length(&self) -> usize {
        self.attribute_list.len()
    }

    fn item(&self, index: usize) -> Option<Arc<AttrNode>> {
        self.attribute_list.get(index).map(|v| v.clone())
    }
}