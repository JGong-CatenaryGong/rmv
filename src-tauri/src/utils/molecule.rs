pub mod molecule {
    use lazy_static::lazy_static;
    use rayon::iter::MaxLen;
    use crate::utils::file_processor;
    use std::collections::HashMap;
    use file_processor::file_reader::*;
    use serde::{Deserialize, Serialize};
    use tklog::{debug, error, fatal, info, trace, warn, LEVEL, LOG};
    use serde_json::Value;
    use std::f64::consts::PI;

    lazy_static! {
        pub static ref PERIODIC_TABLE: Value = serde_json::from_str(include_str!("../periodic_table/PeriodicTableJSON.json")).unwrap();
        pub static ref RADII: Value = serde_json::from_str(include_str!("../periodic_table/radii/radii.json")).unwrap();
        pub static ref ELEMENTS_USIZE: HashMap<&'static str, usize> = [
            ("H", 1),
            ("He", 2),
            ("Li", 3),
            ("Be", 4),
            ("B", 5),
            ("C", 6),
            ("N", 7),
            ("O", 8),
            ("F", 9),
            ("Ne", 10),
            ("Na", 11),
            ("Mg", 12),
            ("Al", 13),
            ("Si", 14),
            ("P", 15),
            ("S", 16),
            ("Cl", 17),
            ("Ar", 18),
            ("K", 19),
            ("Ca", 20),
            ("Sc", 21),
            ("Ti", 22),
            ("V", 23),
            ("Cr", 24),
            ("Mn", 25),
            ("Fe", 26),
            ("Co", 27),
            ("Ni", 28),
            ("Cu", 29),
            ("Zn", 30),
            ("Ga", 31),
            ("Ge", 32),
            ("As", 33),
            ("Se", 34),
            ("Br", 35),
            ("Kr", 36),
            ("Rb", 37),
            ("Sr", 38),
            ("Y", 39),
            ("Zr", 40),
            ("Nb", 41),
            ("Mo", 42),
            ("Tc", 43),
            ("Ru", 44),
            ("Rh", 45),
            ("Pd", 46),
            ("Ag", 47),
            ("Cd", 48),
            ("In", 49),
            ("Sn", 50),
            ("Sb", 51),
            ("Te", 52),
            ("I", 53),
            ("Xe", 54),
            ("Cs", 55),
            ("Ba", 56),
            ("La", 57),
            ("Ce", 58),
            ("Pr", 59),
            ("Nd", 60),
            ("Pm", 61),
            ("Sm", 62),
            ("Eu", 63),
            ("Gd", 64),
            ("Tb", 65),
            ("Dy", 66),
            ("Ho", 67),
            ("Er", 68),
            ("Tm", 69),
            ("Yb", 70),
            ("Lu", 71),
            ("Hf", 72),
            ("Ta", 73),
            ("W", 74),
            ("Re", 75),
            ("Os", 76),
            ("Ir", 77),
            ("Pt", 78),
            ("Au", 79),
            ("Hg", 80),
            ("Tl", 81),
            ("Pb", 82),
            ("Bi", 83),
            ("Po", 84),
            ("At", 85),
            ("Rn", 86),
            ("Fr", 87),
            ("Ra", 88),
            ("Ac", 89),
            ("Th", 90),
            ("Pa", 91),
            ("U", 92),
            ("Np", 93),
            ("Pu", 94),
            ("Am", 95),
            ("Cm", 96),
            ("Bk", 97),
            ("Cf", 98),
            ("Es", 99),
            ("Fm", 100),
            ("Md", 101),
            ("No", 102),
            ("Lr", 103),
            ("Rf", 104),
            ("Db", 105),
            ("Sg", 106),
            ("Bh", 107),
            ("Hs", 108),
            ("Mt", 109),
            ("Ds", 110),
            ("Rg", 111),
            ("Cn", 112),
            ("Nh", 113),
            ("Fl", 114),
            ("Mc", 115),
            ("Lv", 116),
            ("Ts", 117),
            ("Og", 118)
        ]
        .iter()
        .cloned()
        .collect();
    }

    mod vector_calc {
        pub fn vec_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
            [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
        }

        pub fn vec_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
            [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
        }

        pub fn vec_mod(a: [f64; 3]) -> f64 {
            (a[0].powi(2) + a[1].powi(2) + a[2].powi(2)).sqrt()
        }

        pub fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
            vec_mod(vec_sub(a, b))
        }
    }

    fn calc_direction_unit_vector(a: [f64; 3], b: [f64; 3]) -> (f64, f64, f64) {
        let dx = b[0] - a[0];
        let dy = b[1] - a[1];
        let dz = b[2] - a[2];
        let r = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();

        let dx1 = dx / r;
        let dy1 = dy / r;
        let dz1 = dz / r;

        let rotation_z = dz1;
        let rotation_x = dx1;
        let rotation_y = dy1;

        // let pitch = (dy1.atan() * 180.0);
        // let roll = if dx1.abs() > 0.998 || dz1.abs() > 0.998 {
        //     (-dx1).atan2(dz1)
        // } else {
        //     dx1.atan2(dz1)
        // };

        // let yaw = if(dx1 * dx1 + dz1 * dz1) > 0.001 {
        //     dx1.atan2(dy1)
        // } else {
        //     0.0
        // };

        // let pitch = pitch * 180.0) / std::f64::consts::PI;
        // let roll = (roll * 180.0) / std::f64::consts::PI;
        // let yaw = (yaw * 180.0) / std::f64::consts::PI;

        (rotation_x, rotation_y, rotation_z)
    }

    #[derive(Serialize, Deserialize)]
    pub struct Molecule {
        name: String,
        atoms: Vec<usize>,
        coordinates: Vec<[f64; 3]>,
    }

    impl Molecule {
        pub fn new(name: String, atoms: Vec<usize>, coordinates: Vec<[f64; 3]>) -> Self {
            if atoms.len() == coordinates.len() {
                return Molecule {
                    name,
                    atoms,
                    coordinates,
                };
            } else {
                warn!("WARN: Number of atoms and coordinates do not match");
                return Molecule {
                    name,
                    atoms,
                    coordinates,
                };
            }
        }

        fn __eq__(&self, other: &Molecule) -> bool {
            self.atoms == other.atoms && self.coordinates == other.coordinates
        }

        fn __repr__(&self) -> String {
            let no_positions = self.coordinates.len();
            let no_atoms = self.atoms.len();
            format!(
                "Molecule({}, with {} atoms in {} positions)",
                self.name, no_atoms, no_positions
            )
        }

        pub fn get_atoms(&self) -> Vec<usize> {
            self.atoms.clone()
        }

        pub fn get_coordinates(&self) -> Vec<[f64; 3]> {
            self.coordinates.clone()
        }

        fn is_isomer(&self, other: &Molecule) -> bool {
            self.atoms == other.atoms
        }

        fn gen_idx_matrix(&self) -> Vec<Vec<(usize, usize)>> {
            let rows = self.atoms.len();
            let matrix: Vec<Vec<(usize, usize)>> = (0..rows)
            .map(|i| {
                (0..rows)
                    .map(|j| (i, j))
                    .collect()
            })
            .collect();
            matrix
        }


        // fn get_vdw_threshold_matrix(&self) -> Vec<Vec<f64>> {
        //     let idx_matrix = self.gen_idx_matrix();
        //     let vdw_matrix = idx_matrix.iter().map(|row| {
        //         row.iter().map(|(i, j)| {
        //             let mut vdw_i = 0.0;
        //             let mut vdw_j = 0.0;
        //             if let Some(element) = RADII[self.atoms[*i] - 1].get("van der Waals"){
        //                 if let Some(element2) = RADII[self.atoms[*j] - 1].get("van der Waals"){
        //                     let atom_i = self.atoms[*i];
        //                     let atom_j = self.atoms[*j];
        //                     vdw_i = RADII[atom_i - 1]["van der Waals"].as_f64().unwrap_or(200.0);
        //                     vdw_j = RADII[atom_j - 1]["van der Waals"].as_f64().unwrap_or(200.0);
        //                 } else {
        //                     return 4.0
        //                 }
        //             } else {
        //                 return 4.0
        //             }
        //             return vdw_i + vdw_j
        //         }).collect()
        //     }
        //     ).collect();
        //     return vdw_matrix
        // }

        fn get_vdw_threshold_matrix(&self) -> Vec<Vec<f64>> {
            let mut vdw_matrix = vec![vec![0.0; self.coordinates.len()]; self.coordinates.len()];
            for i in  0..self.coordinates.len() {
                for j in 0..self.coordinates.len() {
                    if let Some(element) = RADII[self.atoms[i] - 1].get("van der Waals"){
                        if let Some(element2) = RADII[self.atoms[j] - 1].get("van der Waals"){
                            let atom_no_i = self.atoms[i];
                            let atom_no_j = self.atoms[j];
                            vdw_matrix[i][j] = (RADII[atom_no_i - 1]["van der Waals"].as_f64().unwrap_or(200.0) + RADII[atom_no_j - 1]["van der Waals"].as_f64().unwrap_or(200.0)) / 100.0;
                        } else {
                            vdw_matrix[i][j] = 4.0;
                        }
                    } else {
                        vdw_matrix[i][j] = 4.0;
                    }
                }
            }
            vdw_matrix
        }

        fn get_cov_threshold_matrix(&self) -> Vec<Vec<f64>> {
            let mut cov_matrix = vec![vec![0.0; self.coordinates.len()]; self.coordinates.len()];
            for i in 0..self.coordinates.len() {
                for j in 0..self.coordinates.len() {
                    if let Some(element) = RADII[self.atoms[i] - 1].get("Covalent (single bond)"){
                        if let Some(element2) = RADII[self.atoms[j] - 1].get("Covalent (single bond)"){
                            let atom_no_i = self.atoms[i];
                            let atom_no_j = self.atoms[j];
                            cov_matrix[i][j] = (RADII[atom_no_i - 1]["Covalent (single bond)"].as_f64().unwrap_or(100.0) + RADII[atom_no_j - 1]["Covalent (single bond)"].as_f64().unwrap_or(100.0)) / 100.0;
                        } else {
                            cov_matrix[i][j] = 2.0;
                        }
                    } else {
                        cov_matrix[i][j] = 2.0;
                    }
                }
            }
            cov_matrix
        }

        pub fn get_distance_matrix(&self) -> Vec<Vec<f64>> {
            let mut distance_matrix =
                vec![vec![0.0; self.coordinates.len()]; self.coordinates.len()];
            for i in 0..self.coordinates.len() {
                for j in 0..self.coordinates.len() {
                    if i == j {
                        distance_matrix[i][j] = 0.0
                    } else {
                        distance_matrix[i][j] =
                            vector_calc::distance(self.coordinates[i], self.coordinates[j]);
                    }
                }
            }
            return distance_matrix;
        }

        pub fn get_vdw_matrix (&self) -> Vec<Vec<bool>> {
            let mut vdw_matrix = vec![vec![false; self.coordinates.len()]; self.coordinates.len()];
            let distance_matrix = self.get_distance_matrix();
            let vdw_threshold_matrix = self.get_vdw_threshold_matrix();
            let cov_threshold_matrix = self.get_cov_threshold_matrix();

            let atoms = self.get_atoms();

            for i in 0..self.coordinates.len() {
                for j in 0..self.coordinates.len() {
                    if i == j {
                        vdw_matrix[i][j] = false
                    } else if distance_matrix[i][j] < vdw_threshold_matrix[i][j] * 0.7 && distance_matrix[i][j] > cov_threshold_matrix[i][j] * 1.1 {
                        if atoms[i] == 1 && atoms[j] != 6 { // Hydrogen bond
                            vdw_matrix[i][j] = true
                        } 
                    }
                }
            }
            vdw_matrix
        }

        pub fn get_cov_matrix(&self) -> Vec<Vec<bool>> {
            let mut cov_matrix = vec![vec![false; self.coordinates.len()]; self.coordinates.len()];
            let distance_matrix = self.get_distance_matrix();
            let cov_threshold_matrix = self.get_cov_threshold_matrix();
            for i in 0..self.coordinates.len() {
                for j in 0..self.coordinates.len() {
                    if i == j {
                        cov_matrix[i][j] = false
                    } else {
                        cov_matrix[i][j] = distance_matrix[i][j] < cov_threshold_matrix[i][j] * 1.1;
                    }
                }
            }
            cov_matrix
        }

        pub fn calc_bond_positions(&self) -> Vec<Vec<[f64; 7]>> {
            let mut bond_positions: Vec<Vec<[f64; 7]>> = vec![vec![[0.0; 7]; self.coordinates.len()]; self.coordinates.len()];
            let vdw_matrix = self.get_vdw_matrix();
            let cov_matrix = self.get_cov_matrix();
            for i in 0..self.coordinates.len() {
                for j in 0..self.coordinates.len() {
                    if i != j && cov_matrix[i][j] {
                        bond_positions[i][j][0] = (self.coordinates[i][0] + self.coordinates[j][0]) / 2.0;
                        bond_positions[i][j][1] = (self.coordinates[i][1] + self.coordinates[j][1]) / 2.0;
                        bond_positions[i][j][2] = (self.coordinates[i][2] + self.coordinates[j][2]) / 2.0;
                        bond_positions[i][j][3] = 1.0;

                        (bond_positions[i][j][4], bond_positions[i][j][5], bond_positions[i][j][6]) = calc_direction_unit_vector(self.coordinates[i], self.coordinates[j]);// For bond orientation
                    } else if vdw_matrix[i][j] {
                        bond_positions[i][j][0] = (self.coordinates[i][0] + self.coordinates[j][0]) / 2.0;
                        bond_positions[i][j][1] = (self.coordinates[i][1] + self.coordinates[j][1]) / 2.0;
                        bond_positions[i][j][2] = (self.coordinates[i][2] + self.coordinates[j][2]) / 2.0;
                        bond_positions[i][j][3] = 0.5;

                        (bond_positions[i][j][4], bond_positions[i][j][5], bond_positions[i][j][6]) = calc_direction_unit_vector(self.coordinates[i], self.coordinates[j]);// For bond orientation
                    }
                }
            }
            bond_positions
        }

        pub fn get_draw_info_list(&self) -> (Vec<String>, Vec<f64>, Vec<f64>, Vec<f64>) {
            let no_atoms = self.coordinates.len();
            let atom_list = self.get_atoms().clone();
            // let mut cpk_hex_list = vec![""; no_atoms];
            // let mut vdw_radii_list = vec![0.0; no_atoms];
            let mut cov_radii_list = vec![0.0; no_atoms];
            let mut cpk_radii_list = vec![0.0; no_atoms];

            let cpk_hex_list = atom_list.iter().map(
                |x| if let Some(elements) = PERIODIC_TABLE.get("elements") {
                    if let Some(element) = elements.get(x - 1) {
                        if let Some(cpk_hex) = element.get("cpk-hex") {
                            cpk_hex.as_str().unwrap_or("90e050").to_string()
                        } else {
                            "90e050".to_string()
                        }
                    } else {
                        "90e050".to_string()
                    }
                } else {
                    "90e050".to_string()
                }).collect();

            let vdw_radii_list: Vec<f64> = atom_list.iter().map(
                |x| if let Some(element) = RADII.get(x - 1) {
                    if let Some(vdw_radii) = element.get("van der Waals") {
                        vdw_radii.as_f64().unwrap_or(2.0)
                    } else {
                        2.0
                    }
                } else {
                        2.0
                }).collect();

            let cov_radii_list: Vec<f64> = atom_list.iter().map(
                |x| if let Some(element) = RADII.get(x - 1) {
                    if let Some(cov_radii) = element.get("Covalent (single bond)") {
                        cov_radii.as_f64().unwrap_or(1.0)
                    } else {
                        1.0
                    }
                } else {
                    1.0
                }).collect();

            let cpk_radii_list: Vec<f64> = (0..no_atoms).map(
                |x| (vdw_radii_list[x] + cov_radii_list[x]) / 6.0).collect();

            (cpk_hex_list, vdw_radii_list, cov_radii_list, cpk_radii_list)
        }

        fn get_coulomb_matrix(&self) -> Vec<Vec<f64>> {
            let mut coulomb_matrix =
                vec![vec![0.0; self.coordinates.len()]; self.coordinates.len()];
            for i in 0..self.coordinates.len() {
                for j in 0..self.coordinates.len() {
                    if i == j {
                        coulomb_matrix[i][j] = 0.0
                    } else {
                        coulomb_matrix[i][j] = (self.atoms[i] * self.atoms[j]) as f64
                            / vector_calc::distance(self.coordinates[i], self.coordinates[j]);
                    }
                }
            }
            return coulomb_matrix;
        }

        pub fn get_formula(&self) -> String {
            let mut formula = String::new();
            for element in ELEMENTS_USIZE.keys() {
                let mut count = 0;
                let var = *ELEMENTS_USIZE.get(element).unwrap();
                for j in 0..self.atoms.len() {
                    if self.atoms[j] == var {
                        count += 1;
                    }
                }
                if count > 0 {
                    formula.push_str(&format!("{}{}", element, count));
                }
            }
            return formula;
        }

        pub fn get_molecular_weight(&self) -> f64 {
            let mut weight = 0.0;
            for i in 0..self.atoms.len() {
                if let Some(elements) = PERIODIC_TABLE.get("elements") {
                    if let Some(element) = elements.get(&self.atoms[i] - 1) {
                        if let Some(atomic_mass) = element.get("atomic_mass") {
                            weight += atomic_mass.as_f64().unwrap_or(0.0);
                        }
                    }
                }
            }
            return weight;
        }
    }

    pub fn mol_from_xyz_file(filename: &str) -> Molecule {
        let mol = file_reader::read_xyz_file(filename);
        return Molecule {
            name: filename.to_string(),
            atoms: mol.0,
            coordinates: mol.1,
        };
    }

    pub fn mol_from_xyz_str(content: &str) -> Molecule {
        let mol = file_reader::read_xyz_str(content);
        return Molecule {
            name: "xyz".to_string(),
            atoms: mol.0,
            coordinates: mol.1,
        };
    }

    fn geom_to_mol(geom: (Vec<usize>, Vec<[f64; 3]>)) -> Molecule {
        return Molecule {
            name: "geom".to_string(),
            atoms: geom.0,
            coordinates: geom.1,
        };
    }

    pub fn mol_from_orca_output(filename: &str) -> (Molecule, Vec<(String, f64)>) {
        let (geoms, calculated_infos) = file_reader::read_orca_output(filename);
        let len = geoms.len();
        let mol: (Vec<usize>, Vec<[f64; 3]>) = geoms[len - 1].clone();
        let calculated_info = calculated_infos[len - 1].clone();
        return (Molecule {
            name: filename.to_string(),
            atoms: mol.0,
            coordinates: mol.1,
        }, calculated_info);
    }

    pub fn mol_from_gaussian_output(filename: &str) -> (Molecule, Vec<(String, f64)>) {
        let (geoms, calculated_infos) = file_reader::read_gaussian_output(filename);
        let len = geoms.len();
        let mol: (Vec<usize>, Vec<[f64; 3]>) = geoms[len - 1].clone();
        let len_info = calculated_infos.len();
        let calculated_info = calculated_infos[len_info - 1].clone();
        return (Molecule {
            name: filename.to_string(),
            atoms: mol.0,
            coordinates: mol.1,
        }, calculated_info)
    }

    pub fn mol_from_orca_output_str(content: &str) -> Molecule {
        let geoms: Vec<(Vec<usize>, Vec<[f64; 3]>)> = file_reader::read_orca_output_str(content);
        let mol: (Vec<usize>, Vec<[f64; 3]>) = geoms[0].clone();
        return Molecule {
            name: "orca".to_string(),
            atoms: mol.0,
            coordinates: mol.1,
        };
    }

    pub fn mols_from_orca_trajectory(filename: &str) -> (Vec<Molecule>, Vec<Vec<(String, f64)>>) {
        let (geoms, calculated_infos) = file_reader::read_orca_output(filename);
        let mols: Vec<Molecule> = geoms
            .into_iter()
            .map(|geom| geom_to_mol(geom.clone()))
            .collect();

        (mols, calculated_infos)
    }

    pub fn mols_from_orca_trajectory_str(content: &str) -> Vec<Molecule> {
        let geoms: Vec<(Vec<usize>, Vec<[f64; 3]>)> = file_reader::read_orca_output_str(content);
        let mols: Vec<Molecule> = geoms
            .into_iter()
            .map(|geom| geom_to_mol(geom.clone()))
            .collect();

        mols
    }

    pub fn mols_from_xyz_trajectory(filename: &str) -> Vec<Molecule> {
        let geoms: Vec<(Vec<usize>, Vec<[f64; 3]>)> = file_reader::read_xyz_trajectory(filename);
        let mols: Vec<Molecule> = geoms
            .into_iter()
            .map(|geom| geom_to_mol(geom.clone()))
            .collect();

        mols
    }

    pub fn mols_from_xyz_trajectory_str(content: &str) -> Vec<Molecule> {
        let geoms: Vec<(Vec<usize>, Vec<[f64; 3]>)> = file_reader::read_xyz_trajectory_str(content);
        let mols: Vec<Molecule> = geoms
            .into_iter()
            .map(|geom| geom_to_mol(geom.clone()))
            .collect();

        mols
    }
}
