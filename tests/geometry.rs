use std::panic;
use panic_message::{panic_message};
use ufbx;

#[test]
fn triangulate() {
    let scene = ufbx::load_file("tests/data/blender_default.fbx", ufbx::LoadOpts::default())
        .expect("expected to load scene");

    let node = scene.find_node("Cube").expect("expected to find 'Cube'");
    let mesh = node.mesh.as_ref().expect("expected 'Cube' to have a mesh");

    let mut indices = vec![0u32; mesh.max_face_triangles * 3];
    for &face in &mesh.faces {
        let num_tris = mesh.triangulate_face(&mut indices, face);
        assert_eq!(num_tris, 2);
    }
}

#[test]
fn triangulate_fail() {
    let scene = ufbx::load_file("tests/data/blender_default.fbx", ufbx::LoadOpts::default())
        .expect("expected to load scene");

    let node = scene.find_node("Cube").expect("expected to find 'Cube'");
    let mesh = node.mesh.as_ref().expect("expected 'Cube' to have a mesh");

    let err = panic::catch_unwind(|| {
        let mut indices = [0u32, 0u32, 0u32];
        let face = mesh.faces[0];
        mesh.triangulate_face(&mut indices, face);
    }).err().expect("expected triangulation to fail");
    let msg = panic_message(&err);
    assert!(msg.contains("ufbx::triangulate_face() Face needs at least 6 indices for triangles, got space for 3"));
}
