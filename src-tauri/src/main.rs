// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{result, vec};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::local::{Db, RocksDb},
    sql::Thing,
    Surreal,
};

/// 全体で共通のデータベースインスタンス
static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_topic])
        // DBの使用準備
        .setup(|_| {
            tauri::async_runtime::block_on(async {
                let mut dir = std::env::current_dir().unwrap();
                dir.push("database.db");
                DB.connect::<RocksDb>(dir).await.unwrap();
                DB.use_ns("personal").use_db("my-note").await.unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Topic {
    title: String,
    content: String,
}
impl Topic {
    fn new(title: &str, content: Option<&str>) -> Topic {
        Topic {
            title: title.to_string(),
            content: match content {
                Some(s) => s.to_string(),
                None => String::new(),
            },
        }
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct TopicWithId {
    id: Thing,
    #[serde(flatten)]
    topic: Topic,
}

#[tauri::command]
async fn create_topic(title: &str, content: Option<&str>) -> Result<String, String> {
    let result: Vec<TopicWithId> = DB
        .create("topic")
        .content(Topic::new(title, content))
        .await
        .unwrap();
    Ok(result[0].id.to_raw())
}

#[tauri::command]
async fn show_topic(id: Option<&str>) -> Result<Vec<TopicWithId>, String> {
    let resource = match id {
        Some(id) => id,
        None => "topic",
    };

    let mut resource = resource.split(":");
    match (resource.next(), resource.next()) {
        (Some(tb), Some(id)) => {
            let result: Option<TopicWithId> = DB.select((tb, id)).await.unwrap();
            Ok(match result {
                Some(topic) => vec![topic],
                None => Vec::new(),
            })
        }
        (Some(tb), None) => {
            let result = DB.select::<Vec<TopicWithId>>(tb).await.unwrap();
            Ok(result)
        }
        _ => Err("idの形式が違います。".to_string()),
    }
}

mod test {
    use std::vec;

    use super::*;

    #[test]
    fn connect_test() {
        tauri::async_runtime::block_on(async {
            let mut dir = std::env::current_dir().unwrap();
            dir.push("test.db");
            DB.connect::<RocksDb>(dir.clone()).await.unwrap();
            DB.use_ns("test").use_db("test").await.unwrap();
            std::fs::remove_dir_all(dir).unwrap();
        });
    }

    #[test]
    fn create_topic_test() {
        tauri::async_runtime::block_on(async {
            let mut dir = std::env::current_dir().unwrap();
            dir.push("test.db");
            DB.connect::<RocksDb>(dir.clone()).await.unwrap();
            DB.use_ns("test").use_db("test").await.unwrap();

            create_topic("title", Some("content")).await.unwrap();

            std::fs::remove_dir_all(dir).unwrap();
        });
    }

    #[test]
    fn show_topic_test() {
        tauri::async_runtime::block_on(async {
            let mut dir = std::env::current_dir().unwrap();
            dir.push("test.db");
            DB.connect::<RocksDb>(dir.clone()).await.unwrap();
            DB.use_ns("test").use_db("test").await.unwrap();

            let id = create_topic("title", Some("content")).await.unwrap();

            let result = show_topic(Some(&id)).await.unwrap();
            assert!(result.len() == 1);
            assert_eq!(result[0].id.to_raw(), id);

            std::fs::remove_dir_all(dir).unwrap();
        });
    }

    #[test]
    fn show_topics_test() {
        tauri::async_runtime::block_on(async {
            let mut dir = std::env::current_dir().unwrap();
            dir.push("test.db");
            if std::fs::metadata(&dir).is_ok() {
                std::fs::remove_dir_all(&dir).unwrap();
            }
            DB.connect::<RocksDb>(dir.clone()).await.unwrap();
            DB.use_ns("test").use_db("test").await.unwrap();
            let mut ids = Vec::new();
            for _ in 0..5 {
                let id = create_topic("title", Some("content")).await.unwrap();
                ids.push(id);
            }

            let result = show_topic(None).await.unwrap();

            assert!(result.len() == 5);
            for id in ids {
                assert!(result
                    .clone()
                    .into_iter()
                    .map(|e| e.id.to_raw())
                    .collect::<Vec<_>>()
                    .contains(&id));
            }

            std::fs::remove_dir_all(&dir).unwrap();
        });
    }
}
