// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{fs, io};

use calamine::{open_workbook, DeError, RangeDeserializerBuilder, Reader, Xlsx, XlsxError};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    De(#[from] DeError),
    #[error(transparent)]
    Xlsx(#[from] XlsxError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
}

// we must manually implement serde::Serialize
impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;

#[derive(Serialize, Deserialize)]
struct StockItem {
    barcode: String,
    product: String,
    quantity: i32,
}

#[tauri::command]
fn get_data(path: &str) -> CommandResult<String> {
    Ok(fs::read_to_string(path)?)
}

#[tauri::command]
fn update_data(path: &str, data: &str) -> CommandResult<()> {
    Ok(fs::write(path, data)?)
}

#[tauri::command]
fn import_excel_to_json(path: &str) -> CommandResult<String> {
    let mut item_list: Vec<StockItem> = Vec::new();
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let (_, sheet) = &workbook.worksheets()[0];
    let items = RangeDeserializerBuilder::new().from_range(&sheet)?;
    for item in items {
        item_list.push(item?)
    }
    Ok(serde_json::to_string(&item_list)?)
}
#[tauri::command]
fn export_json_to_csv(path: &str, data: &str) -> CommandResult<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    let item_list: Vec<StockItem> = serde_json::from_str(data)?;
    for item in item_list.iter() {
        wtr.serialize(item)?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_data,
            update_data,
            import_excel_to_json,
            export_json_to_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
