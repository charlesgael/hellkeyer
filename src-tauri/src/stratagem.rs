use std::fs::{File, read};
use std::io::Read;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State, Url};
use tauri::http::{Request, Response as HttpResponse, ResponseBuilder};

use crate::state::AppState;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Stratagem {
    pub name: String,
    icon: String,
    pub sequence: String,
    group: i32,
}

pub fn read_stratagem(name: &String, group: i32) -> Result<Stratagem, String> {
    match File::open(format!("data/stratagems/{name}/sequence.txt")) {
        Ok(mut file) => {
            let mut seq = String::new();
            file.read_to_string(&mut seq).unwrap();
            Ok(Stratagem {
                name: name.clone(),
                icon: format!("https://stratagem.local/{name}"),
                sequence: seq,
                group,
            })
        }
        Err(e) => {
            Err(format!("{e:?}"))
        }
    }
}

#[tauri::command]
pub fn read_stratagems() -> Vec<Vec<Stratagem>> {
    let groups: Vec<Vec<String>> = {
        if let Ok(file) = File::open("data/groups.yml") {
            serde_yaml::from_reader(file).unwrap()
        } else {
            Vec::new()
        }
    };

    let strats: Vec<Vec<Stratagem>> = groups.iter()
        .enumerate()
        .map(|(index, group)| {
            group.iter()
                .map(|stratagem| read_stratagem(stratagem, index as i32).unwrap())
                .collect()
        })
        .collect();

    return strats;
}

#[tauri::command]
pub fn get_stratagems(state: State<AppState>) -> Vec<Vec<Stratagem>> {
    return state.stratagems.clone();
}

pub fn stratagem_filescheme(_app: &AppHandle, req: &Request) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let url: Url = req.uri().parse().unwrap();

    let stratagem = &url.path()[1..];

    match read(format!("data/stratagems/{stratagem}/icon.png")) {
        Ok(data) => {
            ResponseBuilder::new()
                .status(200)
                .mimetype("image/png")
                .body(data)
        }
        _ => {
            ResponseBuilder::new()
                .status(200)
                .mimetype("image/png")
                .body(read("data/noicon.png").unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::stratagem::{get_stratagems, read_stratagem};

    #[test]
    fn test_get_stratagem() {
        println!("anti_materiel_rifle: {:?}", read_stratagem(&"anti_materiel_rifle".to_string(), 0))
    }


    #[test]
    fn test_get_stratagems() {
        println!("stratagems: {:?}", get_stratagems());
    }
}