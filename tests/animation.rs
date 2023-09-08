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

#[test]
fn cube_anim_evaluate() {
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
        let state = scene.evaluate(anim, time, Default::default())
            .expect("failed to evaluate animation");
        let cube_state = &state.nodes[cube.element.typed_id as usize];
        let transform = cube_state.local_transform;

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

    for &(time, ref_color) in &material_refs {
        let state = scene.evaluate(anim, time, Default::default())
            .expect("failed to evaluate animation");
        let material_state = &state.materials[material.element.typed_id as usize];

        let diffuse_color = &material_state.fbx.diffuse_color;
        assert_close(ref_color.0, diffuse_color.value_vec4.x);
        assert_close(ref_color.1, diffuse_color.value_vec4.y);
        assert_close(ref_color.2, diffuse_color.value_vec4.z);
    }
}

#[test]
fn cube_anim_visibility_curve() {
    let scene = ufbx::load_file("tests/data/cube_anim.fbx", Default::default())
        .expect("expected to load scene");

    let cube = scene.find_node("pCube1").expect("expected to find a cube");

    let stack = scene.find_anim_stack("Take 001")
        .expect("expected to find anim stack 'Take 001'");
    assert_eq!(stack.layers.len(), 1);
    let layer = &stack.layers[0];

    let anim_props = layer.find_anim_props(&cube.element);
    assert_eq!(anim_props.len(), 4);

    let visibility = layer.find_anim_prop(&cube.element, "Visibility")
        .expect("expected to find anim prop Visibility");
    let curve = visibility.anim_value.curves[0].as_ref()
        .expect("expected Visibility curve 0 to be defined");
    assert!(visibility.anim_value.curves[1].is_none());
    assert!(visibility.anim_value.curves[2].is_none());

    assert_eq!(curve.keyframes.len(), 2);
    assert_eq!(curve.keyframes[0].time, 0.0);
    assert_eq!(curve.keyframes[1].time, 12.0/24.0);
}
