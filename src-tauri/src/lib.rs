use serde::Serialize;
use splr::*;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Clone)]
struct Cell {
    value: i32,
}

#[derive(Serialize)]
struct Map {
    x: i32,
    y: i32,
    data: Vec<Vec<Cell>>,
}

#[tauri::command]
async fn get_map() -> Map {
    Map {
        x: 10,
        y: 1,
        data: vec![vec![Cell { value: 0 }; 10]; 10],
    }
}

fn add_near(solver: &mut Solver, near: Vec<i32>, num: usize) {
    for i in 0..near.len() {
        for j in 0..near.len() {
            if j - i + 1 > num {
                let _ = solver.add_clause(vec![-near[i], -near[j]]);
            }
        }
    }
}

#[tauri::command]
async fn get_splr() -> String {
    let mut x = 3;
    let mut y = 3;
    let mut rules: Vec<Vec<i32>> = Vec::new();
    let config = splr::Config {
        splr_interface: true,
        quiet_mode: false,
        ..Default::default()
    };
    let mut solver = Solver::try_from((config, rules.as_ref())).expect("panic");
    for ans in solver.iter().take(1) {
        println!("found!");
        println!("{:?}", ans);
    }
    "Hello, world!".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_map, get_splr])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
