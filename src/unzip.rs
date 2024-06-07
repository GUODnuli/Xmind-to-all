use std::path::PathBuf;
use std::fs::File;
use std::io;
use zip::ZipArchive;

pub fn extract_zip(file_path: &PathBuf) -> zip::result::ZipResult<PathBuf> {
    let file = File::open(file_path).map_err(|e| {
        eprintln!("Failed to open file {:?}: {}", file_path, e);
        e
    })?;

    let mut archive = ZipArchive::new(file).map_err(|e| {
        eprintln!("Failed to read ZIP archive from file {:?}: {}", file_path, e);
        e
    })?;

    let mut output_dir = PathBuf::new();
    if let Some(parent) = file_path.parent() {
        output_dir.push(parent);
    }
    if let Some(stem) = file_path.file_stem() {
        output_dir.push(stem)
    }

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| {
            eprintln!("Failed to access file at index {}: {}", i, e);
            e
        })?;
        let outpath = output_dir.join(file.mangled_name());

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath).map_err(|e| {
                eprintln!("Failed to create directory {:?}: {}", outpath, e);
                e
            })?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).map_err(|e| {
                        eprintln!("Failed to create parent directory {:?}: {}", p, e);
                        e
                    })?;
                }
            }
            let mut outfile = File::create(&outpath).map_err(|e| {
                eprintln!("Failed to create file {:?}: {}", outpath, e);
                e
            })?;
            io::copy(&mut file, &mut outfile).map_err(|e| {
                eprintln!("Failed to copy to file {:?}: {}", outpath, e);
                e
            })?;
        }
    }
    Ok(output_dir)
}