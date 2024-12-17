pub mod file_reader {
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use tklog::{debug, error, fatal, info, trace, warn, LEVEL, LOG};

    lazy_static! {
        pub static ref ELEMENTS: HashMap<&'static str, f64> = [
            ("H", 1.0),
            ("He", 2.0),
            ("Li", 3.0),
            ("Be", 4.0),
            ("B", 5.0),
            ("C", 6.0),
            ("N", 7.0),
            ("O", 8.0),
            ("F", 9.0),
            ("Ne", 10.0),
            ("Na", 11.0),
            ("Mg", 12.0),
            ("Al", 13.0),
            ("Si", 14.0),
            ("P", 15.0),
            ("S", 16.0),
            ("Cl", 17.0),
            ("Ar", 18.0),
            ("K", 19.0),
            ("Ca", 20.0),
            ("Sc", 21.0),
            ("Ti", 22.0),
            ("V", 23.0),
            ("Cr", 24.0),
            ("Mn", 25.0),
            ("Fe", 26.0),
            ("Co", 27.0),
            ("Ni", 28.0),
            ("Cu", 29.0),
            ("Zn", 30.0),
            ("Ga", 31.0),
            ("Ge", 32.0),
            ("As", 33.0),
            ("Se", 34.0),
            ("Br", 35.0),
            ("Kr", 36.0),
            ("Rb", 37.0),
            ("Sr", 38.0),
            ("Y", 39.0),
            ("Zr", 40.0),
            ("Nb", 41.0),
            ("Mo", 42.0),
            ("Tc", 43.0),
            ("Ru", 44.0),
            ("Rh", 45.0),
            ("Pd", 46.0),
            ("Ag", 47.0),
            ("Cd", 48.0),
            ("In", 49.0),
            ("Sn", 50.0),
            ("Sb", 51.0),
            ("Te", 52.0),
            ("I", 53.0),
            ("Xe", 54.0),
            ("Cs", 55.0),
            ("Ba", 56.0),
            ("La", 57.0),
            ("Ce", 58.0),
            ("Pr", 59.0),
            ("Nd", 60.0),
            ("Pm", 61.0),
            ("Sm", 62.0),
            ("Eu", 63.0),
            ("Gd", 64.0),
            ("Tb", 65.0),
            ("Dy", 66.0),
            ("Ho", 67.0),
            ("Er", 68.0),
            ("Tm", 69.0),
            ("Yb", 70.0),
            ("Lu", 71.0),
            ("Hf", 72.0),
            ("Ta", 73.0),
            ("W", 74.0),
            ("Re", 75.0),
            ("Os", 76.0),
            ("Ir", 77.0),
            ("Pt", 78.0),
            ("Au", 79.0),
            ("Hg", 80.0),
            ("Tl", 81.0),
            ("Pb", 82.0),
            ("Bi", 83.0),
            ("Po", 84.0),
            ("At", 85.0),
            ("Rn", 86.0),
            ("Fr", 87.0),
            ("Ra", 88.0),
            ("Ac", 89.0),
            ("Th", 90.0),
            ("Pa", 91.0),
            ("U", 92.0),
            ("Np", 93.0),
            ("Pu", 94.0),
            ("Am", 95.0),
            ("Cm", 96.0),
            ("Bk", 97.0),
            ("Cf", 98.0),
            ("Es", 99.0),
            ("Fm", 100.0),
            ("Md", 101.0),
            ("No", 102.0),
            ("Lr", 103.0),
            ("Rf", 104.0),
            ("Db", 105.0),
            ("Sg", 106.0),
            ("Bh", 107.0),
            ("Hs", 108.0),
            ("Mt", 109.0),
            ("Ds", 110.0),
            ("Rg", 111.0),
            ("Cn", 112.0),
            ("Nh", 113.0),
            ("Fl", 114.0),
            ("Mc", 115.0),
            ("Lv", 116.0),
            ("Ts", 117.0),
            ("Og", 118.0)
        ]
        .iter()
        .cloned()
        .collect();
    }
    fn symbol_to_atomic_number(symbol: &str) -> Option<f64> {
        ELEMENTS.get(symbol).cloned()
    }

    fn element_atomic_parser(element: &str) -> f64 {
        element
            .parse::<f64>()
            .unwrap_or(symbol_to_atomic_number(element).unwrap_or(0.0))
    }

    pub fn read_xyz_str(content: &str) -> (Vec<usize>, Vec<[f64; 3]>) {
        let mut lines: Vec<String> = Vec::new();

        for line in content.lines() {
            lines.push(line.to_string());
        }

        let mut no_atoms: u32 = 0;
        let mut atoms: Vec<usize> = Vec::new();
        let mut coordinates: Vec<[f64; 3]> = Vec::new();

        for (i, line) in lines.into_iter().enumerate() {
            debug!(format!("Reading the {}th line", i + 1));
            if i == 0 {
                let no_atoms: u32 = line.parse::<u32>().unwrap();
                debug!(format!("Number of atoms: {}", no_atoms));
            } else if i >= 2 {
                debug!(format!("{}", line.clone()));
                let l = line
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|s| element_atomic_parser(s))
                    .collect::<Vec<f64>>();
                atoms.push(l[0] as usize);
                coordinates.push([l[1], l[2], l[3]]);
            }
        }

        (atoms, coordinates)
    }

    pub fn read_orca_output_str(content: &str) -> Vec<(Vec<usize>, Vec<[f64; 3]>)> {
        tklog::LOG.set_level(LEVEL::Info);
        let mut lines: Vec<String> = Vec::new();
        for line in content.lines() {
            lines.push(line.to_string());
        }

        let version_indicator: Regex = Regex::new(r"[0-9]*.[0-9]*.[0-9]*").unwrap();
        let mut orca_version = "".to_string();

        let mut no_atoms: u32 = 0;

        let mut coords_idxs: Vec<(usize, usize)> = Vec::new();
        let mut block_idx: (usize, usize) = (0, 0);

        for (i, line) in lines.iter().enumerate() {
            if line.contains("Program Version ") {
                orca_version = version_indicator.find(line).unwrap().as_str().to_string();
                info!(format!("ORCA version: {}", orca_version));
            } else if line.contains("CARTESIAN COORDINATES (ANGSTROEM)") {
                debug!(format!("Found coord start at line {}", i + 2));
                block_idx.0 = i + 2;
            } else if line.contains("CARTESIAN COORDINATES (A.U.)") {
                debug!(format!("Found coord end at line {}", i - 3));
                block_idx.1 = i - 3;
                debug!(format!("Push the block: {:?}", block_idx));
                coords_idxs.push(block_idx);
            }
        }

        let mut geoms: Vec<(Vec<usize>, Vec<[f64; 3]>)> = Vec::new();

        for coords_idx in coords_idxs {
            let mut atoms: Vec<usize> = Vec::new();
            let mut coordinates: Vec<[f64; 3]> = Vec::new();

            let coords_block = lines[coords_idx.0..coords_idx.1].to_vec();
            for coord in coords_block {
                let items = coord
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .filter(|x| !x.is_empty())
                    .map(|s| element_atomic_parser(s))
                    .collect::<Vec<f64>>();
                atoms.push(items[0] as usize);
                coordinates.push([items[1], items[2], items[3]]);
            }

            geoms.push((atoms, coordinates))
        }

        geoms
    }

    pub fn read_xyz_trajectory_str(content: &str) -> Vec<(Vec<usize>, Vec<[f64; 3]>)> {
        tklog::LOG.set_level(LEVEL::Info);

        let mut lines: Vec<String> = Vec::new();
        for line in content.lines() {
            lines.push(line.to_string());
        }

        let n_atoms: usize = lines[0].parse().unwrap();
        let n_geoms: usize = lines.len() / (n_atoms + 2);

        let mut geoms = Vec::new();

        for i in 0..n_geoms {
            let mut atoms = Vec::new();
            let mut coordinates = Vec::new();

            for j in 0..n_atoms {
                let line = &lines[i * (n_atoms + 2) + j + 2];
                let items: Vec<f64> = line
                    .split_whitespace()
                    .filter(|x| !x.is_empty())
                    .map(|s| element_atomic_parser(s))
                    .collect();

                atoms.push(items[0] as usize);
                coordinates.push([items[1], items[2], items[3]]);
            }

            geoms.push((atoms, coordinates));
        }

        geoms
    }

    pub fn read_xyz_file(file_path: &str) -> (Vec<usize>, Vec<[f64; 3]>) {
        tklog::LOG.set_level(LEVEL::Info);

        let file = File::open(file_path).expect("File not found");
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();

        for line in reader.lines() {
            lines.push(line.unwrap());
        }

        //let no_atoms:u32 = buffered.lines()[0].parse::<u32>().unwrap();
        //let mut atoms = vec![0; no_atoms as usize];
        //let mut coordinates = vec![[0.0; 3]; no_atoms as usize];

        let mut no_atoms: u32 = 0;
        let mut atoms: Vec<usize> = Vec::new();
        let mut coordinates: Vec<[f64; 3]> = Vec::new();

        //     let things = l.unwrap().split_whitespace().collect::<Vec<&str>>();
        // }

        for (i, line) in lines.into_iter().enumerate() {
            debug!(format!("Reading the {}th line", i + 1));
            if i == 0 {
                no_atoms = line.parse::<u32>().unwrap();
                debug!(format!("Number of atoms: {}", no_atoms));
            } else if i >= 2 && i <= no_atoms as usize + 1 {
                debug!(format!("{}", line.clone()));
                let l = line
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|s| element_atomic_parser(s))
                    .collect::<Vec<f64>>();
                atoms.push(l[0] as usize);
                coordinates.push([l[1], l[2], l[3]]);
            }
        }

        (atoms, coordinates)
    }

    pub fn read_orca_output(filename: &str) -> Vec<(Vec<usize>, Vec<[f64; 3]>)> {
        tklog::LOG.set_level(LEVEL::Info);

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            lines.push(line.unwrap());
        }

        let version_indicator: Regex = Regex::new(r"[0-9]*.[0-9]*.[0-9]*").unwrap();
        let mut orca_version = "".to_string();

        let mut no_atoms: u32 = 0;

        let mut coords_idxs: Vec<(usize, usize)> = Vec::new();
        let mut block_idx: (usize, usize) = (0, 0);

        for (i, line) in lines.iter().enumerate() {
            if line.contains("Program Version ") {
                orca_version = version_indicator.find(line).unwrap().as_str().to_string();
                info!(format!("ORCA version: {}", orca_version));
            } else if line.contains("CARTESIAN COORDINATES (ANGSTROEM)") {
                debug!(format!("Found coord start at line {}", i + 2));
                block_idx.0 = i + 2;
            } else if line.contains("CARTESIAN COORDINATES (A.U.)") {
                debug!(format!("Found coord end at line {}", i - 3));
                block_idx.1 = i - 3;
                debug!(format!("Push the block: {:?}", block_idx));
                coords_idxs.push(block_idx);
            }
        }

        let mut geoms: Vec<(Vec<usize>, Vec<[f64; 3]>)> = Vec::new();

        for coords_idx in coords_idxs {
            let mut atoms: Vec<usize> = Vec::new();
            let mut coordinates: Vec<[f64; 3]> = Vec::new();

            let coords_block = lines[coords_idx.0..coords_idx.1].to_vec();
            for coord in coords_block {
                let items = coord
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .filter(|x| !x.is_empty())
                    .map(|s| element_atomic_parser(s))
                    .collect::<Vec<f64>>();
                atoms.push(items[0] as usize);
                coordinates.push([items[1], items[2], items[3]]);
            }

            geoms.push((atoms, coordinates))
        }

        geoms
    }

    pub fn read_xyz_trajectory(filename: &str) -> Vec<(Vec<usize>, Vec<[f64; 3]>)> {
        tklog::LOG.set_level(LEVEL::Info);

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            lines.push(line.unwrap());
        }

        let n_atoms: usize = lines[0].parse().unwrap();
        let n_geoms: usize = lines.len() / (n_atoms + 2);

        let mut geoms = Vec::new();

        for i in 0..n_geoms {
            let mut atoms = Vec::new();
            let mut coordinates = Vec::new();

            for j in 0..n_atoms {
                let line = &lines[i * (n_atoms + 2) + j + 2];
                let items: Vec<f64> = line
                    .split_whitespace()
                    .filter(|x| !x.is_empty())
                    .map(|s| element_atomic_parser(s))
                    .collect();

                atoms.push(items[0] as usize);
                coordinates.push([items[1], items[2], items[3]]);
            }

            geoms.push((atoms, coordinates));
        }

        geoms
    }
}
