use std::collections::HashMap;
use tower_lsp::lsp_types::{Position, Url};
use tree_sitter::{InputEdit, Point};

use crate::buffer::Buffer;

pub struct DocumentState {
    pub documents: HashMap<Url, Buffer>,
}

impl DocumentState {
    pub fn get_state(&self, url: &Url) -> Option<&Buffer> {
        self.documents.get(url)
    }

    pub fn get_state_mut(&mut self, url: &Url) -> Option<&mut Buffer> {
        self.documents.get_mut(url)
    }

    pub fn update_state(&mut self, url: &Url, new_src: String) {
        let Some(buffer) = self.documents.get_mut(url) else {
            return;
        };

        buffer.tree = match buffer.parser.parse(&new_src, None) {
            Some(tree) => tree,
            None => return,
        };
        buffer.text = new_src;
    }

    pub fn insert_state(&mut self, url: Url, buffer: Buffer) {
        self.documents.insert(url, buffer);
    }
}
