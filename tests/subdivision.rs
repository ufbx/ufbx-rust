use common::check_mesh_positions;
use ufbx::{self, AllocatorOpts};
mod common;

#[test]
fn cube_subdivision() {
    let opts = ufbx::LoadOpts {
        target_axes: ufbx::axes_right_handed_y_up(),
        target_unit_meters: 1.0,
        ..Default::default()
    };
    let scene = ufbx::load_file("tests/data/blender_default.fbx", opts)
        .expect("expected to load scene");

    let obj_scene = ufbx::load_file("tests/data/cube_subdivided.obj", ufbx::LoadOpts::default())
        .expect("expected to load obj scene reference");

    let node = scene.find_node("Cube").expect("expected to find 'Cube'");
    let mesh = node.mesh.as_ref().expect("expected FBX node to have mesh");

    let obj_node = obj_scene.find_node("Cube").expect("expected to find 'Cube'");
    let obj_mesh = obj_node.mesh.as_ref().expect("expected OBJ node to have mesh");

    let sub_mesh = mesh.subdivide(1, ufbx::SubdivideOpts::default())
        .expect("expected to subdivide mesh");

    check_mesh_positions(node, &sub_mesh, &obj_mesh);
}


#[test]
fn subdivision_fail() {
    let scene = ufbx::load_file("tests/data/blender_default.fbx", Default::default())
        .expect("expected to load scene");

    let node = scene.find_node("Cube").expect("expected to find 'Cube'");
    let mesh = node.mesh.as_ref().expect("expected 'Cube' to have mesh");

    let sub_opts = ufbx::SubdivideOpts {
        result_allocator: AllocatorOpts {
            memory_limit: 1,
            ..Default::default()
        },
        ..Default::default()
    };
    let err = mesh.subdivide(1, sub_opts)
        .err().expect("expected subdivision to fail");

    assert_eq!(err.type_, ufbx::ErrorType::MemoryLimit);
    assert!(err.info() == "result");
}