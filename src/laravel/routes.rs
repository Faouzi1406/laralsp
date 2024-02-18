use std::process::Command;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Route {
    pub name: Option<String>,
    pub method: String,
}

impl Route {
    pub fn get() -> anyhow::Result<Vec<Route>> {
        let routes = Command::new("php")
            .arg("artisan")
            .arg("route:list")
            .arg("--json")
            .output()?;

        let routes = String::from_utf8(routes.stdout)?;

        let routes: Vec<Route> = serde_json::from_str(&routes)?;

        Ok(routes)
    }

    pub fn route_description(&self) -> String {
        format!(
            "
            {}
            ",
            &self.method
        )
    }
}
