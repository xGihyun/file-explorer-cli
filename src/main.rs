use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let mut dir_path_input = String::new();

    println!("Enter path:\n");

    io::stdin()
        .read_line(&mut dir_path_input)
        .expect("Invalid input");

    let dir_path = dir_path_input.replace("\\", "/").trim().to_string();

    // println!("{}", dir_path);
    println!();

    let read_dir = fs::read_dir(dir_path);

    match read_dir {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    let file_path_str = file_path.to_string_lossy().replace("\\", "/");
                    let file_name = entry.file_name();

                    get_metadata(&file_path);

                    if let Some(name) = file_name.to_str() {
                        println!("{}", name);
                    }

                    println!("{}\n", file_path_str);
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    }
}

fn get_metadata(path: &Path) {
    if path.is_file() {
        let metadata = fs::metadata(path).unwrap();

        let size_in_kb = metadata.len() as f64 / (1024.0);

        if size_in_kb < 1024.0 {
            println!("Size: {:.2} KB", size_in_kb);
        } else if size_in_kb < 1024.0 * 1024.0 {
            let size_in_mb = size_in_kb / 1024.0;

            println!("Size: {:.2} MB", size_in_mb);
        } else {
            let size_in_gb = size_in_kb / (1024.0 * 1024.0);

            println!("Size: {:.2} GB", size_in_gb);
        }
    }
}
