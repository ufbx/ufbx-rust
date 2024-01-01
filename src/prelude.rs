use std::any::Any;
use std::{ptr, str, slice};
use std::marker::PhantomData;
use std::ops::{Deref, Index};
use std::alloc::{self,Layout,System,GlobalAlloc};
use std::ffi::c_void;
use std::cmp::min;
use std::io::{Read,Seek,SeekFrom};
use std::fs::File;
use std::mem;
use std::string;
use std::ptr::NonNull;
use std::fmt::{self, Debug, Display, Formatter};
use crate::{OpenFileInfo, RawThreadPool};
use crate::generated::{RawStream, RawAllocator, RawVertexStream, Progress, ProgressResult, Error, Vec2, Vec3, Vec4};
use crate::generated::format_error;

pub type Real = f64;
pub type ThreadPoolContext = usize;

#[repr(C)]
pub struct List<T> {
    data: *const T,
    pub count: usize,
    _marker: PhantomData<T>,
}

impl<T> List<T> {
    pub(crate) unsafe fn as_static_ref(&self) -> &'static [T] {
        slice::from_raw_parts(self.data, self.count)
    }
}

impl<T> AsRef<[T]> for List<T> {
    fn as_ref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data, self.count) }
    }
}

impl<T> Deref for List<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.data, self.count) }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_iter()
    }
}

impl<T> Index<usize> for List<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.as_ref()[index]
    }
}

#[repr(C)]
pub struct RefList<T> {
    data: *const Ref<T>,
    pub count: usize,
    _marker: PhantomData<T>,
}

impl<T> RefList<T> {
    #[allow(dead_code)]
    pub(crate) unsafe fn as_static_ref(&self) -> &'static [Ref<T>] {
        slice::from_raw_parts(self.data, self.count)
    }
}

impl<T> AsRef<[Ref<T>]> for RefList<T> {
    fn as_ref(&self) -> &[Ref<T>] {
        unsafe { slice::from_raw_parts(self.data, self.count) }
    }
}

impl<T> Deref for RefList<T> {
    type Target = [Ref<T>];
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.data, self.count) }
    }
}

pub struct RefIter<'a, T> {
    inner: slice::Iter<'a, Ref<T>>,
}

impl<'a, T> Iterator for RefIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|v| v.as_ref())
    }
}

impl<'a, T> IntoIterator for &'a RefList<T> {
    type Item = &'a T;
    type IntoIter = RefIter<'a, T>;
    fn into_iter(self) -> RefIter<'a, T> {
        RefIter::<'_, T> { inner: self.as_ref().into_iter() }
    }
}

impl<T> Index<usize> for RefList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.as_ref()[index]
    }
}

#[repr(transparent)]
pub struct Ref<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> AsRef<T> for Ref<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr.as_ptr() }
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ptr() }
    }
}

#[repr(C)]
pub struct RawString {
    pub data: *const u8,
    pub length: usize,
}

impl RawString {
    fn new(s: &[u8]) -> Self {
        RawString {
            data: s.as_ptr(),
            length: s.len(),
        }
    }
}

impl Default for RawString {
    fn default() -> Self {
        RawString {
            data: ptr::null(),
            length: 0,
        }
    }
}

#[repr(C)]
pub struct RawBlob {
    pub data: *const u8,
    pub size: usize,
}

impl RawBlob {
    fn new(s: &[u8]) -> Self {
        RawBlob {
            data: s.as_ptr(),
            size: s.len(),
        }
    }
}

impl Default for RawBlob {
    fn default() -> Self {
        RawBlob {
            data: ptr::null(),
            size: 0,
        }
    }
}

#[repr(C)]
pub struct RawList<T> {
    pub data: *const T,
    pub count: usize,
}

impl<T> Default for RawList<T> {
    fn default() -> Self {
        RawList {
            data: ptr::null(),
            count: 0,
        }
    }
}

#[repr(C)]
pub struct OptionRef<T> {
    ptr: *const T,
    _marker: PhantomData<T>,
}

impl<T> OptionRef<T> {
    pub fn is_some(&self) -> bool { self.ptr.is_null() }
    pub fn is_none(&self) -> bool { !self.ptr.is_null() }

