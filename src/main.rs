use tauri::{command, Manager};
use uuid::Uuid;
use std::time::SystemTime;

#[command]
async fn scan_files(path: String) -> Result<Vec<String>, String> {
    Ok(file_ops::scan_directory(&path))
}

#[command]
async fn compute_hash(file_path: String) -> Result<String, String> {
    hash::compute_file_hash(&file_path).map_err(|e| e.to_string())
}

#[command]
async fn check_duplicate(hash: String) -> Result<bool, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    db::check_duplicate(&conn, &hash).map_err(|e| e.to_string())
}

#[command]
async fn insert_import_record(file_name: String, hash: String, export_path: String) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    let record = db::ImportRecord {
        id: Uuid::new_v4().to_string(),
        file_name,
        hash,
        import_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs().to_string(),
        export_path,
    };
    db::insert_record(&conn, &record).map_err(|e| e.to_string())
}

#[command]
async fn copy_files(files: Vec<(String, String)>) -> Result<(), String> {
    for (source, dest) in files {
        file_ops::copy_file(&source, &dest).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[command]
async fn create_directory(path: String) -> Result<(), String> {
    file_ops::create_directory(&path).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_files,
            compute_hash,
            check_duplicate,
            insert_import_record,
            copy_files,
            create_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}