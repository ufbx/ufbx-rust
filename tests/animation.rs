use ufbx;

fn assert_close(a: f64, b: f64) {
    let delta = a - b;
    assert!(delta.abs() <= 0.001, "expected approximately {}={}", a, b);
}

fn assert_close_vec4(a: ufbx::Vec4, b: ufbx::Vec4) {
    assert_close(a.x, b.x);
    assert_close(a.y, b.y);
    assert_close(a.z, b.z);
    assert_close(a.w, b.w);
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

#[test]
fn anim_override() {
    let scene = ufbx::load_file("tests/data/cube_anim.fbx", Default::default())
        .expect("expected to load scene");

    let cube = scene
        .find_node("pCube1")
        .expect("expected to find pCube1");
    let material = scene.
        find_element(ufbx::ElementType::Material, "lambert1")
        .and_then(ufbx::as_material)
        .expect("expected to find material lambert1");

    let uv_prop = cube.element.props.find_prop("currentUVSet")
        .expect("expected to find currentUVSet in pCube1");
    assert_eq!(uv_prop.value_str, "map1");

    let new_prop = cube.element.props.find_prop("NewProperty");
    assert!(new_prop.is_none());

    let diffuse_prop = material.element.props.find_prop("DiffuseColor")
        .expect("expected to find DiffuseColor in lambert1");
    assert_close_vec4(diffuse_prop.value_vec4, ufbx::Vec4{
        x: 0.740740716457367,
        y: 0.259259253740311,
        z: 0.0,
        w: 0.0,
    });

    let overrides = [
        ufbx::PropOverrideDesc {
            element_id: cube.element.element_id,
            prop_name: "currentUVSet".into(),
            value_str: format!("map{}", 99).into(),
            ..Default::default()
        },
        ufbx::PropOverrideDesc {
            element_id: cube.element.element_id,
            prop_name: "NewProperty".into(),
            value_int: 0x4000_0000_0000_0001_i64,
            ..Default::default()
        },
        ufbx::PropOverrideDesc {
            element_id: material.element.element_id,
            prop_name: "DiffuseColor".into(),
            value: ufbx::Vec4{ x: 1.0, y: 0.0, z: 1.0, w: 1.0 },
            ..Default::default()
        },
    ];

    let anim_opts = ufbx::AnimOpts {
        prop_overrides: overrides.as_slice().into(),
        ..Default::default()
    };
    let anim = ufbx::create_anim(&scene, anim_opts)
        .expect("expected to create anim");

    let state = scene.evaluate(&anim, 0.0, Default::default())
        .expect("expected to evaluate scene");

    let cube_state = state.elements
        .get(cube.element.element_id as usize)
        .and_then(|e| ufbx::as_node(e))
        .expect("expected to find pCube1 in evaluated scene");
    let material_state = state.elements
        .get(material.element.element_id as usize)
        .and_then(|e| ufbx::as_material(e))
        .expect("expected to find lambert1 in evaluated scene");

    let uv_prop = cube_state.element.props.find_prop("currentUVSet")
        .expect("expected to find currentUVSet in evaluated pCube1");
    assert_eq!(uv_prop.value_str, "map99");

    let new_prop = cube_state.element.props.find_prop("NewProperty")
        .expect("expected to find NewProperty in evaluated pCube1");
    assert_eq!(new_prop.value_int, 0x4000_0000_0000_0001_i64);
    assert_close(new_prop.value_vec4.x, 0x4000_0000_0000_0001_i64 as f64);

    let diffuse_prop = material_state.element.props.find_prop("DiffuseColor")
        .expect("expected to find DiffuseColor in evaluated lambert1");
    assert_close_vec4(diffuse_prop.value_vec4, ufbx::Vec4{
        x: 1.0,
        y: 0.0,
        z: 1.0,
        w: 1.0,
    });
}
