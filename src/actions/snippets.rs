use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, CompletionResponse};

pub struct Snippet {
    label: &'static str,
    detail: &'static str,
    insert_text: &'static str,
}

const SNIPPETS: [Snippet; 1] = [
    Snippet {
        insert_text: "
class MyController extends BaseController {
    public function index() {
        return 'Hello, world!';
    }
}
",
        label: "ctrler",
        detail: "Snippet",
    },
];

pub fn get_snippets() -> CompletionResponse {
    let new_item = |label: &str, detail, insert_text: &str| CompletionItem {
        label: label.to_string(),
        insert_text:  Some(insert_text.into()),
        detail: Some(detail),
        kind: Some(CompletionItemKind::SNIPPET),
        ..CompletionItem::default()
    };

    let arr = SNIPPETS.map(|snippet| {
        new_item(snippet.label, snippet.detail.into(), snippet.insert_text.into())
    });

    CompletionResponse::Array(arr.to_vec())

}
