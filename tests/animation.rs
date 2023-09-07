use ufbx;

fn assert_close(a: f64, b: f64) {
    let delta = a - b;
    assert!(delta.abs() <= 0.001, "expected approximately {}={}", a, b);
}

#[test]
fn cube_anim() {
    let scene = ufbx::load_file("tests/data/cube_anim.fbx", Default::default())
        .expect("expected to load scene");

    let cube = scene.find_node("pCube1").expect("expected to find a cube");

    let refs = [
        (0.0/24.0, (0.0, 0.0, 0.0), (0.0, 0.0, 0.0), (1.0, 1.0, 1.0)),
        (4.0/24.0, (0.0, 0.519, 0.0), (11.667, 11.667, 0.0), (1.052, 1.104, 1.156)),
        (8.0/24.0, (0.0, 1.481, 0.0), (33.333, 33.333, 0.0), (1.148, 1.296, 1.444)),
        (12.0/24.0, (0.0, 2.0, 0.0), (45.0, 45.0, 0.0), (1.2, 1.4, 1.6)),
    ];

    let anim = scene.anim.as_ref();
    for &(time, pos, rot, scale) in &refs {
        let transform = cube.evaluate_transform(anim, time);

        assert_close(pos.0, transform.translation.x);
        assert_close(pos.1, transform.translation.y);
        assert_close(pos.2, transform.translation.z);

        let euler = ufbx::quat_to_euler(transform.rotation, ufbx::RotationOrder::Xyz);
        assert_close(rot.0, euler.x);
        assert_close(rot.1, euler.y);
        assert_close(rot.2, euler.z);

        assert_close(scale.0, transform.scale.x);
        assert_close(scale.1, transform.scale.y);
        assert_close(scale.2, transform.scale.z);
    }

    let material_refs = [
        (0.0/24.0, (1.0, 0.0, 0.0)),
        (4.0/24.0, (0.741, 0.259, 0.0)),
        (8.0/24.0, (0.259, 0.741, 0.0)),
        (12.0/24.0, (0.0, 1.0, 0.0)),
    ];

    let mesh = cube.mesh.as_ref().expect("expected cube to have mesh");
    let material = mesh.materials.get(0).expect("expected mesh to have material");
    assert!(material.element.name == "lambert1");

    for &(time, diffuse_color) in &material_refs {
        let prop = ufbx::evaluate_prop(anim, &material.element, "DiffuseColor", time);
        assert_close(diffuse_color.0, prop.value_vec4.x);
        assert_close(diffuse_color.1, prop.value_vec4.y);
        assert_close(diffuse_color.2, prop.value_vec4.z);
    }
}
