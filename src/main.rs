use std::io::prelude::*;
use std::fs;
use std::io;

use flate2::read::GzDecoder;
use chemcore::daylight;
use chemcore::molecule::{ Molecule, Error };
use gamma::graph::Graph;

fn main() -> std::io::Result<()> {
    let _ = fs::create_dir("./results");
    let read_root = "./smilesreading/2-aromaticsmiles/chembl";

    for entry in fs::read_dir(read_root)? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy()
            .to_string().replace(".smi.gz", ".txt");
        let in_file = fs::File::open(entry.path())?;
        let in_stream = GzDecoder::new(in_file);
        let reader = io::BufReader::new(in_stream);
        let out_path = String::from(format!("./results/{}", file_name));

        let out_file = fs::File::create(out_path)?;
        let mut writer = io::LineWriter::new(out_file);

        for result in reader.lines() {
            let line = result?;
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let id = parts.last().expect("no parts");

            if parts.len() == 1 {
                writeln!(&mut writer, "# {} No_input", id);
                
                continue;
            }

            let smiles = parts.first().expect("no smiles");

            match daylight::read(&smiles) {
                Ok(molecule) => {
                    let hcounts = molecule.nodes().iter().map(|id| {
                        molecule.hydrogens(*id).unwrap().to_string()
                    }).collect::<Vec<_>>();

                    writeln!(&mut writer, "{} {}", id, hcounts.join(" "));
                },
                Err(Error::CanNotKekulize) => {
                    writeln!(&mut writer, "# {} Kekulization_failure", id);
                },
                Err(Error::Hypervalent(_)) => {
                    writeln!(&mut writer, "# {} Bad_valence", id);
                },
                Err(error) => {
                    writeln!(&mut writer, "# {} ERROR: {:?}", id, error);
                }
            }
        }
    }

    Ok(())
}