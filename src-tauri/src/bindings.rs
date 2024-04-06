use std::{thread, time};

use inputbot::KeybdKey::*;
use interception::{Interception, KeyState};
use interception::Stroke::Keyboard;
use tauri::{App, Manager, State};

use crate::state::{AppState, Mapping};

fn stratagem_bind(name: &String, state: &AppState) {
    let conf = &state.config;

    if let Some(strat) = state.stratagems_map.get(name) {
        let int = Interception::new().unwrap();

        let wait = time::Duration::from_millis(state.config.wait);
        println!("sequence {}", &strat.sequence);

        int.send(state.config.keyboard_id, &[Keyboard {
            code: conf.keys.stratagem,
            state: KeyState::DOWN,
            information: 1,
        }]);
        println!("start");
        thread::sleep(wait);
        println!("end");

        for x in strat.sequence.chars() {
            println!("{x}");
            if let Some(key) = match x {
                'U' => Some(conf.keys.up),
                'D' => Some(conf.keys.down),
                'L' => Some(conf.keys.left),
                'R' => Some(conf.keys.right),
                _ => None
            } {
                int.send(state.config.keyboard_id, &[Keyboard {
                    code: key,
                    state: KeyState::DOWN,
                    information: 1
                }]);
                thread::sleep(wait);
                int.send(state.config.keyboard_id, &[Keyboard {
                    code: key,
                    state: KeyState::UP,
                    information: 1
                }]);
                thread::sleep(wait);
            }
        }
        int.send(state.config.keyboard_id, &[Keyboard {
            code: conf.keys.stratagem,
            state: KeyState::UP,
            information: 1
        }]);

        // int.send(state.config.keyboard_id, strokes.as_slice());
    }
}

pub fn start_macro_handler(arg_app: &mut App) {
    let app = arg_app.handle();

    thread::spawn(move || {
        let app_f1 = app.app_handle();
        F1Key.block_bind(move || {
            let state1 = app_f1.state::<AppState>();
            stratagem_bind(&"reinforce".to_string(), &state1)
        });
        let app_f2 = app.app_handle();
        F2Key.block_bind(move || {
            let state2 = app_f2.state::<AppState>();
            stratagem_bind(&"resupply".to_string(), &state2)
        });

        let app_f5 = app.app_handle();
        F5Key.block_bind(move || {
            let state5 = app_f5.state::<AppState>();
            let map = &state5.mappings.read().unwrap();
            if let Some(key) = &map.f5 {
                stratagem_bind(key, &state5)
            }
        });
        let app_f6 = app.app_handle();
        F6Key.block_bind(move || {
            let state6 = app_f6.state::<AppState>();
            let map = &state6.mappings.read().unwrap();
            if let Some(key) = &map.f6 {
                stratagem_bind(key, &state6)
            }
        });
        let app_f7 = app.app_handle();
        F7Key.block_bind(move || {
            let state7 = app_f7.state::<AppState>();
            let map = &state7.mappings.read().unwrap();
            if let Some(key) = &map.f7 {
                stratagem_bind(key, &state7)
            }
        });
        let app_f8 = app.app_handle();
        F8Key.block_bind(move || {
            let state8 = app_f8.state::<AppState>();
            let map = &state8.mappings.read().unwrap();
            if let Some(key) = &map.f8 {
                stratagem_bind(key, &state8)
            }
        });
        let app_f9 = app.app_handle();
        F9Key.block_bind(move || {
            let state9 = app_f9.state::<AppState>();
            let map = &state9.mappings.read().unwrap();
            if let Some(key) = &map.f9 {
                stratagem_bind(key, &state9)
            }
        });
        let app_f10 = app.app_handle();
        F10Key.block_bind(move || {
            let state10 = app_f10.state::<AppState>();
            let map = &state10.mappings.read().unwrap();
            if let Some(key) = &map.f10 {
                stratagem_bind(key, &state10)
            }
        });

        let app_f11 = app.app_handle();
        F11Key.block_bind(move || {
            let w = app_f11.get_window("main").unwrap();

            if w.is_visible().unwrap() {
                w.hide().unwrap();
            } else {
                w.show().unwrap();
            }
        });

        inputbot::handle_input_events();
    });
}

#[tauri::command]
pub fn get_bindings(state: State<AppState>) -> Mapping {
    let map = state.mappings.read().unwrap();
    return map.clone();
}

#[tauri::command]
pub fn bind(key: String, stratagem: Option<String>, state: State<AppState>) {
    let mut map = state.mappings.write().unwrap();
    match (&key).as_str() {
        "f5" => map.f5 = stratagem.clone(),
        "f6" => map.f6 = stratagem.clone(),
        "f7" => map.f7 = stratagem.clone(),
        "f8" => map.f8 = stratagem.clone(),
        "f9" => map.f9 = stratagem.clone(),
        "f10" => map.f10 = stratagem.clone(),
        _ => {
            println!("Invalid key");
        }
    }
}