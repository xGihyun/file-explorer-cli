use std::fs;
use std::io;
use std::path::Path;

struct FileData {
    name: String,
    path: String,
    size: f64,
}

fn main() {
    let dir_path = read_user_input("Enter path:");

    let mut files: Vec<FileData> = match get_file_data(&dir_path) {
        Ok(data) => data.into_iter().collect(),
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new() // or handle the error case as needed
        }
    };
    
    files.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

    for file in &files {
        println!("{}", file.name);
        println!("{}", file.path);
        println!("{:.2} KB\n", file.size);
    }

    println!("Total of {} files.\n", files.len());
}

fn read_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");

    input.trim().replace('\\', "/")
}

fn get_file_data(dir_path: &str) -> io::Result<Vec<FileData>> {
    let entries = fs::read_dir(dir_path)?;

    let mut file_data = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let path = entry.path();

        let data = if file_type.is_file() {
            let size_in_kb: f64 = get_file_size(&path).unwrap();
            let name = entry.file_name().to_string_lossy().to_string();

            FileData {
                name,
                path: path.to_string_lossy().replace('\\', "/"),
                size: size_in_kb,
            }
        } else if file_type.is_dir() {
            let size_in_kb: f64 = get_directory_size(&path)?;
            let name = entry.file_name().to_string_lossy().to_string();

            FileData {
                name,
                path: path.to_string_lossy().replace('\\', "/"),
                size: size_in_kb,
            }
        } else {
            continue; // Skip non-file and non-directory entries
        };

        file_data.push(data);
    }

    Ok(file_data)
}

fn get_file_size(path: &Path) -> io::Result<f64> {
    let metadata = fs::metadata(path)?;
    let size_in_bytes = metadata.len();

    Ok(size_in_bytes as f64 / 1024.0)
}

fn get_directory_size(path: &Path) -> io::Result<f64> {
    let mut total_size: f64 = 0.0;
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let entry_path = entry.path();

        if file_type.is_file() {
            let size_in_kb: f64 = get_file_size(&entry_path)?;
            total_size += size_in_kb;
        } else if file_type.is_dir() {
            let size_in_kb: f64 = get_directory_size(&entry_path)?;
            total_size += size_in_kb;
        }
    }

    Ok(total_size)
}