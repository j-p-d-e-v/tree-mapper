# TreeMapper

## Overview

This utility generates a hierarchical mapping of files and directories, represented as a `HashMap`. It reflects the structure of directories and files based on their levels, providing a clear view of the organization of the file system.


## Installation

1. Clone the repository:

   ```bash
   git clone j-p-d-e-v/tree-mapper
   ```

2. Navigate to the project directory:

   ```bash
   cd file-tree-mapping-generator
   ```

3. Build and run the application:

   ```bash
   cargo build
   cargo run
   ```

## Usage

### Explore a directory then convert it to json string.

```rust
use std::collections::HashMap;
use std::path::Path;
use tree_mapper::explore;
use tree_mapper::TreeType;
let base_path: String = String::from("storage/test-data");
let mut data: HashMap<String,Vec<TreeType>> = HashMap::new();
explore(Path::new(&base_path),&mut data,1);
let mapping: String = to_json(data);
println!("{}",mapping);
```

## Contributing

Feel free to open issues or submit pull requests for enhancements and bug fixes.

## License

This project is licensed under the MIT License.
