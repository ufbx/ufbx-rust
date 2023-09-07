#![allow(dead_code)]

use ufbx;

pub fn distance(a: ufbx::Vec3, b: ufbx::Vec3) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx*dx + dy*dy + dz*dz
}

pub fn remove_close(points: &mut Vec<ufbx::Vec3>, point: ufbx::Vec3) -> bool {
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

pub fn check_mesh_positions(node: &ufbx::Node, mesh: &ufbx::Mesh, ref_mesh: &ufbx::Mesh) {
    let mut ref_points = Vec::<ufbx::Vec3>::new();
    for ix in 0..ref_mesh.num_indices {
        let pos = ref_mesh.vertex_position[ix];
        ref_points.push(pos);
    }

    for ix in 0..mesh.num_indices {
        let local_pos = mesh.vertex_position[ix];
        let pos = ufbx::transform_position(&node.geometry_to_world, local_pos);
        let found = remove_close(&mut ref_points, pos);
        assert!(found, "{} not found", pos);
    }
    assert!(ref_points.len() == 0);
}
