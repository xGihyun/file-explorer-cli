use std::fs;
use std::io;
use std::path::Path;

pub struct FileData {
    pub name: String,
    pub path: String,
    pub size: f64,
}

pub fn get_file_data(dir_path: &str) -> io::Result<Vec<FileData>> {
    let mut files = Vec::new();

    traverse_directory(Path::new(dir_path), &mut files)?;

    Ok(files)
}

pub fn traverse_directory(path: &Path, files: &mut Vec<FileData>) -> io::Result<()> {
    if path.is_file() {
        let size_in_kb = get_file_size(path)?;
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let file_path = path.to_string_lossy().to_string();

        let file_data = FileData {
            name,
            path: file_path,
            size: size_in_kb,
        };

        files.push(file_data);
    } else if path.is_dir() {
        let entries = fs::read_dir(path)?;

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            traverse_directory(&entry_path, files)?;
        }
    }

    Ok(())
}

pub fn get_file_size(path: &Path) -> io::Result<f64> {
    let metadata = fs::metadata(path)?;
    let size_in_bytes = metadata.len();

    Ok(size_in_bytes as f64 / 1024.0)
}
