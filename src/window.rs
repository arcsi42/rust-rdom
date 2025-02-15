//! A virtual browser window.

use crate::internal_prelude::*;

use crate::node::{Document, DocumentStorage};

/// A simulated window for static rendering
pub struct Window {
    context: Weak<Sandbox>,
    document: Arc<Document>,
}

impl Window {
    pub(crate) fn new(context: Weak<Sandbox>) -> Arc<Window> {
        Arc::new_cyclic(|win_weak| -> Window {
            let document: Arc<Document> = Document::new(
                context.clone(),
                DocumentStorage {
                    default_view: win_weak.clone(),
                },
            );
            Window { context, document }
        })
    }

    /// Gets the window's document
    pub fn document(&self) -> Arc<Document> {
        self.document.clone()
    }
}
