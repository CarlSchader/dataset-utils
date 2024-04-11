use std::collections::HashMap;
use std::env::args;
use std::fs;

/// Pytorch DatasetFolder type expects a directory structure where each class has its own sub-directory and each sample is inside it's class directory.
/// This cli tool will merge multiple DatasetFolders into a single one. 

const SUFFIX: &str = "-";

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_dir>... <output_dir>", args[0]);
        std::process::exit(1);
    }

    let input_dirs = &args[1..args.len() - 1];
    let output_dir = &args[args.len() - 1];

    let mut seen: HashMap<String, u32> = HashMap::new();

    for input_dir in input_dirs {
        let entries = fs::read_dir(input_dir).unwrap();
        for class_dir in entries {
            let class_dir = class_dir.unwrap();
            let class_name = class_dir.file_name();
            let class_name = class_name.to_str().unwrap();
            let class_output_dir = format!("{}/{}", output_dir, class_name);
            fs::create_dir_all(&class_output_dir).unwrap();

            let samples = fs::read_dir(class_dir.path()).unwrap();
            for sample in samples {
                let sample_path = sample.unwrap().path();
                let sample_name = sample_path.file_stem().unwrap().to_str().unwrap().to_string();
                let extension = sample_path.extension();
                let sample_output_path = match extension {
                    Some(ext) => format!("{}/{}.{}", class_output_dir, sample_name, ext.to_str().unwrap()),
                    None => format!("{}/{}", class_output_dir, sample_name),
                };

                if seen.contains_key(&sample_output_path) {
                    let count = seen.get(&sample_output_path).unwrap();
                    let new_sample_output_path = match extension {
                        Some(ext) => format!("{}/{}{}{}.{}", class_output_dir, sample_name, SUFFIX, count, ext.to_str().unwrap()),
                        None => format!("{}/{}{}{}", class_output_dir, sample_name, SUFFIX, count),
                    };
                    match fs::copy(&sample_path, &new_sample_output_path) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Error copying {} to {}: Message {}", sample_path.display(), new_sample_output_path, e),
                    };
                    seen.insert(sample_output_path, count + 1);
                } else {
                    match fs::copy(&sample_path, &sample_output_path) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Error copying {} to {}: Message {}", sample_path.display(), sample_output_path, e),
                    }
                    seen.insert(sample_output_path, 1);
                }
            }
        }
    }
}