    pub fn as_ref(&self) -> Option<&T> {
        unsafe { self.ptr.as_ref() }
    }
}

#[repr(C)]
pub struct String {
    data: *const u8,
    pub length: usize,
    _marker: PhantomData<u8>,
}

impl String {
    pub(crate) unsafe fn as_static_ref(&self) -> &'static str {
        str::from_utf8_unchecked(slice::from_raw_parts(self.data, self.length))
    }
}

impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        unsafe { str::from_utf8_unchecked(slice::from_raw_parts(self.data, self.length)) }
    }
}

impl Deref for String {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Default for String {
    fn default () -> String {
        String{ data: ptr::null(), length: 0, _marker: PhantomData }
    }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.deref())
    }
}

impl<'a> PartialEq<&'a str> for String {
    fn eq(&self, rhs: &&'a str) -> bool {
        &self.as_ref() == rhs
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

#[repr(C)]
pub struct Blob {
    data: *const u8,
    pub size: usize,
    _marker: PhantomData<u8>,
}

impl Deref for Blob {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.data, self.size) }
    }
}

pub trait AllocatorInterface {
    fn alloc(&mut self, layout: Layout) -> *mut u8;
    fn free(&mut self, ptr: *mut u8, layout: Layout);
    fn realloc(&mut self, ptr: *mut u8, old_layout: Layout, new_layout: Layout) -> *mut u8 {
        self.free(ptr, old_layout);
        self.alloc(new_layout)
    }
    fn free_allocator(&mut self) { }
}

#[repr(transparent)]
#[derive(Default)]
pub struct Unsafe<T>(T);

impl<T> Unsafe<T> {
    pub unsafe fn new(t: T) -> Self { Self(t) }
}

impl<T> Unsafe<T> where T: Default {
    pub fn take(&mut self) -> T { mem::take(&mut self.0) }
}

pub trait StreamInterface {
    fn read(&mut self, buf: &mut [u8]) -> Option<usize>;
    fn skip(&mut self, bytes: usize) -> bool {
        #![allow(deprecated)]
        unsafe {
            let mut local_buf: [mem::MaybeUninit<u8>; 512] = mem::MaybeUninit::uninit().assume_init();
            let mut left = bytes;
            while left > 0 {
                let to_read = min(left, local_buf.len());
                let num_read = self.read(mem::transmute(&mut local_buf[0..to_read])).unwrap_or(0);
                if num_read != to_read { return false }
                left -= num_read
            }
            true
        }
    }
    fn close(&mut self) { }
}

pub enum Stream {
    File(File),
    Read(Box<dyn Read>),
    Box(Box<dyn StreamInterface>),
    Raw(Unsafe<RawStream>),
}

unsafe extern "C" fn global_alloc(_user: *mut c_void, size: usize) -> *mut c_void {
    let layout = Layout::from_size_align(size, 8).unwrap();
    alloc::alloc(layout) as *mut _
}

unsafe extern "C" fn global_realloc(_user: *mut c_void, ptr: *mut c_void, old_size: usize, new_size: usize) -> *mut c_void {
    let old_layout = Layout::from_size_align(old_size, 8).unwrap();
    alloc::realloc(ptr as *mut _, old_layout, new_size) as *mut _
}

unsafe extern "C" fn global_free(_user: *mut c_void, ptr: *mut c_void, size: usize) {
    let layout = Layout::from_size_align(size, 8).unwrap();
    alloc::dealloc(ptr as *mut _, layout)
}

unsafe extern "C" fn system_alloc(_user: *mut c_void, size: usize) -> *mut c_void {
    let layout = Layout::from_size_align(size, 8).unwrap();
    System.alloc(layout) as *mut _
}

unsafe extern "C" fn system_realloc(_user: *mut c_void, ptr: *mut c_void, old_size: usize, new_size: usize) -> *mut c_void {
    let old_layout = Layout::from_size_align(old_size, 8).unwrap();
    System.realloc(ptr as *mut _, old_layout, new_size) as *mut _
}

unsafe extern "C" fn system_free(_user: *mut c_void, ptr: *mut c_void, size: usize) {
    let layout = Layout::from_size_align(size, 8).unwrap();
    System.dealloc(ptr as *mut _, layout)
}

