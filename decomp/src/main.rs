// unzipping files using rust

use std::env::args;
use std::fs::{self, create_dir_all, File};
use std::io;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // arguments
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <zipfile>", args[0]);
        return Ok(());
    }

    let fname = Path::new(&args[1]);
    let file = File::open(fname)?;

    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue, // skip unsafe paths
        };

        if file.is_dir() {
            create_dir_all(&outpath)?;
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i+1,
                outpath.display(),
                file.size()
            );

            // create parent directories if they don't exist
            if let Some(parent) = outpath.parent() {
                create_dir_all(parent)?;
            }

            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }

        // preserve unix permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
