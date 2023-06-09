// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;

use futures::executor::block_on;
use pangu_application::sslcert::SSLCertApplicationService;
use pangu_infras::repository::{
    app_data_path, db_conn_pool, init_logger, run_migrations, DnsProviderRepositoryImpl,
};
use pangu_infras::service::sslcert::SSLCertApplicationServiceImpl;
use pangu_infras::service::DnspodServiceImpl;
use std::fs;
use std::sync::Mutex;
use tauri::api::path;
use tauri::async_runtime;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// struct DbConnection {
//     db_pool: Mutex<&'static sqlx::Pool<sqlx::Sqlite>>,
// }

pub struct ApplicationService {
    pub sslcert_app_svc: Box<dyn SSLCertApplicationService + Send + Sync + 'static>,
}

fn init_lib() {
    let mut p = path::data_dir().unwrap();
    p = p.join("Pangu Studio");
    let dbpath = p.as_path().as_os_str().to_str().unwrap();
    fs::create_dir_all(p.clone()).unwrap();
    println!("data dir {:?}", dbpath);
    app_data_path(dbpath.to_string());
    init_logger(0);
    match async_runtime::block_on(run_migrations()) {
        Ok(_) => {
            println!("success");
        }
        Err(err) => {
            println!("error: {:?}", err)
        }
    };
}

fn main() {
    init_lib();
    // =================== wire ====================
    let db_pool = async_runtime::block_on(async { db_conn_pool().await.unwrap() });

    //===================== repository ======================
    let dns_provider_repo = DnsProviderRepositoryImpl::new(db_pool);

    //================= domain service ======================

    let dns_provider_service = DnspodServiceImpl::new(Box::new(dns_provider_repo.clone()));

    //================= application service ======================
    let app_svc = ApplicationService {
        sslcert_app_svc: Box::new(SSLCertApplicationServiceImpl::new(
            Box::new(dns_provider_service),
            Box::new(dns_provider_repo),
        )),
    };

    tauri::Builder::default()
        .manage(app_svc)
        .invoke_handler(tauri::generate_handler![
            greet,
            command::sslcert::list_dns_providers,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