unsafe extern "C" fn allocator_imp_alloc(user: *mut c_void, size: usize) -> *mut c_void {
    let ator: &mut Box<dyn AllocatorInterface> = &mut *(user as *mut Box<dyn AllocatorInterface>);
    let layout = Layout::from_size_align(size, 8).unwrap();
    ator.alloc(layout) as *mut _
}

unsafe extern "C" fn allocator_imp_realloc(user: *mut c_void, ptr: *mut c_void, old_size: usize, new_size: usize) -> *mut c_void {
    let ator: &mut Box<dyn AllocatorInterface> = &mut *(user as *mut Box<dyn AllocatorInterface>);
    let old_layout = Layout::from_size_align(old_size, 8).unwrap();
    let new_layout = Layout::from_size_align(new_size, 8).unwrap();
    ator.realloc(ptr as *mut _, old_layout, new_layout) as *mut _
}

unsafe extern "C" fn allocator_imp_free(user: *mut c_void, ptr: *mut c_void, size: usize) {
    let ator: &mut Box<dyn AllocatorInterface> = &mut *(user as *mut Box<dyn AllocatorInterface>);
    let layout = Layout::from_size_align(size, 8).unwrap();
    ator.free(ptr as *mut _, layout)
}

unsafe extern "C" fn allocator_imp_box_free_allocator(user: *mut c_void) {
    let mut ator = Box::from_raw(user as *mut Box<dyn AllocatorInterface>);
    ator.free_allocator()
}

pub enum Allocator {
    Libc,
    Global,
    System,
    Box(Box<dyn AllocatorInterface>),
    Raw(Unsafe<RawAllocator>),
}

impl Default for Allocator {
    fn default() -> Self { Allocator::Global }
}

impl Allocator {
    pub(crate) fn from_rust(&self) -> RawAllocator {
        match self {
        Allocator::Libc => RawAllocator {
            alloc_fn: None,
            realloc_fn: None,
            free_fn: None,
            free_allocator_fn: None,
            user: ptr::null::<c_void>() as *mut c_void,
        },
        Allocator::Global => RawAllocator {
            alloc_fn: Some(global_alloc),
            realloc_fn: Some(global_realloc),
            free_fn: Some(global_free),
            free_allocator_fn: None,
            user: ptr::null::<c_void>() as *mut c_void,
        },
        Allocator::System => RawAllocator {
            alloc_fn: Some(system_alloc),
            realloc_fn: Some(system_realloc),
            free_fn: Some(system_free),
            free_allocator_fn: None,
            user: ptr::null::<c_void>() as *mut c_void,
        },
        _ => panic!("required mutable reference"),
        }
    }
    pub(crate) fn from_rust_mut(&mut self) -> RawAllocator {
        match self {
        Allocator::Box(b) => RawAllocator {
            alloc_fn: Some(allocator_imp_alloc),
            realloc_fn: Some(allocator_imp_realloc),
            free_fn: Some(allocator_imp_free),
            free_allocator_fn: Some(allocator_imp_box_free_allocator),
            user: Box::into_raw(Box::new(b)) as *mut _,
        },
        Allocator::Raw(raw) => raw.take(),
        _ => Self::from_rust(self),
        }
    }
}

pub enum ThreadPool {
    None,
    Raw(Unsafe<RawThreadPool>),
}

impl Default for ThreadPool {
    fn default() -> Self { ThreadPool::None }
}
impl ThreadPool {
    pub(crate) fn from_rust(&self) -> RawThreadPool {
        match self {
        ThreadPool::None => RawThreadPool {
            init_fn: None,
            run_fn: None,
            wait_fn: None,
            free_fn: None,
            user: ptr::null::<c_void>() as *mut c_void,
        },
        _ => panic!("required mutable reference"),
        }
    }
    pub(crate) fn from_rust_mut(&mut self) -> RawThreadPool {
        match self {
        ThreadPool::None => RawThreadPool {
            init_fn: None,
            run_fn: None,
            wait_fn: None,
            free_fn: None,
            user: ptr::null::<c_void>() as *mut c_void,
        },
        ThreadPool::Raw(raw) => raw.take(),
        }
    }
}

pub struct VertexStream<'a> {
    pub(crate) data: *mut c_void,
    pub vertex_count: usize,
    pub vertex_size: usize,
    _marker: PhantomData<&'a mut ()>,
}

