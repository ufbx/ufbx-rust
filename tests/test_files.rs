use ufbx;
use std::vec::Vec;

/*
#[test]
fn load_file() {
    let result = ufbx::load_file_len("test.fbx", ufbx::LoadOpts::default());
    let scene: ufbx::SceneRoot = result.expect("Expected to load scene");
    assert!(scene.metadata.creator == "Blender (stable FBX IO) - 2.79 (sub 0) - 3.7.13");
}
*/

#[test]
fn load_progress() {
    let mut opts = ufbx::LoadOpts::default();

    let mut count: usize = 0;

    let mut cb = |progress: &ufbx::Progress| -> ufbx::ProgressResult {
        println!("{}/{}", progress.bytes_read, progress.bytes_total);
        count += 1; 
        if count > 1 {
            println!("cancel?");
            return ufbx::ProgressResult::Cancel
        }
        ufbx::ProgressResult::Continue
    };

    // opts.progress_cb = ufbx::ProgressCb::Mut(&mut cb);

    let file = std::fs::File::open("test.fbx").unwrap();
    let result = ufbx::load_stream(ufbx::Stream::File(file), opts);
    let scene: ufbx::SceneRoot = result.expect("Expected to load scene");

    for elem in &scene.elements {
        println!("elem: {:?} '{}'", elem.type_, elem.name);
        match elem.as_data() {
        ufbx::ElementData::Node(mesh) => {
            println!("  parent: {}", mesh.parent.as_ref().map(|p| p.element.element_id).unwrap_or(0u32));
        },
        ufbx::ElementData::Mesh(mesh) => {
            println!("  faces: {}", mesh.num_faces);
        },
        _ => (),
        }
    }

    let mesh = &scene.meshes[0];

    let mut topo = Vec::<ufbx::TopoEdge>::new();
    topo.resize(mesh.num_indices, ufbx::TopoEdge::default());
    ufbx::compute_topology(mesh, &mut topo);

    let mut indices = Vec::<u32>::new();
    indices.resize(500, 0);

    let face = ufbx::Face{
        index_begin: 10,
        num_indices: 100,
    };

    ufbx::triangulate_face(&mut indices, mesh, face);

    let scene2 = scene.clone();

    println!("{}", count);
}