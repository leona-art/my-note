// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use once_cell::sync::Lazy;
use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};

static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|_| {
            tauri::async_runtime::block_on(async {
                DB.connect::<RocksDb>("./data.db").await.unwrap();
                DB.use_ns("personal").use_db("my-note").await.unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
