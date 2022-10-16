use std::collections::HashMap;
use walkdir::WalkDir;

fn main() {
    let mut filenames = HashMap::new();

    for entry in WalkDir::new("/users/Claudiu/Downloads")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        let f_name = String::from(entry.file_name().to_string_lossy());
        let f_path = String::from(entry.path().to_string_lossy());
        let counter = filenames.entry(f_name.clone()).or_insert(0);
        *counter += 1;

        if *counter >= 2 {
            println!("{} - {}", f_name, f_path);
        }
    }

    println!("{:?}", filenames);
    println!("{:?}", filenames.len())
}
