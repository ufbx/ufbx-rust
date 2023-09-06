use ufbx::{self, LoadOpts};

#[test]
fn blender_default() {
    let scene = ufbx::load_file("tests/data/blender_default.fbx", LoadOpts::default())
        .expect("expected to load scene");

    {
        let node = scene.find_node("Cube").expect("expected to find 'Cube'");
        let mesh = node.mesh.as_ref().expect("expected 'Cube' to have a mesh");
        assert_eq!(mesh.num_faces, 6);

        let mesh_material = &mesh.materials[0];
        let node_material = &node.materials[0];
        assert_eq!(mesh_material.element.element_id, node_material.element.element_id);
    }

    {
        let node = scene.find_node("Light").expect("expected to find 'Light'");
        let light = node.light.as_ref().expect("expected 'Light' to have a light");
        assert_eq!(light.type_, ufbx::LightType::Point);
    }

    {
        let node = scene.find_node("Camera").expect("expected to find 'Camera'");
        let camera = node.camera.as_ref().expect("expected 'Camera' to have a camera");
        assert_eq!(camera.projection_mode, ufbx::ProjectionMode::Perspective);
    }
}

#[test]
fn not_found() {
    let err = ufbx::load_file("tests/data/not_found.fbx", LoadOpts::default())
        .err().expect("expected loading 'not_found.fbx' to fail");
    assert!(err.description.contains("File not found"));
    assert!(err.info().contains("not_found.fbx"));
    assert_eq!(err.type_, ufbx::ErrorType::FileNotFound);
}