impl VertexStream<'_> {
    pub fn new<T: Copy + Sized>(data: &mut [T]) -> VertexStream {
        return VertexStream {
            data: data.as_mut_ptr() as *mut c_void,
            vertex_count: data.len(),
            vertex_size: mem::size_of::<T>(),
            _marker: PhantomData,
        }
    }
}

impl<'a> FromRust for [VertexStream<'a>] {
    type Result = Vec<RawVertexStream>;
    fn from_rust_mut(&mut self, _arena: &mut Arena) -> Self::Result {
        self.iter().map(|s| RawVertexStream {
            data: s.data,
            vertex_count: s.vertex_count,
            vertex_size: s.vertex_size,
        }).collect()
    }
}

unsafe extern "C" fn stream_read_read(user: *mut c_void, buf: *mut c_void, size: usize) -> usize {
    let imp = &mut *(user as *mut Box<dyn Read>);
    imp.read(slice::from_raw_parts_mut(buf as *mut u8, size)).unwrap_or(usize::MAX)
}

unsafe extern "C" fn stream_read_close(user: *mut c_void) {
    let _ = Box::from_raw(user as *mut Box<dyn Read>);
}

unsafe extern "C" fn stream_imp_read(user: *mut c_void, buf: *mut c_void, size: usize) -> usize {
    let imp = &mut *(user as *mut Box<dyn StreamInterface>);
    imp.read(slice::from_raw_parts_mut(buf as *mut u8, size)).unwrap_or(usize::MAX)
}

unsafe extern "C" fn stream_imp_skip(user: *mut c_void, size: usize) -> bool {
    let imp = &mut *(user as *mut Box<dyn StreamInterface>);
    imp.skip(size)
}

unsafe extern "C" fn stream_imp_box_close(user: *mut c_void) {
    let mut imp = Box::from_raw(user as *mut Box<dyn StreamInterface>);
    imp.close()
}

struct StreamRead<T: Read>(T);

impl<T: Read> StreamInterface for StreamRead<T> {
    fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        self.0.read(buf).ok()
    }
}

struct StreamReadSeek<T: Read + Seek>(T);

impl<T: Read + Seek> StreamInterface for StreamReadSeek<T> {
    fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        self.0.read(buf).ok()
    }
    fn skip(&mut self, bytes: usize) -> bool {
        match self.0.stream_position() {
            Ok(cur) => match self.0.seek(SeekFrom::Current(bytes as i64)) {
                Ok(pos) => pos == cur + (bytes as u64),
                Err(_) => false,
            },
            Err(_) => false,
        }
    }
}

impl Stream {
    pub(crate) fn from_rust_mut(&mut self) -> RawStream {
        let local = mem::replace(self, Stream::Raw(unsafe { Unsafe::new(Default::default()) }));
        match local {
            Stream::File(file) => {
                let mut inner = Stream::Box(Box::new(StreamReadSeek(file)));
                inner.from_rust_mut()
            },
            Stream::Read(b) => RawStream {
                read_fn: Some(stream_read_read),
                skip_fn: None,
                close_fn: Some(stream_read_close),
                user: Box::into_raw(Box::new(b)) as *mut _,
            },
            Stream::Box(b) => RawStream {
                read_fn: Some(stream_imp_read),
                skip_fn: Some(stream_imp_skip),
                close_fn: Some(stream_imp_box_close),
                user: Box::into_raw(Box::new(b)) as *mut _,
            },
            Stream::Raw(mut raw) => raw.take(),
        }
    }
}

pub unsafe extern "C" fn call_progress_cb<F>(user: *mut c_void, progress: *const Progress) -> ProgressResult
    where F: FnMut(&Progress) -> ProgressResult
{
    let func: &mut F = &mut *(user as *mut F);
    (func)(&*progress)
}

pub unsafe extern "C" fn call_open_file_cb<F>(user: *mut c_void, dst: *mut RawStream, path: *const u8, path_len: usize, info: *const OpenFileInfo) -> bool
    where F: FnMut(&str, &OpenFileInfo) -> Option<Stream>
{
    let func: &mut F = &mut *(user as *mut F);

    let path_str = match str::from_utf8(slice::from_raw_parts(path, path_len)) {
        Ok(path_str) => path_str,
        Err(_) => return false,
    };

    let mut stream = match (func)(path_str, &*info) {
        Some(stream) => stream,
        None => return false,
    };

    *dst = stream.from_rust_mut();
    true
}

