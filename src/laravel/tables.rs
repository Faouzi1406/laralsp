use std::process::Command;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Table {
    pub table: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tables {
    pub tables: Vec<Table>
}

impl Tables {
    pub fn get() -> anyhow::Result<Tables> {
        let tables = Command::new("php")
            .arg("artisan")
            .arg("db:show")
            .arg("--json")
            .output()?;

        let tables = String::from_utf8(tables.stdout)?;

        let tables: Tables = serde_json::from_str(&tables)?;

        Ok(tables)
    }
}

impl Table {
    pub fn table_description(&self) -> String {
        format!(
            "
            Table {}
            ",
            &self.table
        )
    }
}
