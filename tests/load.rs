use std::{fs::File, io::{BufReader, Read}};
use ufbx;

fn check_blender_default(scene: &ufbx::Scene) {
    {
        let node = scene.find_node("Cube").expect("expected to find 'Cube'");
        let mesh = node.mesh.as_ref().expect("expected 'Cube' to have a mesh");
        assert_eq!(mesh.num_faces, 6);

        let mesh_material = &mesh.materials[0];
        let node_material = &node.materials[0];
        assert_eq!(mesh_material.element.element_id, node_material.element.element_id);
    }

    {
        let node = scene.find_node("Light").expect("expected to find 'Light'");
        let light = node.light.as_ref().expect("expected 'Light' to have a light");
        assert_eq!(light.type_, ufbx::LightType::Point);
    }

    {
        let node = scene.find_node("Camera").expect("expected to find 'Camera'");
        let camera = node.camera.as_ref().expect("expected 'Camera' to have a camera");
        assert_eq!(camera.projection_mode, ufbx::ProjectionMode::Perspective);
    }
}

fn read_file(path: &str) -> Vec<u8> {
    let file = File::open(path).expect("could not find file");
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::<u8>::new();
    reader.read_to_end(&mut buffer).expect("failed to read");
    buffer
}

#[test]
fn blender_default() {
    let scene = ufbx::load_file("tests/data/blender_default.fbx", ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene);
}

#[test]
fn blender_default_memory() {
    let data = read_file("tests/data/blender_default.fbx");
    let scene = ufbx::load_memory(&data, ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene);
}

#[test]
fn blender_default_file() {
    let path = "tests/data/blender_default.fbx";
    let file = File::open(path).expect("could not find file");
    let scene = ufbx::load_stream(ufbx::Stream::Read(Box::new(file)), ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene);
}

#[test]
fn blender_default_reader() {
    let path = "tests/data/blender_default.fbx";
    let file = File::open(path).expect("could not find file");
    let reader = BufReader::new(file);
    let scene = ufbx::load_stream(ufbx::Stream::Read(Box::new(reader)), ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene);
}

#[test]
fn not_found() {
    let err = ufbx::load_file("tests/data/not_found.fbx", ufbx::LoadOpts::default())
        .err().expect("expected loading 'not_found.fbx' to fail");
    assert!(err.description.contains("File not found"));
    assert!(err.info().contains("not_found.fbx"));
    assert_eq!(err.type_, ufbx::ErrorType::FileNotFound);
}
