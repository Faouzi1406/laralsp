use tree_sitter::Parser;
use tree_sitter_php::language_php;

use crate::types::languages;

pub struct Buffer {
    pub language: languages::Language,
    pub parser: Parser,
    pub text: String,
    pub tree: tree_sitter::Tree,
}

pub struct CurrentBuffer(pub Option<Buffer>);

impl Buffer {
    pub fn new(text: String) -> anyhow::Result<Self> {
        let mut parser = Parser::new();
        parser.set_language(language_php())?;
        match parser.parse(&text, None) {
            Some(tree) => Ok(Buffer {
                language: languages::Language::PHP,
                parser,
                text,
                tree,
            }),
            None => Err(anyhow::anyhow!("Could not parse tree.")),
        }
    }

    pub fn get_node_at_point(&self, point: tree_sitter::Point) -> Option<tree_sitter::Node> {
        self.tree
            .root_node()
            .descendant_for_point_range(point, point)
    }

    pub fn get_variables_in_scope(&self, point: tree_sitter::Point) {
        let node = self.get_node_at_point(point);
    }

    /// Returns the function name if the point is inside a function call.
    pub fn get_function(&self, point: tree_sitter::Point) -> Option<&str> {
        let node = self.get_node_at_point(point)?;

        let function_expression_node = match node.kind() {
            "string_value" => match node.parent()? {
                node => match node.kind() {
                    "string" => {
                        let parent = node.parent()?;
                        if parent.kind() != "argument" {
                            return None;
                        }

                        let parent = parent.parent()?;
                        if parent.kind() != "arguments" {
                            return None;
                        }

                        let parent = parent.parent()?;

                        if parent.kind() != "function_call_expression" {
                            return None;
                        }
                        Some(parent)
                    }
                    _ => None,
                },
            },
            _ => None,
        }?;

        let mut tree_cursor = function_expression_node.walk();
        let children = function_expression_node.children(&mut tree_cursor);

        for node in children {
            if node.kind() == "name" {
                return Some(&self.text[node.start_byte()..node.end_byte()]);
            }
        }

        None
    }
}

#[cfg(test)]
pub mod test_buffer {
    #[test]
    fn get_function() {
        let str = "<?php


        /*
        |--------------------------------------------------------------------------
        | Web Routes
        |--------------------------------------------------------------------------
        |
        | Here is where you can register web routes for your application. These
        | routes are loaded by the RouteServiceProvider and all of them will
        | be assigned to the \"web\" middleware group. Make something great!
        |
        */

        Route::get('/', function () {
            return view('welcome');
        })->name(\"homepage\");
        route('h')";

        let buffer = super::Buffer::new(str.to_string()).unwrap();
        let point = tree_sitter::Point {
            row: 17,
            column: 15,
        };

        println!("buffer: {:#?}", buffer.get_function(point));
    }
}
