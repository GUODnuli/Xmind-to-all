use std::path::PathBuf;
use std::fs::File;
use std::io;
use zip::ZipArchive;

pub fn extract_zip(file_path: &PathBuf) -> zip::result::ZipResult<PathBuf> {
    let file = File::open(file_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut output_dir = PathBuf::new();
    if let Some(parent) = file_path.parent() {
        output_dir.push(parent);
    }
    if let Some(stem) = file_path.file_stem() {
        output_dir.push(stem)
    }
    println!("{:?}", output_dir);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = output_dir.join(file.mangled_name());

        if (&*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(output_dir)
}