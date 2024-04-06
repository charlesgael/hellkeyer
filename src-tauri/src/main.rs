// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use tauri::Manager;

use config::Config;
use stratagem::{get_stratagems, stratagem_filescheme};

use crate::bindings::{bind, get_bindings};
use crate::state::AppState;
use crate::stratagem::read_stratagems;

mod keydbkey_serde;
mod config;
mod stratagem;
mod state;
mod bindings;
mod window_mgmt;

fn main() {
    let config = Config::load();

    println!("{:?}", config);

    let stratagems = read_stratagems();

    let stratagems_map = stratagems.iter()
        .flat_map(|group| group)
        .map(|strat| (strat.name.clone(), strat.clone()))
        .collect::<HashMap<_,_>>();

    let state = AppState {
        config,
        stratagems,
        stratagems_map,
        ..Default::default()
    };

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_stratagems,
            get_bindings,
            bind
          ])
        .register_uri_scheme_protocol("stratagem", stratagem_filescheme)
        .manage(state)
        .setup(|app| {
            bindings::start_macro_handler(app);
            window_mgmt::move_window_bottom_left(app.get_window("main").unwrap());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}