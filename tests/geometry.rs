use std::panic;
use panic_message::panic_message;
use ufbx::{self, VertexStream, AllocatorOpts};
mod common;

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

#[test]
fn generate_indices_truncated() {
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

    normals.pop();

    let mut indices = vec![0u32; positions.len()];
    let mut streams = [
        VertexStream::new(&mut positions),
        VertexStream::new(&mut normals),
    ];
    let err = ufbx::generate_indices(&mut streams, &mut indices, AllocatorOpts::default())
        .err().expect("expected generate_indices() to fail");

    assert_eq!(err.type_, ufbx::ErrorType::TruncatedVertexStream);
    assert_eq!(err.info(), "1");
}

#[test]
fn tessellate_saddle() {
    let scene = ufbx::load_file("tests/data/nurbs_saddle.fbx", ufbx::LoadOpts::default())
        .expect("expected to load scene");

    let node = scene.find_node("nurbsPlane1").expect("expected to find nurbsPlane1");
    let nurbs_plane = node.attrib.as_ref().and_then(|e| ufbx::as_nurbs_surface(e))
        .expect("expected nurbsPlane1 to have NurbsSurface");

    let tessellate_opts = ufbx::TessellateSurfaceOpts {
        span_subdivision_u: 4,
        span_subdivision_v: 4,
        ..Default::default()
    };
    let mesh = nurbs_plane.tessellate(tessellate_opts)
        .expect("expected to tessellate");

    let obj_scene = ufbx::load_file("tests/data/nurbs_saddle.obj", ufbx::LoadOpts::default())
        .expect("expected to load obj scene reference");
    let obj_node = obj_scene.find_node("nurbsToPoly1").expect("expected to find 'nurbsToPoly1'");
    let obj_mesh = obj_node.mesh.as_ref().expect("expected OBJ node to have mesh");

    assert_eq!(mesh.num_faces, obj_mesh.num_faces);
    common::check_mesh_positions(node, &mesh, &obj_mesh);
}
