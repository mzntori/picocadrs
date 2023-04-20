# picocadrs

A library to deserialize and serialize picoCAD files.
As of now there aren't many features that help with manipulating the file, however these features are planned.

## Example

This is a simple example of how to read a file, modify it and write the results to the same file.

```rust
use std::{fs, env};
use picocadrs::assets::{PicoColor, Vector};
use picocadrs::save::PicoSave;

// generate path
let mut path = env::var("picocad_path").expect("Invalid environment variable.");
path.push_str("plane.txt");

// read file and create save
let mut save = PicoSave::from(fs::read_to_string(path.clone()).expect("Couldn't read file."));

// set bg color
save.header.bg_color = PicoColor::DarkGreen;

// edit mesh
let mesh = save.meshes.get_mut(0).unwrap();
// rename the mesh to "first_mesh"
mesh.name = "first_mesh".to_string();
// set mesh origin to 0|3|0
mesh.pos = Vector::new(0.0, 3.0, 0.0);

// write save to file
fs::write(path, save.to_string()).expect("Couldn't write to file.");
```