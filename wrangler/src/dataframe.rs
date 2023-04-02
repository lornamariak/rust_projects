use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::error::Error;

[derive(Debug, Serialize, Deserialize)]
pub struct DataFrame {
    pub columns: Vec<String>,
    pub data: Vec<Vec<f64>>,
}

impl DataFrame {
    // read df
    pub fn read_csv(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;

        let headers = reader.headers()?.iter().map(|h| h.to_string()).collect();

        let mut data = Vec::new();
        for result in reader.deserialize() {
            let row: Vec<f64> = result?;
            data.push(row);
        }

        Ok(Self {
            columns: headers,
            data,
        })
    }
    // write df 
    pub fn write_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut writer = WriterBuilder::new()
            .has_headers(true)
            .from_path(file_path)?;

        writer.write_record(&self.columns)?;

        for row in &self.data {
            writer.write_record(row)?;
        }

        writer.flush()?;
        Ok(())
    }
    //select many fn
    pub fn select(&self, column_name: &str) -> Option<Vec<f64>> {
        match column_name {
            "a" => Some(self.data.iter().map(|row| row.a).collect()),
            "b" => Some(self.data.iter().map(|row| row.b).collect()),
            "c" => Some(self.data.iter().map(|row| row.c).collect()),
            _ => None,
        }
    }

    pub fn select_many(&self, column_names: &[&str]) -> Option<Vec<Vec<f64>>> {
        let mut result = Vec::new();
        for &column_name in column_names {
            if let Some(column_data) = self.select(column_name) {
                result.push(column_data);
            } else {
                return None;
            }
        }
        Some(result)
    }
}

