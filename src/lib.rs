pub mod generated;
pub mod prelude;

pub use prelude::*;
pub use generated::*;

use std::ffi::c_void;
use std::mem;
use std::vec::Vec;

#[cfg(feature = "mint")]
pub mod impl_mint;

pub fn open_memory(data: &[u8], opts: OpenMemoryOpts) -> Result<Stream> {
    let mut stream: RawStream = Default::default();
    let mut opts_mut = opts;
    let opts_raw = RawOpenMemoryOpts::from_rust(&mut opts_mut);
    let ok = unsafe { open_memory_raw(&mut stream, data, &opts_raw) }?;
    assert!(ok);
    Ok(Stream::Raw(unsafe { Unsafe::new(stream) }))
}

pub trait IntoVertexStream {
    unsafe fn get_streams(&mut self, streams: &mut Vec<VertexStream>);
    unsafe fn count(&mut self) -> usize;
    unsafe fn update(&mut self, size: usize);
}

impl<T> IntoVertexStream for &mut Vec<T> where T: Copy + Sized {
    unsafe fn get_streams(&mut self, streams: &mut Vec<VertexStream>) {
        streams.push(VertexStream {
            data: self.as_mut_ptr() as *const c_void,
            vertex_size: mem::size_of::<T>(),
        });
    }
    unsafe fn count(&mut self) -> usize {
        self.len()
    }
    unsafe fn update(&mut self, size: usize) {
        self.truncate(size);
        self.shrink_to_fit();
    }
}

/*

pub struct VertexStreams<'a> {
    pub(crate) streams: Vec<Box<dyn IntoVertexStream + 'a>>,
}

impl<'a> VertexStreams<'a> {
    pub fn new() -> Self {
        Self { streams: Vec::new() }
    }

    pub fn add<T: IntoVertexStream + 'a>(&mut self, stream: T) {
        self.streams.push(Box::new(stream));
    }
}

*/

pub fn generate_indices_vec(mut streams: impl IntoVertexStream, opts: AllocatorOpts) -> Result<Vec<u32>> {
    unsafe {
        let mut raw_streams: Vec<VertexStream> = Vec::new();
        streams.get_streams(&mut raw_streams);

        let mut indices: Vec<u32> = Vec::new();
        let num_indices = streams.count();
        indices.reserve(num_indices);
        let indices_uninit = indices.spare_capacity_mut();

        let verts = generate_indices(&raw_streams, mem::transmute(indices_uninit), opts)?;
        streams.update(verts);
        indices.set_len(num_indices);

        Ok(indices)
    }
}

pub fn triangulate_face_vec(mut indices: &mut Vec<u32>, mesh: &Mesh, face: Face) -> u32 {
    if face.num_indices <= 3 {
        indices.clear();
        return 0;
    }

    let num_triangles = face.num_indices as usize - 2;
    indices.resize(num_triangles * 3, 0);
    let num_triangles = triangulate_face(&mut indices, mesh, face);
    indices.shrink_to(num_triangles as usize * 3);
    num_triangles
}