pub unsafe extern "C" fn call_close_memory_cb<F>(user: *mut c_void, data: *mut c_void, data_size: usize)
    where F: FnMut(*mut c_void, usize) -> ()
{
    let func: &mut F = &mut *(user as *mut F);
    (func)(data, data_size)
}

#[repr(transparent)]
pub struct InlineBuf<T> {
    pub data: mem::MaybeUninit<T>,
}

impl<T> Default for InlineBuf<T> {
    fn default() -> Self {
        Self { data: mem::MaybeUninit::uninit() }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #![allow(deprecated)]
        unsafe {
            let mut local_buf: [mem::MaybeUninit<u8>; 1024] = mem::MaybeUninit::uninit().assume_init();
            let length = format_error(mem::transmute(local_buf.as_mut_slice()), self);
            f.write_str(str::from_utf8_unchecked(mem::transmute(&local_buf[..length])))
        }
    }
}

#[repr(C)]
pub struct ExternalRef<'a, T> {
    data: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> ExternalRef<'a, T> {
    pub unsafe fn new(t: T) -> Self {
        Self {
            data: t,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> AsRef<T> for ExternalRef<'a, T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<'a, T> Deref for ExternalRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub(crate) struct Arena {
    items: Vec<Box<dyn Any>>,
}

impl Arena {
    pub fn new() -> Arena {
        Arena{
            items: Vec::new(),
        }
    }

    #[allow(unused)]
    pub fn push_box<T: 'static>(&mut self, s: Box<T>) -> *const T {
        let ptr = Box::as_ref(&s) as *const T;
        self.items.push(s);
        ptr
    }
    pub fn push_vec<T: 'static>(&mut self, vec: Vec<T>) -> *const T {
        if vec.len() == 0 { return ptr::null(); }
        let ptr = vec.as_ptr();
        self.items.push(Box::new(vec));
        ptr
    }
}

pub fn format_flags(f: &mut fmt::Formatter<'_>, names: &[(&str, u32)], value: u32) -> fmt::Result {
    let mut has_any = false;

    for (name, v) in names {
        if (value & v) != 0 {
            let prefix = if has_any { "|" } else { "" };
            has_any = true;
            write!(f, "{}{}", prefix, name)?;
        }
    }

    if !has_any {
            write!(f, "NONE")?;
    }

    Ok(())
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (f.precision(), f.sign_plus()) {
            (None, false) => write!(f, "({}, {})", self.x, self.y),
            (None, true) => write!(f, "({:+}, {:+})", self.x, self.y),
            (Some(p), false) => write!(f, "({1:.0$}, {2:.0$})", p, self.x, self.y),
            (Some(p), true) => write!(f, "({1:+.0$}, {2:+.0$})", p, self.x, self.y),
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (f.precision(), f.sign_plus()) {
            (None, false) => write!(f, "({}, {}, {})", self.x, self.y, self.z),
            (None, true) => write!(f, "({:+}, {:+}, {:+})", self.x, self.y, self.z),
            (Some(p), false) => write!(f, "({1:.0$}, {2:.0$}, {3:.0$})", p, self.x, self.y, self.z),
            (Some(p), true) => write!(f, "({1:+.0$}, {2:+.0$}, {3:+.0$})", p, self.x, self.y, self.z),
        }
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (f.precision(), f.sign_plus()) {
            (None, false) => write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w),
            (None, true) => write!(f, "({:+}, {:+}, {:+}, {})", self.x, self.y, self.z, self.w),
            (Some(p), false) => write!(f, "({1:.0$}, {2:.0$}, {3:.0$}, {4:.0$})", p, self.x, self.y, self.z, self.w),
            (Some(p), true) => write!(f, "({1:+.0$}, {2:+.0$}, {3:+.0$}, {4:+.0$})", p, self.x, self.y, self.z, self.w),
        }
    }
}

