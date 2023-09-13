#![cfg_attr(feature="nightly", feature(no_coverage))]

pub mod generated;
pub mod prelude;

pub use prelude::*;
pub use generated::*;

use std::vec::Vec;

#[cfg(feature = "mint")]
pub mod impl_mint;

/*
pub fn open_memory(data: &[u8], opts: OpenMemoryOpts) -> Result<Stream> {
    let mut stream: RawStream = Default::default();
    let mut opts_mut = opts;
    let opts_raw = RawOpenMemoryOpts::from_rust(&mut opts_mut);
    let ok = unsafe { open_memory_raw(&mut stream, data, &opts_raw) }?;
    assert!(ok);
    Ok(Stream::Raw(unsafe { Unsafe::new(stream) }))
}
*/

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
