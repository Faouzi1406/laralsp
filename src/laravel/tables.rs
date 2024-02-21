use std::process::Command;

use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Column {
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Table {
    pub table: String,
    pub columns: Option<Vec<Column>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Tables {
    pub tables: Vec<Table>,
}

impl Tables {
    pub fn get() -> anyhow::Result<Tables> {
        let tables = Command::new("php")
            .arg("artisan")
            .arg("db:show")
            .arg("--json")
            .output()?;

        let tables = String::from_utf8(tables.stdout)?;

        let mut tables: Tables = serde_json::from_str(&tables)?;
        for table in tables.tables.iter_mut() {
            _ = table.load_columns();
        }

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

    /// Loads in the columns of the table into the table,
    pub fn load_columns(&mut self) -> anyhow::Result<()> {
        let tables = Command::new("php")
            .arg("artisan")
            .arg("db:list")
            .arg(&self.table)
            .arg("--json")
            .output()?;

        let str_result = String::from_utf8(tables.stdout)?;
        let json: Value = serde_json::from_str(&str_result)?;

        let Some(columns_json) = json.get("columns") else {
            anyhow::bail!("No columns field was present.")
        };

        let Some(obj) = columns_json.as_object() else {
            anyhow::bail!("No columns field was present.")
        };

        let mut columns: Vec<Column> = Vec::new();
        for (name, _) in obj.iter() {
            columns.push(Column {
                name: name.to_owned(),
            });
        }

        Ok(())
    }

    pub fn get_colums(&self) -> Option<&Vec<Column>> {
        self.columns.as_ref()
    }
}
