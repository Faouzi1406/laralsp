use tower_lsp::lsp_types::{CompletionItem, CompletionResponse};
use tree_sitter::Point;

use crate::{buffer::Buffer, types::languages};

pub trait Completion {
    fn complete(&self, point: Point) -> anyhow::Result<Option<CompletionResponse>>;
}

pub type CompletionFunction = fn() -> anyhow::Result<CompletionResponse>;

fn complete_routes() -> anyhow::Result<CompletionResponse> {
    let routes = &crate::PROJECT_CONFIG.lock().unwrap().routes;

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

fn get_completion_function(
    language: &languages::Language,
    function: &str,
) -> Option<CompletionFunction> {
    match language {
        languages::Language::PHP => match function {
            "route" => Some(complete_routes),
            _ => None,
        },
    }
}

impl Completion for Buffer {
    fn complete(&self, point: Point) -> anyhow::Result<Option<CompletionResponse>> {
        match &self.language {
            languages::Language::PHP => {
                let function = match self.get_function(point) {
                    Some(func) => func,
                    None => return Ok(None),
                };

                let completion_function = match get_completion_function(&self.language, function) {
                    Some(func) => func,
                    None => return Ok(None),
                };

                Ok(Some(completion_function()?))
            }
        }
    }
}
