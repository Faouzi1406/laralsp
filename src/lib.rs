use std::{collections::HashMap, sync::Mutex};

use document_state::DocumentState;

use crate::laravel::{project_config::ProjectConfig, routes::Route, tables::Tables};

pub mod actions;
pub mod buffer;
pub mod document_state;
pub mod laravel;
pub mod types;

lazy_static::lazy_static! {
    pub static ref PROJECT_CONFIG: Mutex<ProjectConfig> = Mutex::new(ProjectConfig {
        routes: Route::get().unwrap_or(Vec::new()),
        tables: Tables::get().unwrap_or(Tables { tables:Vec::new() })
    });


    pub static ref DOCUMENT_STATE: Mutex<DocumentState> = Mutex::new(DocumentState {
        documents: HashMap::new()
    });
}
