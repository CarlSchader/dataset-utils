use std::{env, fs, fmt};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::Path;

const SUFFIX: &str = "-";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("flatten command will take all files from the input directories and copy them to the output directory. Any clashing names will be resolved by addind suffixes before the file extension.");
        println!("Usage: flatten <dir1> <dir2> ... <out-dir>");
        return;
    }

    let output_dir: &String = &args.last().unwrap();
    let mut path_stack: Vec<String> = args[1..args.len()-1].to_vec();
    let mut seen: HashMap<OsString, u32> = HashMap::new();


    // attempt to make out dir
    if !Path::new(output_dir).exists() {
        let res = fs::create_dir(output_dir);
        match res {
            Ok(_) => (),
            Err(e) => panic!("Error making output_dir: {}", e),
        };
    }
    

    while !path_stack.is_empty() {
        let curr: String = match path_stack.pop() {
           Some(s) => s,
           None => panic!("Error popping next path.")
        };

        let path = Path::new(&curr);
        if !path.exists() {
            panic!("Error path {} does not exit", curr);
        }

        if !path.is_dir() {
            let file_name = match path.file_name() {
                Some(s) => s,
                None => panic!("Error getting file name for path: {}", curr),
            };

            let count = seen.get(file_name);
            let mut new_file_name: String = file_name.to_str().unwrap().to_string();
            if count.is_none() {
                seen.insert(file_name.into(), 1);
            } else {
                // if extension does not exist save empty string
                let extension: String = match path.extension() {
                    Some(s) => ".".to_owned() + s.to_str().unwrap(),
                    None => String::from(""),
                };
                let stem = path.file_stem().unwrap().to_str().unwrap();
                new_file_name = stem.to_owned() + SUFFIX + count.unwrap().to_string().as_str() + &extension;
                seen.insert(file_name.into(), count.unwrap() + 1);
            }
            
            let _ = match fs::copy(&path, &Path::new(&fmt::format(format_args!("{}/{}", &output_dir, &new_file_name)))) {
                Ok(_) => (),
                Err(e) => panic!("Error copying file to {}: {}", new_file_name, e),
            };
        } else {
            let paths = fs::read_dir(path).unwrap(); 
            for sub_path in paths {
                path_stack.push(sub_path.unwrap().path().to_str().unwrap().to_string());
            }
        }
    }
}

