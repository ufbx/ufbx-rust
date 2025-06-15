use core::str;

#[test]
fn dom_arrays() {
    let opts = ufbx::LoadOpts {
        retain_dom: true,
        ..Default::default()
    };
    let scene = ufbx::load_file("tests/data/blender_default.fbx", opts)
        .expect("expected to load scene");
    scene.dom_root.as_ref().expect("expected to have dom_root");

    let cube = scene.find_node("Cube").expect("expected to find 'Cube'");
    cube.element.dom_node.as_ref().expect("expected 'Cube' to have a DOM node");
    let mesh = cube.mesh.as_ref().expect("expected 'Cube' to have a mesh");
    let dom_mesh = mesh.element.dom_node.as_ref().expect("expected mesh to have a DOM node");

    let dom_indices = dom_mesh.find("PolygonVertexIndex").expect("expected to find PolygonVertexIndex");
    assert!(ufbx::dom_is_array(dom_indices));

    let dom_indices_arr = dom_indices.as_int32_list();
    assert_eq!(dom_indices_arr, [
        0, 4, 6, -3, 3, 2, 6, -8, 7, 6, 4, -6, 5, 1, 3, -8, 1, 0, 2, -4, 5, 4, 0, -2
    ]);

    let dom_indices_floats = dom_indices.as_float_list();
    assert_eq!(dom_indices_floats.len(), 0);
}

#[test]
fn dom_blob_array() {
    let opts = ufbx::LoadOpts {
        retain_dom: true,
        ..Default::default()
    };
    let scene = ufbx::load_file("tests/data/legacy_blob.fbx", opts)
        .expect("expected to load scene");

    let box_node = scene.find_node("Box01").expect("expected to find 'Box01'");

    let child_names = [
        "Pyramid04", "Pyramid05", "Pyramid06", "Pyramid07", "Pyramid08", "Pyramid09"
        , "Pyramid10", "Pyramid11", "Pyramid12", "Pyramid13", "Pyramid14", "Pyramid15"
        , "Pyramid16", "Pyramid17", "Pyramid18", "Pyramid19",
    ];
    assert!(Iterator::eq(box_node.children.iter().map(|c| c.element.name.as_ref()), child_names));

    let box_dom = box_node.element.dom_node.as_ref().expect("expected 'Box01' to have a DOM node");
    let dom_children = box_dom.find("Children").expect("expected 'Box01' DOM to have 'Children' node");
    assert!(dom_children.is_array());
    assert_eq!(dom_children.array_size(), child_names.len());

    let dom_child_blobs = dom_children.as_blob_list();
    assert_eq!(dom_child_blobs.len(), child_names.len());

    for (blob, name_ref) in dom_child_blobs.iter().zip(child_names) {
        let child_ref = format!("Model::{}", name_ref);

        let child = str::from_utf8(blob).expect("expected child name to be a string");
        assert_eq!(child, child_ref);
    }
}
