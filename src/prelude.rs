use std::{ptr, str, slice};
use std::marker::PhantomData;
use std::ops::{Deref};
use std::alloc::{self,Layout,System,GlobalAlloc};
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::cmp::min;
use std::io::{Read,Seek,SeekFrom};
use std::fs::File;
use std::mem;
use std::ptr::NonNull;
use std::fmt::{self, Debug, Display, Formatter};
use crate::generated::{RawStream, RawAllocator, Progress, ProgressResult, Error};
use crate::generated::{format_error};

pub type Real = f64;

#[repr(C)]
pub struct List<T> {
    data: *const T,
    pub count: usize,
    _marker: PhantomData<T>,
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

#[repr(C)]
pub struct RefList<T> {
    data: *const Ref<T>,
    pub count: usize,
    _marker: PhantomData<T>,
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

impl<'a, T> IntoIterator for &'a RefList<T> {
    type Item = &'a Ref<T>;
    type IntoIter = slice::Iter<'a, Ref<T>>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_iter()
    }
}

#[repr(transparent)]
pub struct Ref<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ptr() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawString {
    pub data: *const u8,
    pub length: usize,
}

impl Default for RawString {
    fn default() -> Self {
        RawString {
            data: ptr::null(),
            length: 0,
        }
    }
}

impl RawString {
    pub fn from_rust(s: &mut Option<&str>) -> RawString {
        match s {
            Some(s) => RawString { data: s.as_ptr(), length: s.len() },
            None => Default::default(),
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
            let mut local_buf: [u8; 512] = std::mem::uninitialized();
            let mut left = bytes;
            while left > 0 {
                let to_read = min(left, local_buf.len());
                let num_read = self.read(&mut local_buf[0..to_read]).unwrap_or(0);
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

/*

pub trait StreamInterface {
    fn read(&mut self, buf: &mut [u8]) -> Option<usize>;
    fn skip(&mut self, bytes: usize) -> bool {
        #![allow(deprecated)]
        unsafe {
            let mut local_buf: [u8; 512] = std::mem::uninitialized();
            let mut left = bytes;
            while left > 0 {
                let to_read = min(left, local_buf.len());
                let num_read = self.read(&mut local_buf[0..to_read]).unwrap_or(0);
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
    Raw(RawStream),
}

*/

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

impl RawAllocator {
    pub fn from_rust(a: &mut Allocator) -> RawAllocator {
        match a {
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
        Allocator::Box(b) => RawAllocator {
            alloc_fn: Some(allocator_imp_alloc),
            realloc_fn: Some(allocator_imp_realloc),
            free_fn: Some(allocator_imp_free),
            free_allocator_fn: Some(allocator_imp_box_free_allocator),
            user: Box::into_raw(Box::new(b)) as *mut _,
        },
        Allocator::Raw(raw) => raw.take(),
        }
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

impl RawStream {
    pub fn from_rust(s: &mut Stream) -> RawStream {
        let local = mem::replace(s, Stream::Raw(unsafe { Unsafe::new(Default::default()) }));
        match local {
            Stream::File(file) => {
                let mut inner = Stream::Box(Box::new(StreamReadSeek(file)));
                RawStream::from_rust(&mut inner)
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

pub unsafe extern "C" fn call_open_file_cb<F>(user: *mut c_void, dst: *mut RawStream, path: *const u8, path_len: usize) -> bool
    where F: FnMut(&str) -> Option<Stream>
{
    let func: &mut F = &mut *(user as *mut F);

    let path_str = match str::from_utf8(slice::from_raw_parts(path, path_len)) {
        Ok(path_str) => path_str,
        Err(_) => return false,
    };

    let mut stream = match (func)(path_str) {
        Some(stream) => stream,
        None => return false,
    };

    *dst = RawStream::from_rust(&mut stream);
    true
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #![allow(deprecated)]
        unsafe {
            let mut local_buf: [u8; 1024] = std::mem::uninitialized();
            let length = format_error(&mut local_buf, self);
            f.write_str(str::from_utf8_unchecked(&local_buf[..length]))
        }
    }
}

#[link(name="ufbx")]
extern "C" {
    pub fn ufbxi_rust_pop_assert() -> *const c_char;
}

pub fn assert_to_panic() {
    unsafe {
        let pointer = ufbxi_rust_pop_assert();
        if !pointer.is_null() {
            let c_message = CStr::from_ptr(pointer);
            let message = c_message.to_str().unwrap_or("(bad assert message)");
            panic!("{}", message);
        }
    }
}

/*

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AllocatorRaw {
    pub alloc_fn: Option<unsafe extern "C" fn (*mut c_void, usize) -> *mut c_void>,
    pub realloc_fn: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, usize, usize) -> *mut c_void>,
    pub free_fn: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, usize)>,
    pub free_allocator_fn: Option<unsafe extern "C" fn (*mut c_void)>,
    pub user: *mut c_void,
}

pub enum Allocator {
    Libc,
    Global,
    System,
    Box(Box<dyn AllocatorImp>),
    Raw(AllocatorRaw),
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StreamRaw {
    pub read_fn: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, usize) -> usize>,
    pub skip_fn: Option<unsafe extern "C" fn (*mut c_void, usize) -> bool>,
    pub close_fn: Option<unsafe extern "C" fn (*mut c_void)>,
    pub user: *mut c_void,
}

unsafe extern "C" fn stream_read_read(user: *mut c_void, buf: *mut c_void, size: usize) -> usize {
    let imp = &mut *(user as *mut Box<dyn Read>);
    imp.read(slice::from_raw_parts_mut(buf as *mut u8, size)).unwrap_or(usize::MAX)
}

unsafe extern "C" fn stream_read_close(user: *mut c_void) {
    let _ = Box::from_raw(user as *mut Box<dyn Read>);
}

unsafe extern "C" fn stream_imp_read(user: *mut c_void, buf: *mut c_void, size: usize) -> usize {
    let imp = &mut *(user as *mut Box<dyn StreamImp>);
    imp.read(slice::from_raw_parts_mut(buf as *mut u8, size)).unwrap_or(usize::MAX)
}

unsafe extern "C" fn stream_imp_skip(user: *mut c_void, size: usize) -> bool {
    let imp = &mut *(user as *mut Box<dyn StreamImp>);
    imp.skip(size)
}

unsafe extern "C" fn stream_imp_box_close(user: *mut c_void) {
    let mut imp = Box::from_raw(user as *mut Box<dyn StreamImp>);
    imp.close()
}

struct StreamRead<T: Read>(T);

impl<T: Read> StreamImp for StreamRead<T> {
    fn read(&mut self, buf: &mut [u8]) -> Option<usize> {
        self.0.read(buf).ok()
    }
}

struct StreamReadSeek<T: Read + Seek>(T);

impl<T: Read + Seek> StreamImp for StreamReadSeek<T> {
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

pub enum Stream {
    File(File),
    Read(Box<dyn Read>),
    Box(Box<dyn StreamImp>),
    Raw(StreamRaw),
}

impl StreamRaw {
    fn from(s: Stream) -> StreamRaw {
        match s {
        Stream::File(file) => StreamRaw::from(Stream::Box(Box::new(StreamReadSeek(file)))),
        Stream::Read(b) => StreamRaw {
            read_fn: Some(stream_read_read),
            skip_fn: None,
            close_fn: Some(stream_read_close),
            user: Box::into_raw(Box::new(b)) as *mut _,
        },
        Stream::Box(b) => StreamRaw {
            read_fn: Some(stream_imp_read),
            skip_fn: Some(stream_imp_skip),
            close_fn: Some(stream_imp_box_close),
            user: Box::into_raw(Box::new(b)) as *mut _,
        },
        Stream::Raw(raw) => raw,
        }
    }
}

struct OpenFileCb {
    pub open_file_fn: Option<unsafe extern "C" fn (*mut c_void, *mut StreamRaw, *const u8, usize) -> bool>,
    pub user: *mut c_void,
}

unsafe extern "C" fn open_file(user: *mut c_void, dst: *mut StreamRaw, path: *const u8, path_len: usize) -> bool {
    let func: &mut dyn FnMut(&str) -> Option<Stream> = *(user as *mut &mut _);

    let path_str = match str::from_utf8(slice::from_raw_parts(path, path_len)) {
        Ok(path_str) => path_str,
        Err(_) => return false,
    };

    let stream = match (func)(path_str) {
        Some(stream) => stream,
        None => return false,
    };

    *dst = StreamRaw::from(stream);
    true
}

impl OpenFileCb {
    fn from(s: &Option<&dyn FnMut(&str) -> Option<Stream>>) -> OpenFileCb {
        match s.as_ref() {
        Some(f) => OpenFileCb {
            open_file_fn: Some(open_file),
            user: f as *const &dyn FnMut(&str) -> Option<Stream> as *mut c_void,
        },
        None => OpenFileCb {
            open_file_fn: None,
            user: ptr::null::<c_void>() as *mut c_void,
        },
        }
    }
}

struct CacheOpts<'a> {
    allocator: Allocator,
    open_file_fn: Option<&'a dyn FnMut(&str) -> Option<Stream>>,
}

struct CacheOptsRaw {
    allocator: AllocatorRaw,
    open_file_fn: OpenFileCb,
}

impl CacheOptsRaw {
    fn from(s: &mut CacheOpts) -> CacheOptsRaw {
        CacheOptsRaw {
            allocator: AllocatorRaw::from(mem::take(&mut s.allocator)),
            open_file_fn: OpenFileCb::from(&mut s.open_file_fn),
        }
    }
}

*/
