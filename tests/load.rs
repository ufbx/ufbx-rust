use std::{fs::File, io::{BufReader, Read}};
use std::ffi::{CString, c_void};
use ufbx;
use libc;

fn check_blender_default(scene: &ufbx::Scene, ignore_geometry: bool) {
    {
        let node = scene.find_node("Cube").expect("expected to find 'Cube'");
        let mesh = node.mesh.as_ref().expect("expected 'Cube' to have a mesh");
        if !ignore_geometry {
            assert_eq!(mesh.num_faces, 6);
        }

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

fn read_file(path: &str) -> Vec<u8> {
    let file = File::open(path).expect("could not find file");
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::<u8>::new();
    reader.read_to_end(&mut buffer).expect("failed to read");
    buffer
}

#[test]
fn blender_default() {
    let scene = ufbx::load_file("tests/data/blender_default.fbx", ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene, false);
}

#[test]
fn blender_default_memory() {
    let data = read_file("tests/data/blender_default.fbx");
    let scene = ufbx::load_memory(&data, ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene, false);
}

#[test]
fn blender_default_file() {
    let path = "tests/data/blender_default.fbx";
    let file = File::open(path).expect("could not find file");
    let scene = ufbx::load_stream(ufbx::Stream::Read(Box::new(file)), ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene, false);
}

#[test]
fn blender_default_file_skip() {
    let path = "tests/data/blender_default.fbx";
    let file = File::open(path).expect("could not find file");
    let opts = ufbx::LoadOpts {
        ignore_geometry: true,
        read_buffer_size: 32,
        ..Default::default()
    };
    let scene = ufbx::load_stream(ufbx::Stream::Read(Box::new(file)), opts)
        .expect("expected to load scene");
    check_blender_default(&scene, true);
}

#[test]
fn blender_default_reader() {
    let path = "tests/data/blender_default.fbx";
    let file = File::open(path).expect("could not find file");
    let reader = BufReader::new(file);
    let scene = ufbx::load_stream(ufbx::Stream::Read(Box::new(reader)), ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene, false);
}

struct ByteReader<R: Read>(R);

impl<R: Read> ufbx::StreamInterface for ByteReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        self.0.read(&mut buf[0..1]).ok()
    }
}

#[test]
fn blender_default_custom_stream() {
    let path = "tests/data/blender_default.fbx";
    let file = File::open(path).expect("could not find file");
    let reader = ByteReader(BufReader::new(file));
    let scene = ufbx::load_stream(ufbx::Stream::Box(Box::new(reader)), ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene, false);
}

struct SkipReader<R: Read> {
    pub reader: R,
    pub skip_calls: usize,
    pub require_skip_calls: usize,
}

impl<R: Read> ufbx::StreamInterface for SkipReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        self.reader.read(&mut buf[0..1]).ok()
    }
    fn skip(&mut self, bytes: usize) -> bool {
        let mut byte = [0u8];
        self.skip_calls += 1;
        for _ in 0..bytes {
            self.reader.read(&mut byte).expect("expected to skip byte");
        }
        true
    }
    fn close(&mut self) {
        assert!(self.skip_calls >= self.require_skip_calls,
            "expected at least {} skip calls, got {}",
            self.require_skip_calls, self.skip_calls);
    }
}

#[test]
fn blender_default_custom_stream_skip() {
    let path = "tests/data/blender_default.fbx";
    let file = File::open(path).expect("could not find file");
    let reader = SkipReader{
        reader: BufReader::new(file),
        skip_calls: 0,
        require_skip_calls: 4,
    };
    let opts = ufbx::LoadOpts {
        ignore_geometry: true,
        read_buffer_size: 16,
        ..Default::default()
    };
    let scene = ufbx::load_stream(ufbx::Stream::Box(Box::new(reader)), opts)
        .expect("expected to load scene");
    check_blender_default(&scene, true);
}

#[test]
fn blender_default_reader_prefix() {
    let path = "tests/data/blender_default.fbx";
    let file = File::open(path).expect("could not find file");
    let mut reader = BufReader::new(file);

    let mut prefix = [0u8; 64];
    let num_read = reader.read(&mut prefix).expect("failed to read prefix");
    assert_eq!(num_read, prefix.len());

    let stream = ufbx::Stream::Read(Box::new(reader));
    let scene = ufbx::load_stream_prefix(stream, &prefix, ufbx::LoadOpts::default())
        .expect("expected to load scene");
    check_blender_default(&scene, false);
}

#[test]
fn blender_default_stdio() {
    let path = "tests/data/blender_default.fbx";
    let result = unsafe {
        let c_path = CString::new(path).unwrap();
        let c_mode = CString::new("rb").unwrap();
        let file = libc::fopen(c_path.as_ptr(), c_mode.as_ptr());
        let result = ufbx::load_stdio(file as *mut c_void, ufbx::LoadOpts::default());
        libc::fclose(file);
        result
    };
    let scene = result.expect("expected to load scene");
    check_blender_default(&scene, false);
}

#[test]
fn blender_default_stdio_prefix() {
    let path = "tests/data/blender_default.fbx";
    let result = unsafe {
        let c_path = CString::new(path).unwrap();
        let c_mode = CString::new("rb").unwrap();
        let file = libc::fopen(c_path.as_ptr(), c_mode.as_ptr());
        
        let mut prefix = [0u8; 32];
        let num_read = libc::fread(prefix.as_mut_ptr() as *mut c_void, 1, prefix.len(), file);
        assert_eq!(num_read, prefix.len());
        
        let result = ufbx::load_stdio_prefix(file as *mut c_void, &prefix, ufbx::LoadOpts::default());
        libc::fclose(file);
        result
    };
    let scene = result.expect("expected to load scene");
    check_blender_default(&scene, false);
}

#[test]
fn not_found() {
    let err = ufbx::load_file("tests/data/not_found.fbx", ufbx::LoadOpts::default())
        .err().expect("expected loading 'not_found.fbx' to fail");
    assert!(err.description.contains("File not found"));
    assert!(err.info().contains("not_found.fbx"));
    assert_eq!(err.type_, ufbx::ErrorType::FileNotFound);
}
