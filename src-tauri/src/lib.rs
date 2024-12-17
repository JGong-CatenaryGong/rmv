mod utils;
use crate::utils::fs_utils;
use serde::{Deserialize, Serialize};
use std::path::Path;
use utils::molecule::*;
use tklog::{debug, error, fatal, info, trace, warn, LEVEL, LOG};

fn jsonize_molecule(molecule: &molecule::Molecule) -> String {
    serde_json::to_string(&molecule).unwrap_or("Error: Could not serialize molecule".to_string())
}

fn mol_json_to_molecule(mol_json_str: &str) -> molecule::Molecule {
    let mol:molecule::Molecule = serde_json::from_str(mol_json_str).unwrap();
    mol
}

#[derive(Serialize, Deserialize)]
struct MolInfo {
    formula: String,
    no_atoms: usize,
    atoms: Vec<usize>,
    coordinates: Vec<[f64; 3]>,
    molecular_weight: f64,
    connections: (Vec<Vec<bool>>, Vec<Vec<bool>>, Vec<Vec<[f64;4]>>),
    draw_info: (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>),
}

impl MolInfo {
    pub fn new(
        formula: &str, 
        no_atoms: usize, 
        atoms: Vec<usize>,
        coordinates: Vec<[f64; 3]>,
        mol_weight: f64, 
        connections:(Vec<Vec<bool>>, Vec<Vec<bool>>, Vec<Vec<[f64;4]>>),  // VDW_mat, COV_mat, BOND_mat
        draw_info: (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>)  // hex_list, vdw_radii, cov_radii, cpk_radii
    ) -> Self {
        MolInfo {
            formula: formula.to_string(),
            no_atoms,
            atoms,
            coordinates,
            molecular_weight: mol_weight,
            connections,
            draw_info,
        }
    }
}

// FS utils

#[tauri::command]
fn jsonize_path(path: &str) -> String {
    let p = Path::new(path);
    let mut paths = Vec::new();
    fs_utils::visit_dirs(p, &mut |entry| paths.push(entry))
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));

    let json_value = serde_json::to_value(paths).unwrap();

    serde_json::to_string(&json_value).unwrap()
}

// Molecule utils

#[tauri::command]
fn load_mol(filename: &str) -> (String, String) {
    tklog::LOG.set_level(LEVEL::Debug);

    debug!(format!("Loading molecule from file: {}", filename));

    let mut molecule = molecule::Molecule::new(filename.to_string(), Vec::new(), Vec::new());

    if filename.ends_with(".xyz") {
        molecule = molecule::mol_from_xyz_file(filename);
    } else if filename.ends_with(".out") {
        molecule = molecule::mol_from_orca_output(filename);
    }

    let formula = molecule.get_formula();
    let no_atoms = molecule.get_atoms().iter().count();
    let mol_weight = molecule.get_molecular_weight();

    let vdw_matrix = molecule.get_vdw_matrix();
    let cov_matrix = molecule.get_cov_matrix();
    let bonds = molecule.calc_bond_positions();
    let connections = (vdw_matrix, cov_matrix, bonds);

    let draw_info = molecule.get_draw_info_list();

    let jsonize_mol_info = serde_json::to_string(&MolInfo::new(&formula, no_atoms, molecule.get_atoms(), molecule.get_coordinates(), mol_weight, connections, draw_info)).unwrap_or("Failed to jsonize".to_string());

    (jsonize_molecule(&molecule), jsonize_mol_info)
}

#[tauri::command]
fn load_mol_from_string(mol_string: &str, file_type: &str, filename: &str) -> String {
    let mut molecule = molecule::Molecule::new(filename.to_string(), Vec::new(), Vec::new());

    if file_type == "xyz" {
        molecule = molecule::mol_from_xyz_str(mol_string);
    } else if file_type == "out" {
        molecule = molecule::mol_from_orca_output_str(mol_string);
    }

    jsonize_molecule(&molecule)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            load_mol,
            load_mol_from_string,
            jsonize_path
        ])
        //.invoke_handler(tauri::generate_handler![jsonize_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
