use std::path::PathBuf;

use tower_lsp::lsp_types::Url;

use crate::laravel::routes::Route;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ProjectConfig {
    pub routes: Vec<Route>,
}

const CONFIG_FILES: [&'static str; 1] = ["web.php"];

impl ProjectConfig {
    pub fn new() -> Self {
        ProjectConfig {
            routes: Route::get().unwrap_or(Vec::new()),
        }
    }

    // Updates the config if the config should update, based on the url.
    pub fn should_update_config(&mut self, url: Url) {
        let path: PathBuf = url.path().into();
        let Some(file) = path.file_name() else {
            return;
        };

        let Some(file) = file.to_str() else {
            return;
        };

        if CONFIG_FILES.contains(&file) {
            let old_routes = self.routes.clone();
            self.routes = Route::get().unwrap_or(old_routes);
        }
    }
}
