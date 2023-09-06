use ufbx::{self, AllocatorOpts};

fn distance(a: ufbx::Vec3, b: ufbx::Vec3) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx*dx + dy*dy + dz*dz
}

fn remove_close(points: &mut Vec<ufbx::Vec3>, point: ufbx::Vec3) -> bool {
    for i in 0..points.len() {
        let ref_pos = points[i];
        let d = distance(point, ref_pos);
        if d < 0.001 {
            points.swap_remove(i);
            return true;
        }
    }
    false
}

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

    let mut ref_points = Vec::<ufbx::Vec3>::new();
    for ix in 0..obj_mesh.num_indices {
        let pos = obj_mesh.vertex_position[ix];
        ref_points.push(pos);
    }

    let sub_mesh = mesh.subdivide(1, ufbx::SubdivideOpts::default())
        .expect("expected to subdivide mesh");
    for ix in 0..sub_mesh.num_indices {
        let local_pos = sub_mesh.vertex_position[ix];
        let pos = ufbx::transform_position(&node.geometry_to_world, local_pos);
        let found = remove_close(&mut ref_points, pos);
        assert!(found, "{} not found", pos);
    }
    assert!(ref_points.len() == 0);
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