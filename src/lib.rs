
use std::collections::HashMap;
use std::fs::read_dir;
use std::path::Path;
use round::round;
use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct FileInfo {
    pub path: String,
    pub extension: String,
    pub file_size_raw: u64,
    pub file_size_label: String
}

#[derive(Debug,Serialize)]
pub enum TreeType {
    File(FileInfo),
    Dir(HashMap<String,Vec<TreeType>>)
}
/// "Convert byte values into a human-readable format. Example: 1024 bytes to 1 KB."
/// ```rust
/// use tree_mapper::file_size_to_string;
/// let file_size: f64 = 1024.00;
/// let result: String = file_size_to_string(file_size,1);
/// println!("{:#?}",result);
/// ```
pub fn file_size_to_string(mut file_size: f64,file_size_round: i32) -> String {
    let units: Vec<&str> = Vec::from(
        ["b","kb","mb","gb","tb","pb"]
    );
    let mut selected_unit: &str = "b";

    for unit in units {
        if  file_size < 1024.00 {
            selected_unit = unit;
            break;
        }
        else {            
            file_size = file_size / 1024.00;
        }
    }
    format!("{}{}",round(file_size,file_size_round),selected_unit)
}

/// Generate a hierarchical mapping of files and directories, organized by their depth within the directory structure.
/// ```rust
/// use std::collections::HashMap;
/// use std::path::Path;
/// use tree_mapper::explore;
/// use tree_mapper::TreeType;
/// let base_path: String = String::from("storage/test-data");
/// let mut data: HashMap<String,Vec<TreeType>> = HashMap::new();
/// explore(Path::new(&base_path),&mut data,1);
/// println!("{:#?}",data);
/// ```
pub fn explore(base_path: &Path, data: &mut HashMap<String,Vec<TreeType>>,file_size_round: i32) {
    match read_dir(base_path){
        Ok(rd) => {
            let mut inner_files: Vec<TreeType> = Vec::new();
            for dir_entry in rd {
                if let Ok(ref entry) = dir_entry {
                    if entry.path().is_dir() {
                        let mut inner_dirs: HashMap<String,Vec<TreeType>> = HashMap::new();
                        explore(entry.path().as_path(), &mut inner_dirs,file_size_round);                        
                        inner_files.push(
                            TreeType::Dir( inner_dirs)
                        );
                    }
                    else if entry.path().is_file() {                        
                        let file_size_raw:u64 = entry.path().metadata().unwrap().len();
                        inner_files.push(
                            TreeType::File( 
                                FileInfo{ 
                                    path: entry.file_name().to_str().unwrap().to_string(),
                                    extension: entry.path().extension().unwrap().to_str().unwrap().to_string(),
                                    file_size_raw,
                                    file_size_label: file_size_to_string(file_size_raw as f64,file_size_round)
                                }
                            ) 
                        );
                    }
                }
            }
            if inner_files.len() > 0 {
                data.insert(base_path.file_name().unwrap().to_str().unwrap().to_string(), inner_files );
            }
        }
        Err(error) => {
            panic!("{}",error);
        }
    }
}

/// Converts the hashmap data to json string.
/// ```rust
/// use std::collections::HashMap;
/// use std::path::Path;
/// use tree_mapper::{explore, to_json};
/// use tree_mapper::TreeType;
/// let base_path: String = String::from("storage/test-data");
/// let mut data: HashMap<String,Vec<TreeType>> = HashMap::new();
/// explore(Path::new(&base_path),&mut data,1);
/// let mapping:String = to_json(data);
/// println!("{}",mapping);
/// ```
pub fn to_json(data: HashMap<String,Vec<TreeType>>) -> String {
    serde_json::to_string_pretty(&data).unwrap()
}

#[cfg(test)]
mod test_fd_mapping {
    use super::*;
    
    #[test]
    fn test_explore(){
        let base_path: String = String::from("storage/test-data");
        let mut data: HashMap<String,Vec<TreeType>> = HashMap::new();
        explore(Path::new(&base_path),&mut data,1);
        let mapping:String = to_json(data);
        println!("{}",mapping);
    }
    #[test]
    fn test_file_size_converter(){

        for t in vec![
            (1024,"1kb"),
            (1024,"1kb"),
            (1048576,"1mb"),
            (1073741824,"1gb"),
            (1,"1b")
        ] {
            let result: String = file_size_to_string(f64::from(t.0),1);
            println!("{:#?}",result);
            assert_eq!(result,t.1);
        }
    }
}