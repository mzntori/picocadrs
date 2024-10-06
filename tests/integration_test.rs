use picocadrs::assets::{Color, Model, Point3D}; // Point3D required for point macro
use picocadrs::point;
use std::ffi::OsString;

#[test]
#[ignore]
fn readme() {
    // Loads the file "test.txt" located in the picoCAD project folder as a model.
    // This model now can access any part of that project.
    // For this example test.txt is a new picoCAD project that has a single plane added without
    // modifying it saved under the name "test".
    let model = Model::load(OsString::from("test")).unwrap();

    println!("Model name: {}", model.header.name); // "Model name: test"
    println!("Amount of meshes: {}", model.meshes.len()); // "Amount of meshes: 1"

    let mesh = model.meshes.get(0).unwrap();
    println!("Mesh name: {}", mesh.name); // "Mesh name: plane"
    println!("Mesh position: {}", mesh.position); // "Mesh position: 0,0,0"

    let face = mesh.faces.get(0).unwrap();
    println!("Face color: {}", face.color.as_i32()); // "Face color: 6"
    println!("Double sided: {}", face.double_sided); // "Double sided: true"
    println!("No texture: {}", face.no_texture); // "No texture: false"

    print!("\n");

    // Of course, you can change these values too.
    let mut model = Model::load(OsString::from("test")).unwrap();

    model.header.name = "model_name".to_string();
    println!("Model name: {}", model.header.name); // "Model name: model_name"
    println!("Amount of meshes: {}", model.meshes.len()); // "Amount of meshes: 1"

    let mesh = model.meshes.get_mut(0).unwrap();
    mesh.name = "some_plane".to_string();
    mesh.position = point!(1.5, -1.0, 2.0);
    println!("Mesh name: {}", mesh.name); // "Mesh name: some_plane"
    println!("Mesh position: {}", mesh.position); // "Mesh position: 1.5,-1,2"

    let face = mesh.faces.get_mut(0).unwrap();
    face.color = Color::Lavender;
    face.double_sided = false;
    face.no_texture = true;
    println!("Face color: {}", face.color.as_i32()); // "Face color: 13"
    println!("Double sided: {}", face.double_sided); // "Double sided: false"
    println!("No texture: {}", face.no_texture); // "No texture: true"

    reset_readme_file(model);
}

fn reset_readme_file(mut model: Model) {
    model.header.name = "test".to_string();

    let mesh = model.meshes.get_mut(0).unwrap();
    mesh.name = "plane".to_string();
    mesh.position = point!(0.0, 0.0, 0.0);

    let face = mesh.faces.get_mut(0).unwrap();
    face.color = Color::from(6);
    face.double_sided = true;
    face.no_texture = false;
}