pub(crate) trait FromRust {
    type Result: 'static;
    fn from_rust(&self, _arena: &mut Arena) -> Self::Result {
        panic!("type must be used via mutable reference")
    }
    fn from_rust_mut(&mut self, arena: &mut Arena) -> Self::Result {
        self.from_rust(arena)
    }
}

pub enum StringOpt<'a> {
    Unset,
    Ref(&'a str),
    Owned(string::String),
}

impl Default for StringOpt<'_> {
    fn default() -> Self {
        StringOpt::Unset
    }
}

impl<'a> From<&'a str> for StringOpt<'a> {
    fn from(v: &'a str) -> Self {
        StringOpt::Ref(v)
    }
}

impl<'a> From<string::String> for StringOpt<'a> {
    fn from(v: string::String) -> Self {
        StringOpt::Owned(v)
    }
}

impl<'a> FromRust for StringOpt<'a> {
    type Result = RawString;
    fn from_rust(&self, _arena: &mut Arena) -> Self::Result {
        match self {
        StringOpt::Unset => RawString::default(),
        StringOpt::Ref(r) => RawString::new(r.as_bytes()),
        StringOpt::Owned(r) => RawString::new(r.as_bytes()),
        }
    }
}

pub enum BlobOpt<'a> {
    Unset,
    Ref(&'a [u8]),
    Owned(Vec<u8>),
}

impl Default for BlobOpt<'_> {
    fn default() -> Self {
        BlobOpt::Unset
    }
}

impl<'a> From<&'a [u8]> for BlobOpt<'a> {
    fn from(v: &'a [u8]) -> Self {
        BlobOpt::Ref(v)
    }
}

impl<'a> From<Vec<u8>> for BlobOpt<'a> {
    fn from(v: Vec<u8>) -> Self {
        BlobOpt::Owned(v)
    }
}

impl<'a> FromRust for BlobOpt<'a> {
    type Result = RawBlob;
    fn from_rust(&self, _arena: &mut Arena) -> Self::Result {
        match self {
        BlobOpt::Unset => RawBlob::default(),
        BlobOpt::Ref(r) => RawBlob::new(r),
        BlobOpt::Owned(r) => RawBlob::new(r.as_slice()),
        }
    }
}

pub enum ListOpt<'a, T> {
    Unset,
    Ref(&'a [T]),
    Mut(&'a mut [T]),
    Owned(Vec<T>),
}

impl<T> Default for ListOpt<'_, T> {
    fn default() -> Self {
        ListOpt::Unset
    }
}

impl<'a, T> From<&'a [T]> for ListOpt<'a, T> {
    fn from(v: &'a [T]) -> Self {
        ListOpt::Ref(v)
    }
}

impl<'a, T> From<Vec<T>> for ListOpt<'a, T> {
    fn from(v: Vec<T>) -> Self {
        ListOpt::Owned(v)
    }
}

impl<'a, T: FromRust> FromRust for ListOpt<'a, T> {
    type Result = RawList<T::Result>;

    fn from_rust(&self, arena: &mut Arena) -> Self::Result {
        let items: Vec<T::Result> = match self {
            ListOpt::Unset => return RawList::default(),
            ListOpt::Ref(v) => v.iter().map(|v| T::from_rust(v, arena)).collect(),
            ListOpt::Mut(v) => v.iter().map(|v| T::from_rust(v, arena)).collect(),
            ListOpt::Owned(v) => v.iter().map(|v| T::from_rust(v, arena)).collect(),
        };
        let count = items.len();
        RawList { data: arena.push_vec(items), count }
    }

    fn from_rust_mut(&mut self, arena: &mut Arena) -> Self::Result {
        let items: Vec<T::Result> = match mem::take(self) {
            ListOpt::Unset => return RawList::default(),
            ListOpt::Ref(v) => v.iter().map(|v| T::from_rust(v, arena)).collect(),
            ListOpt::Mut(v) => v.into_iter().map(|v| T::from_rust_mut(v, arena)).collect(),
            ListOpt::Owned(v) => v.into_iter().map(|mut v| T::from_rust_mut(&mut v, arena)).collect(),
        };
        let count = items.len();
        RawList { data: arena.push_vec(items), count }
    }

}

impl<T: Copy + 'static> FromRust for T {
    type Result = T;
    fn from_rust(&self, _arena: &mut Arena) -> Self::Result {
        *self
    }
}
