use tower_lsp::lsp_types::{CompletionItem, CompletionResponse};
use tree_sitter::{Node, Point};

use crate::{buffer::Buffer, types::languages};

pub trait Completion {
    fn complete(&self, point: Point) -> anyhow::Result<Option<CompletionResponse>>;
}

pub type CompletionFunction = fn() -> anyhow::Result<CompletionResponse>;

fn complete_routes() -> anyhow::Result<CompletionResponse> {
    let Ok(config) = &crate::PROJECT_CONFIG.try_lock() else {
        return Err(anyhow::anyhow!("Was unable to lock config."));
    };

    let routes = &config.routes;
    let completions = routes
        .into_iter()
        .filter(|route| route.name.is_some())
        .map(|route| {
            CompletionItem::new_simple(
                route.name.as_ref().unwrap().clone(),
                route.route_description(),
            )
        })
        .collect();

    Ok(CompletionResponse::Array(completions))
}

fn complete_tables() -> anyhow::Result<CompletionResponse> {
    todo!("Table completion");
}

fn get_completion_function(
    language: &languages::Language,
    function: &str,
    ctx_node: Option<Node>,
) -> Option<CompletionFunction> {
    match language {
        languages::Language::PHP => match function {
            // Laravel route() function
            "route" => {
                if ctx_node?.kind() == "string_value" {
                    None
                } else {
                    Some(complete_routes)
                }
            }
            _ => None,
        },
    }
}

impl Completion for Buffer {
    fn complete(&self, point: Point) -> anyhow::Result<Option<CompletionResponse>> {
        match &self.language {
            languages::Language::PHP => {
                let function = match self.get_function_call(point) {
                    Some(func) => func,
                    None => return Ok(None),
                };

                let completion_function = match get_completion_function(&self.language, function, self.get_node_at_point(point)) {
                    Some(func) => func,
                    None => return Ok(None),
                };

                Ok(Some(completion_function()?))
            }
        }
    }
}
