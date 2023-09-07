use std::{fs::File, io::{BufReader, Read}};
use std::ffi::{CString, c_void};
use ufbx;
use libc;

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
fn blender_default_reader_prefix() {
    let path = "tests/data/blender_default.fbx";
    let file = File::open(path).expect("could not find file");
    let mut reader = BufReader::new(file);

    let mut prefix = [0u8; 64];
    let num_read = reader.read(&mut prefix).expect("failed to read prefix");
    assert_eq!(num_read, prefix.len());

    let stream = ufbx::Stream::Read(Box::new(reader));
    let scene = ufbx::load_stream_prefix(stream, &prefix, ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene);
}

#[test]
fn blender_default_stdio() {
    let path = "tests/data/blender_default.fbx";
    let result = unsafe {
        let c_path = CString::new(path).unwrap();
        let c_mode = CString::new("rb").unwrap();
        let file = libc::fopen(c_path.as_ptr(), c_mode.as_ptr());
        let result = ufbx::load_stdio(file as *mut c_void, ufbx::LoadOpts::default());
        libc::fclose(file);
        result
    };
    let scene = result.expect("expected to load scene");
    check_blender_default(&scene);
}

#[test]
fn blender_default_stdio_prefix() {
    let path = "tests/data/blender_default.fbx";
    let result = unsafe {
        let c_path = CString::new(path).unwrap();
        let c_mode = CString::new("rb").unwrap();
        let file = libc::fopen(c_path.as_ptr(), c_mode.as_ptr());
        
        let mut prefix = [0u8; 32];
        let num_read = libc::fread(prefix.as_mut_ptr() as *mut c_void, 1, prefix.len(), file);
        assert_eq!(num_read, prefix.len());
        
        let result = ufbx::load_stdio_prefix(file as *mut c_void, &prefix, ufbx::LoadOpts::default());
        libc::fclose(file);
        result
    };
    let scene = result.expect("expected to load scene");
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
