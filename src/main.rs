use std::io::prelude::*;
use std::fs;
use std::io;

use flate2::read::GzDecoder;
use chemcore::daylight;
use chemcore::molecule::{ Molecule, Error };
use gamma::graph::Graph;

fn main() -> io::Result<()> {
    let write_root = "./results/";
    let _ = fs::create_dir(write_root);
    let read_root = "./smilesreading/2-aromaticsmiles/chembl";

    for result in fs::read_dir(read_root)? {
        let entry = result?;
        let writer = create_writer(&entry, write_root)?;
        let reader = create_reader(&entry)?;

        process_file(reader, writer)?;
    }

    Ok(())
}

fn process_file(
    reader: io::BufReader<GzDecoder<std::fs::File>>,
    mut writer: io::LineWriter<std::fs::File>
) -> io::Result<()> {
    for result in reader.lines() {
        let line = result?;
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let id = parts.last().expect("no parts");

        if parts.len() == 1 {
            writeln!(&mut writer, "# {} No_input", id)?;
            
            continue;
        }

        write_line(id, parts.first().expect("no smiles"), &mut writer)?;
    }

    Ok(())
}

fn write_line(
    id: &str,
    smiles: &str,
    writer: &mut io::LineWriter<std::fs::File>
) ->io::Result<()> {
    match daylight::read(smiles) {
        Ok(molecule) => {
            let hcounts = molecule.nodes().iter().map(|id| {
                molecule.hydrogens(*id).unwrap().to_string()
            }).collect::<Vec<_>>();

            writeln!(writer, "{} {}", id, hcounts.join(" "))?;
        },
        Err(Error::CanNotKekulize) => {
            writeln!(writer, "# {} Kekulization_failure", id)?;
        },
        Err(Error::Hypervalent(_)) => {
            writeln!(writer, "# {} Bad_valence", id)?;
        },
        Err(error) => {
            writeln!(writer, "# {} ERROR: {:?}", id, error)?;
        }
    }

    Ok(())
}

fn create_reader(
    entry: &fs::DirEntry
) -> io::Result<io::BufReader<GzDecoder<std::fs::File>>> {
    let file = fs::File::open(entry.path())?;
    let decoder = GzDecoder::new(file);
    
    Ok(io::BufReader::new(decoder))
}

fn create_writer(
    entry: &fs::DirEntry, write_root: &str
) -> io::Result<io::LineWriter<std::fs::File>> {
    let file_name = entry.file_name()
        .to_string_lossy().replace(".smi.gz", ".txt");
    let path = String::from(format!("{}{}", write_root, file_name));
    let file = fs::File::create(path)?;

    Ok(io::LineWriter::new(file))
}