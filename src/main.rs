use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
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
    let query = read_user_input("Search:");

    let files: Vec<FileData> = match get_file_data(&dir_path) {
        Ok(data) => data.into_iter().collect(),
        Err(err) => {
            eprintln!("Error: {}", err);
            Vec::new()
        }
    };

    let matcher = SkimMatcherV2::default();

    let mut filtered_files: Vec<FileData> = files
        .into_iter()
        .filter(|file| matcher.fuzzy_match(&file.name, &query).is_some())
        .collect();

    let sorted_files = &mut filtered_files;

    sorted_files.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

    for file in &mut *sorted_files {
        println!("{}", file.name);
        println!("{}", file.path);
        println!("{:.2} KB\n", file.size);
    }

    println!("Total of {} files.", sorted_files.len());
}

fn read_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);

    if result.is_ok() {
        input.trim().replace('\\', "/").to_string()
    } else {
        println!("No query.");
        String::new()
    }
}

fn get_file_data(dir_path: &str) -> io::Result<Vec<FileData>> {
    let mut files = Vec::new();

    traverse_directory(Path::new(dir_path), &mut files)?;

    Ok(files)
}

fn traverse_directory(path: &Path, files: &mut Vec<FileData>) -> io::Result<()> {
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

fn get_file_size(path: &Path) -> io::Result<f64> {
    let metadata = fs::metadata(path)?;
    let size_in_bytes = metadata.len();

    Ok(size_in_bytes as f64 / 1024.0)
}

// fn get_directory_size(dir_path: &Path) -> io::Result<f64> {
//     let mut total_size: f64 = 0.0;
//     let entries = fs::read_dir(dir_path)?;

//     for entry in entries {
//         let entry = entry?;
//         let file_type = entry.file_type()?;
//         let path = entry.path();

//         if file_type.is_file() {
//             let size_in_kb: f64 = get_file_size(&path)?;
//             total_size += size_in_kb;
//         } else if file_type.is_dir() {
//             let size_in_kb: f64 = get_directory_size(&path)?;
//             total_size += size_in_kb;
//         }
//     }

//     Ok(total_size)
// }
