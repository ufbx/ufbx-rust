use std::panic;
use panic_message::panic_message;
use ufbx::{self, VertexStream, AllocatorOpts};

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

#[test]
fn generate_indices() {
    let scene = ufbx::load_file("tests/data/blender_default.fbx", ufbx::LoadOpts::default())
        .expect("expected to load scene");

    let node = scene.find_node("Cube").expect("expected to find 'Cube'");
    let mesh = node.mesh.as_ref().expect("expected 'Cube' to have a mesh");

    let mut vertices: Vec<(ufbx::Vec3, ufbx::Vec3)> = Vec::new();

    let mut tri_indices = Vec::new();
    for &face in &mesh.faces {
        ufbx::triangulate_face_vec(&mut tri_indices, &mesh, face);
        for &ix in &tri_indices {
            let position = mesh.vertex_position[ix as usize];
            let normal = mesh.vertex_normal[ix as usize];
            vertices.push((position, normal));
        }
    }

    let mut indices = vec![0u32; vertices.len()];
    let mut streams = [
        VertexStream::new(&mut vertices),
    ];
    let result = ufbx::generate_indices(&mut streams, &mut indices, AllocatorOpts::default())
        .expect("failed to generate indices");

    assert_eq!(result, 6*4);
    vertices.shrink_to(result);
}

#[test]
fn generate_indices_multi_stream() {
    let scene = ufbx::load_file("tests/data/blender_default.fbx", ufbx::LoadOpts::default())
        .expect("expected to load scene");

    let node = scene.find_node("Cube").expect("expected to find 'Cube'");
    let mesh = node.mesh.as_ref().expect("expected 'Cube' to have a mesh");

    let mut positions: Vec<ufbx::Vec3> = Vec::new();
    let mut normals: Vec<ufbx::Vec3> = Vec::new();

    let mut tri_indices = Vec::new();
    for &face in &mesh.faces {
        ufbx::triangulate_face_vec(&mut tri_indices, &mesh, face);
        for &ix in &tri_indices {
            let position = mesh.vertex_position[ix as usize];
            let normal = mesh.vertex_normal[ix as usize];
            positions.push(position);
            normals.push(normal);
        }
    }

    let mut indices = vec![0u32; positions.len()];
    let mut streams = [
        VertexStream::new(&mut positions),
        VertexStream::new(&mut normals),
    ];
    let result = ufbx::generate_indices(&mut streams, &mut indices, AllocatorOpts::default())
        .expect("failed to generate indices");

    assert_eq!(result, 6*4);
    positions.shrink_to(result);
    normals.shrink_to(result);
}
