
fn nearly_eq(a: ufbx::Vec4, b: ufbx::Vec4) -> bool
{
    let eps = 1e-6;
    return (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps && (a.z - b.z).abs() <= eps && (a.w - b.w).abs() <= eps
}

#[test]
fn blender_default_material() {
    let scene = ufbx::load_file("tests/data/blender_default.fbx", Default::default())
        .expect("expected to load scene");

    assert_eq!(scene.materials.len(), 1);

    let material = &scene.materials[0];
    assert_eq!(material.element.name, "Material");

    assert!(nearly_eq(material.fbx.diffuse_color.value_vec4, ufbx::Vec4{ x: 0.8, y: 0.8, z: 0.8, w: 1.0 }));
    assert_eq!(material.fbx.diffuse_color.value_components, 3);

    assert!(nearly_eq(material.fbx.ambient_color.value_vec4, ufbx::Vec4{ x: 0.0508761, y: 0.0508761, z: 0.0508761, w: 1.0 }));
    assert_eq!(material.fbx.ambient_color.value_components, 3);

    assert_eq!(material.fbx.ambient_factor.value_vec4.x, 0.0);
    assert_eq!(material.fbx.ambient_factor.value_components, 1);

    assert_eq!(material.fbx.specular_exponent.value_vec4.x, 25.0);
    assert_eq!(material.fbx.specular_exponent.value_components, 1);
}
