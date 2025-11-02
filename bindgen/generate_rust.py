from typing import Optional, List

import ufbx_ir as ir
import argparse
import os
import json

g_argv = None

uses = r"""
use std::ffi::{c_void};
use std::{marker, result, ptr, mem, str};
use std::fmt::{self, Debug};
use std::ops::{Deref, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, FnMut, Index};
use crate::prelude::{Real, List, Ref, RefList, String, Blob, RawString, RawBlob, RawList, Unsafe, ExternalRef, InlineBuf, VertexStream, Arena, FromRust, StringOpt, BlobOpt, ListOpt, ThreadPoolContext, OpenFileContext, format_flags};
""".strip()

post_ffi = r"""

pub struct SceneRoot {
    scene: *mut Scene,
    _marker: marker::PhantomData<Scene>,
}

pub struct MeshRoot {
    mesh: *mut Mesh,
    _marker: marker::PhantomData<Mesh>,
}

pub struct LineCurveRoot {
    line_curve: *mut LineCurve,
    _marker: marker::PhantomData<LineCurve>,
}

pub struct GeometryCacheRoot {
    cache: *mut GeometryCache,
    _marker: marker::PhantomData<GeometryCache>,
}

pub struct AnimRoot {
    anim: *mut Anim,
    _marker: marker::PhantomData<Anim>,
}

pub struct BakedAnimRoot {
    anim: *mut BakedAnim,
    _marker: marker::PhantomData<BakedAnim>,
}

impl SceneRoot {
    fn new(scene: *mut Scene) -> SceneRoot {
        SceneRoot {
            scene: scene,
            _marker: marker::PhantomData,
        }
    }
}

impl MeshRoot {
    fn new(mesh: *mut Mesh) -> MeshRoot {
        MeshRoot {
            mesh: mesh,
            _marker: marker::PhantomData,
        }
    }
}

impl LineCurveRoot {
    fn new(line_curve: *mut LineCurve) -> LineCurveRoot {
        LineCurveRoot {
            line_curve: line_curve,
            _marker: marker::PhantomData,
        }
    }
}


impl GeometryCacheRoot {
    fn new(cache: *mut GeometryCache) -> GeometryCacheRoot {
        GeometryCacheRoot {
            cache: cache,
            _marker: marker::PhantomData,
        }
    }
}

impl AnimRoot {
    fn new(anim: *mut Anim) -> AnimRoot {
        AnimRoot {
            anim: anim,
            _marker: marker::PhantomData,
        }
    }
}

impl BakedAnimRoot {
    fn new(anim: *mut BakedAnim) -> BakedAnimRoot {
        BakedAnimRoot {
            anim: anim,
            _marker: marker::PhantomData,
        }
    }
}

impl Drop for SceneRoot {
    fn drop(&mut self) {
        unsafe { ufbx_free_scene(self.scene) }
    }
}

impl Drop for MeshRoot {
    fn drop(&mut self) {
        unsafe { ufbx_free_mesh(self.mesh) }
    }
}

impl Drop for LineCurveRoot {
    fn drop(&mut self) {
        unsafe { ufbx_free_line_curve(self.line_curve) }
    }
}

impl Drop for GeometryCacheRoot {
    fn drop(&mut self) {
        unsafe { ufbx_free_geometry_cache(self.cache) }
    }
}

impl Drop for AnimRoot {
    fn drop(&mut self) {
        unsafe { ufbx_free_anim(self.anim) }
    }
}

impl Drop for BakedAnimRoot {
    fn drop(&mut self) {
        unsafe { ufbx_free_baked_anim(self.anim) }
    }
}

impl Clone for SceneRoot {
    fn clone(&self) -> Self {
        unsafe { ufbx_retain_scene(self.scene) }
        SceneRoot::new(self.scene)
    }
}

impl Clone for MeshRoot {
    fn clone(&self) -> Self {
        unsafe { ufbx_retain_mesh(self.mesh) }
        MeshRoot::new(self.mesh)
    }
}

impl Clone for LineCurveRoot {
    fn clone(&self) -> Self {
        unsafe { ufbx_retain_line_curve(self.line_curve) }
        LineCurveRoot::new(self.line_curve)
    }
}

impl Clone for GeometryCacheRoot {
    fn clone(&self) -> Self {
        unsafe { ufbx_retain_geometry_cache(self.cache) }
        GeometryCacheRoot::new(self.cache)
    }
}

impl Clone for AnimRoot {
    fn clone(&self) -> Self {
        unsafe { ufbx_retain_anim(self.anim) }
        AnimRoot::new(self.anim)
    }
}

impl Clone for BakedAnimRoot {
    fn clone(&self) -> Self {
        unsafe { ufbx_retain_baked_anim(self.anim) }
        BakedAnimRoot::new(self.anim)
    }
}

impl Deref for SceneRoot {
    type Target = Scene;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.scene }
    }
}

impl Deref for MeshRoot {
    type Target = Mesh;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mesh }
    }
}

impl Deref for LineCurveRoot {
    type Target = LineCurve;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.line_curve }
    }
}

impl Deref for GeometryCacheRoot {
    type Target = GeometryCache;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.cache }
    }
}

impl Deref for AnimRoot {
    type Target = Anim;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.anim }
    }
}

impl Deref for BakedAnimRoot {
    type Target = BakedAnim;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.anim }
    }
}

unsafe impl Send for SceneRoot {}
unsafe impl Sync for SceneRoot {}

unsafe impl Send for MeshRoot {}
unsafe impl Sync for MeshRoot {}

unsafe impl Send for LineCurveRoot {}
unsafe impl Sync for LineCurveRoot {}

unsafe impl Send for GeometryCacheRoot {}
unsafe impl Sync for GeometryCacheRoot {}

unsafe impl Send for AnimRoot {}
unsafe impl Sync for AnimRoot {}

unsafe impl Send for BakedAnimRoot {}
unsafe impl Sync for BakedAnimRoot {}

""".strip()

types = { }
structs = { }
enums = { }
enum_values = { }
functions = { }

file: ir.File = None

alloc_types = {
    "scene": "SceneRoot",
    "mesh": "MeshRoot",
    "line": "LineCurveRoot",
    "geometryCache": "GeometryCacheRoot",
    "anim": "AnimRoot",
    "bakedAnim": "BakedAnimRoot",
}

raw_types = {
    "ufbx_vertex_stream",
}

callback_signatures = {
    "ufbx_open_file_cb": "(&str, &OpenFileInfo) -> Option<Stream>",
    "ufbx_close_memory_cb": "(*mut c_void, usize) -> ()",
    "ufbx_progress_cb": "(&Progress) -> ProgressResult",
}

primitive_types = {
    "void": "c_void",
    "char": "u8",
    "int8_t": "i32",
    "uint8_t": "u32",
    "int32_t": "i32",
    "uint32_t": "u32",
    "int64_t": "i64",
    "uint64_t": "u64",
    "float": "f32",
    "double": "f64",
    "size_t": "usize",
    "uintptr_t": "usize",
    "ptrdiff_t": "isize",
    "bool": "bool",
    "ufbx_real": "Real",
    "ufbx_thread_pool_context": "ThreadPoolContext",
    "ufbx_open_file_context": "OpenFileContext",
}

default_derive_types = {
    "ufbx_error_frame",
    "ufbx_error_type",
    "ufbx_error",
    "ufbx_panic",
}

ignore_types = {
    "ufbx_string",
    "ufbx_blob",
}

ignore_non_raw = {
    "ufbx_open_file",
    "ufbx_open_memory",
    "ufbx_open_file_ctx",
    "ufbx_open_memory_ctx",
    "ufbx_default_open_file",
}

force_mut_args = {
    ("ufbx_generate_indices", 0),
}

override_functions = { }
override_member_functions = { }

override_functions["ufbx_find_real_len"] = """
// TODO: Property find functions
"""

override_functions["ufbx_find_int_len"] = """
// TODO: Property find functions
"""

override_functions["ufbx_find_bool_len"] = """
// TODO: Property find functions
"""

override_functions["ufbx_find_vec3_len"] = """
// TODO: Property find functions
"""

override_functions["ufbx_find_string_len"] = """
// TODO: Property find functions
"""

override_functions["ufbx_find_prop_concat"] = """
// TODO: ufbx_find_prop_concat()
"""

override_functions["ufbx_find_shader_prop_len"] = """
pub fn find_shader_prop<'a>(shader: &'a Shader, name: &'a str) -> &'a str {
    let result = unsafe { ufbx_find_shader_prop_len(shader as *const Shader, name.as_ptr(), name.len()) };
    unsafe { result.as_static_ref() }
}
"""

override_functions["ufbx_thread_pool_set_user_ptr"] = """
pub unsafe fn thread_pool_set_user_ptr(ctx: ThreadPoolContext, user_ptr: *mut c_void) {
    ufbx_thread_pool_set_user_ptr(ctx, user_ptr as *mut c_void)
}
"""

override_functions["ufbx_thread_pool_get_user_ptr"] = """
pub unsafe fn thread_pool_get_user_ptr(ctx: ThreadPoolContext) -> *mut c_void {
    ufbx_thread_pool_get_user_ptr(ctx)
}
"""

override_functions["ufbx_evaluate_prop_len"] = """
pub fn evaluate_prop<'a, 'b>(anim: &'a Anim, element: &'a Element, name: &'b str, time: f64) -> ExternalRef<'b, Prop>
    where 'a: 'b
{
    let result = unsafe { ufbx_evaluate_prop_len(anim as *const Anim, element as *const Element, name.as_ptr(), name.len(), time) };
    unsafe { ExternalRef::new(result) }
}
"""

override_functions["ufbx_prepare_prop_overrides"] = """
// TODO: ufbx_prepare_prop_overrides()
"""

override_functions["ufbx_evaluate_props"] = """
pub fn evaluate_props<'a, 'b>(anim: &'a Anim, element: &'a Element, time: f64, buffer: &'b mut [ExternalRef<'b, Prop>]) -> ExternalRef<'b, Props>
    where 'a: 'b
{
    let result = unsafe { ufbx_evaluate_props(anim as *const Anim, element as *const Element, time, buffer.as_ptr() as *mut Prop, buffer.len()) };
    unsafe { ExternalRef::new(result) }
}
"""

override_member_functions["ufbx_find_real_len"] = """
// TODO: find_real()
"""

override_member_functions["ufbx_find_vec3_len"] = """
// TODO: find_vec3()
"""

override_member_functions["ufbx_find_int_len"] = """
// TODO: find_int()
"""

override_member_functions["ufbx_find_bool_len"] = """
// TODO: find_bool()
"""

override_member_functions["ufbx_find_string_len"] = """
// TODO: find_string()
"""

override_member_functions["ufbx_find_shader_prop_len"] = """
pub fn find_shader_prop<'a>(&'a self, name: &'a str) -> &'a str {
    find_shader_prop(self, name)
}
"""

def get_struct_name(st: ir.Struct):
    name = ir.to_pascal(st.short_name)
    if st.is_input or st.is_callback or st.is_interface or st.name in raw_types:
        name = "Raw" + name
    return name

def get_struct_rust_name(st: ir.Struct):
    name = ir.to_pascal(st.short_name)
    return name

def get_enum_name(en: ir.Enum):
    return ir.to_pascal(en.short_name)

def get_field_name(fd: ir.Field):
    name = fd.name
    if name in ("type", "fn"):
        name = name + "_"
    return name

def get_arg_name(arg: ir.Argument):
    name = arg.name
    if name in ("type", "fn"):
        name = name + "_"
    return name

def get_func_name(fn: ir.Function):
    name = fn.short_name
    if fn.is_catch:
        name = name.replace("catch_", "")
    if fn.is_len:
        name = name[:-4]
    return name

def get_global_name(fn: ir.Global):
    name = fn.short_name
    return name

def get_member_func_name(fn: ir.Function, name: str):
    if fn.is_catch:
        name = name.replace("catch_", "")
    if fn.is_len:
        name = name[:-4]
    return name

class RustType:
    def __init__(self, irt: Optional[ir.Type], inner: Optional["RustType"]):
        self.ir = irt
        self.needs_lifetime = False
        self.rust_needs_lifetime = False
        self.is_list = False
        self.is_ref_list = False
        self.is_result = False
        self.is_void = False
        self.is_synthetic = False
        self.is_function = False
        self.is_raw = False
        self.is_string = False
        self.kind = ""
        self.inner = inner
        if irt:
            self.is_function = irt.is_function
            self.kind = irt.kind
            if irt.kind == "struct":
                st = file.structs[irt.base_name]
                self.name = get_struct_name(st)
                self.rust_name = get_struct_rust_name(st)
                if st.is_list:
                    self.is_list = True
                    data_type = file.types[file.types[st.fields[0].type].inner]
                    if data_type.kind == "pointer":
                        self.is_ref_list = True
                        data_type = file.types[data_type.inner]
                    self.inner = init_type(data_type)
                if st.name == "ufbx_string":
                    self.is_string = True
            elif irt.kind == "enum":
                en = file.enums[irt.base_name]
                self.name = get_enum_name(en)
                self.rust_name = self.name
            elif irt.key in primitive_types:
                self.name = primitive_types[irt.key]
                self.rust_name = self.name
                if irt.key == "void":
                    self.is_void = True
            else:
                self.name = "???"
                self.rust_name = self.name

    def get_leaf(self):
        typ = self
        while typ.inner:
            typ = typ.inner
        return typ

    def fmt_raw(self):
        if self.is_result:
            return f"Result<{self.inner.fmt_raw()}>"
        elif self.is_function:
            typ = self.ir
            if typ.kind == "pointer":
                typ = file.types[typ.inner]
            if typ.kind == "typedef":
                typ = file.types[typ.inner]

            ret_type = types[typ.inner]
            arg_types = [types[arg.type] for arg in typ.func_args]
            arg_str = ", ".join(arg.fmt_raw() for arg in arg_types)
            if ret_type.ir and ret_type.ir.key == "void":
                return f"Option<unsafe extern \"C\" fn ({arg_str})>"
            else:
                ret_str = ret_type.fmt_raw()
                return f"Option<unsafe extern \"C\" fn ({arg_str}) -> {ret_str}>"
        elif self.kind == "pointer":
            if self.ir.is_const:
                return f"*const {self.inner.fmt_raw()}"
            else:
                return f"*mut {self.inner.fmt_raw()}"
        elif self.kind == "unsafe":
            return self.inner.fmt_raw()
        else:
            return self.name

    def fmt_member(self, lifetime=""):
        if self.is_result:
            return f"Result<{self.inner.fmt_member(lifetime)}>"
        elif self.is_function:
            return self.fmt_raw()
        elif self.is_synthetic and self.name == "RawList":
            return f"RawList<{self.inner.fmt_member(lifetime)}>"
        elif self.is_list:
            list_type = "RefList" if self.is_ref_list else "List"
            lt = f"'{lifetime}, " if lifetime else ""
            return f"{list_type}<{self.inner.fmt_member(lifetime)}>"
        elif self.kind == "array":
            num = self.ir.array_length
            return f"[{self.inner.fmt_member(lifetime)}; {num}]"
        elif self.kind == "unsafe":
            return self.inner.fmt_member(lifetime)
        elif self.kind == "pointer":
            lt = f"'{lifetime}, " if lifetime else ""
            if self.ir.inner == "void":
                return self.fmt_raw()
            if self.ir.is_nullable:
                return f"Option<Ref<{self.inner.fmt_member(lifetime)}>>"
            else:
                return f"Ref<{self.inner.fmt_member(lifetime)}>"
        else:
            if self.needs_lifetime:
                lt = f"<'{lifetime}>" if lifetime else ""
                return f"{self.name}{lt}"
            else:
                return self.name

    def fmt_arg(self, lifetime="", force_const=False, non_raw=False):
        if self.is_result:
            return f"Result<{self.inner.fmt_arg(lifetime)}>"
        elif self.is_function:
            return self.fmt_raw()
        elif self.is_list:
            lt = f"'{lifetime} " if lifetime else ""
            return f"&{lt}[{self.inner.fmt_arg(lifetime)}]"
        elif self.kind == "array":
            num = self.ir.array_length
            return f"[{self.inner.fmt_member(lifetime)}; {num}]"
        elif self.kind == "pointer":
            if self.ir.inner == "void":
                return self.fmt_raw()
            lt = f"'{lifetime} " if lifetime else ""
            mut = "" if self.ir.is_const or force_const else "mut "
            if self.ir.is_nullable:
                return f"Option<&{lt}{mut}{self.inner.fmt_arg(lifetime)}>"
            else:
                return f"&{lt}{mut}{self.inner.fmt_arg(lifetime)}"
        elif self.is_string:
            lt = f"'{lifetime} " if lifetime else ""
            return f"&{lt}str"
        else:
            if self.needs_lifetime:
                lt = f"<'{lifetime}>" if lifetime else ""
                return f"{self.name}{lt}"
            elif self.is_raw and non_raw:
                return self.rust_name
            else:
                return self.name

    def fmt_input(self, lifetime=""):
        if self.is_result:
            return f"Result<{self.inner.fmt_input(lifetime)}>"
        elif self.is_function:
            return self.fmt_raw()
        elif self.is_list:
            return f"Vec<{self.inner.fmt_input(lifetime)}>"
        elif self.is_synthetic and self.name == "RawString":
            assert lifetime
            return f"StringOpt<'{lifetime}>"
        elif self.is_synthetic and self.name == "RawBlob":
            assert lifetime
            return f"BlobOpt<'{lifetime}>"
        elif self.is_synthetic and self.name == "RawList":
            assert lifetime
            return f"ListOpt<'{lifetime}, {self.inner.fmt_input(lifetime)}>"
        elif self.kind == "array":
            num = self.ir.array_length
            return f"[{self.inner.fmt_input(lifetime)}; {num}]"
        elif self.kind == "pointer":
            lt = f"'{lifetime}, " if lifetime else ""
            if self.ir.inner == "void":
                return self.fmt_raw()
            if self.ir.is_nullable:
                return f"OptionRef<{self.inner.fmt_input(lifetime)}>"
            else:
                return f"Ref<{self.inner.fmt_input(lifetime)}>"
        elif self.kind == "unsafe":
            return f"Unsafe<{self.inner.fmt_input(lifetime)}>"
        elif self.kind == "struct":
            rs = structs[self.ir.key]
            if rs.ir.is_input or rs.ir.is_interface:
                lt = f"'{lifetime}" if lifetime and self.rust_needs_lifetime else ""
                return f"{rs.rust_name}<{lt}>"
            elif rs.ir.is_callback:
                assert lifetime
                lt = f"'{lifetime}"
                return f"{rs.rust_name}<{lt}>"
            elif rs.ir.name == "ufbx_string":
                lt = f"'{lifetime} " if lifetime else ""
                return f"&{lt}str"
            elif self.needs_lifetime:
                lt = f"<'{lifetime}>" if lifetime else ""
                return f"{self.name}{lt}"
            else:
                return self.name
        else:
            if self.needs_lifetime:
                lt = f"<'{lifetime}>" if lifetime else ""
                return f"{self.name}{lt}"
            else:
                return self.name

    def fmt_raw_default(self):
        if self.is_function:
            return "None"
        elif self.kind == "pointer":
            if self.ir.inner == "void":
                return f"ptr::null::<c_void>() as *mut c_void"
        raise RuntimeError(f"No default for {self.name}")

class RustField:
    def __init__(self, irf: ir.Field, rt: RustType):
        self.ir = irf
        self.name = get_field_name(irf)
        self.type = rt

class RustStruct:
    fields: List[RustField]
    def __init__(self, st: ir.Struct):
        self.ir = st
        self.fields = []
        self.name = get_struct_name(st)
        self.rust_name = get_struct_rust_name(st)
        self.is_raw = False
        self.has_inline_bufs = False

class RustEnumValue:
    def __init__(self, ev: ir.EnumValue):
        self.ir = ev
        if ev.flag:
            self.name = ev.short_name
        else:
            self.name = ir.to_pascal(ev.short_name)
        self.value = ev.value

class RustEnum:
    values: List[RustEnumValue]
    def __init__(self, en: ir.Enum):
        self.ir = en
        self.name = get_enum_name(en)
        self.values = []

class RustArgument:
    def __init__(self, arg: ir.Argument, kind: str, original_index: int):
        self.ir = arg
        self.num_ir = None
        self.name = get_arg_name(arg)
        self.type = init_type(file.types[arg.type])
        self.is_const = self.type.ir.is_const
        self.kind = kind
        leaf = self.type.get_leaf()
        self.is_raw = leaf.is_raw
        self.original_index = original_index

    def fmt_arg(self, lifetime: str, non_raw: bool = False, force_mut: bool = False) -> str:
        if not self.ir.return_ref:
            lifetime = ""
        if self.kind == "string":
            return f"{self.name}: &str"
        elif self.kind == "blob":
            mut = "" if self.is_const else "mut "
            return f"{self.name}: &{mut}[u8]"
        elif self.kind == "slice":
            mut = "" if self.is_const and not force_mut else "mut "
            return f"{self.name}: &{mut}[{self.type.fmt_arg(lifetime, non_raw=non_raw)}]"
        elif non_raw and self.is_raw:
            leaf = self.type.get_leaf()
            return f"{self.name}: {leaf.rust_name}"
        else:
            return f"{self.name}: {self.type.fmt_arg(lifetime)}"

class RustFunction:
    args: List[RustArgument]
    def __init__(self, fn: ir.Function):
        self.ir = fn
        self.name = get_func_name(fn)
        self.args = []
        self.is_raw = False
        self.emitted = False

        if fn.alloc_type:
            name = alloc_types[fn.alloc_type]
            self.return_type = RustType(None, None)
            self.return_type.name = name
            self.return_type.is_synthetic = True
        else:
            self.return_type = init_type(file.types[fn.return_type])

        if fn.has_error:
            self.return_type = RustType(None, self.return_type)
            self.return_type.is_result = True

lifetime_types = set()

def init_type(typ: ir.Type) -> RustType:
    if typ.key not in types:
        inner = None
        inner_lifetime = False
        rust_inner_lifetime = False
        if typ.inner:
            inner = init_type(file.types[typ.inner])
            if inner.needs_lifetime:
                inner_lifetime = True
            if inner.rust_needs_lifetime:
                rust_inner_lifetime = True
        rt = RustType(typ, inner)
        types[typ.key] = rt

        if inner_lifetime:
            rt.needs_lifetime = True
        if rust_inner_lifetime:
            rt.rust_needs_lifetime = True
        if typ.key in lifetime_types:
            rt.needs_lifetime = True

    return types[typ.key]

def propagate_lifetimes():
    updated = True
    while updated:
        updated = False
        for rt in types.values():
            if rt.kind != "struct": continue
            if not rt.needs_lifetime:
                rs = structs[rt.ir.key]
                for field in rs.fields:
                    if field.type.needs_lifetime:
                        rt.needs_lifetime = True
                        updated = True
            if not rt.rust_needs_lifetime:
                rs = structs[rt.ir.key]
                for field in rs.fields:
                    if field.type.rust_needs_lifetime:
                        rt.rust_needs_lifetime = True
                        updated = True

def init_fields(rs: RustStruct, field: ir.Field):
    if field.name == "":
        ist = file.structs[field.type]
        if ist.is_union:
            for ifield in ist.fields:
                if not ifield.union_sized: continue
                if not ifield.union_preferred: continue
                init_fields(rs, ifield)
                break
            else:
                for ifield in ist.fields:
                    if not ifield.union_sized: continue
                    init_fields(rs, ifield)
                    break
                else:
                    raise RuntimeError(f"Could not choose union alternative for {ist.name}")
        else:
            for ifield in ist.fields:
                init_fields(rs, ifield)
        return

    rt = init_type(file.types[field.type])

    if rs.ir.is_input and rt.ir.key == "ufbx_string":
        rt = RustType(None, None)
        rt.name = "RawString"
        rt.is_synthetic = True
        rt.rust_needs_lifetime = True
    elif rs.ir.is_input and rt.ir.key == "ufbx_blob":
        rt = RustType(None, None)
        rt.name = "RawBlob"
        rt.is_synthetic = True
        rt.rust_needs_lifetime = True
    elif rs.ir.is_input and rt.ir.kind == "struct" and file.structs[rt.ir.key].is_list:
        rt = RustType(None, rt.inner)
        rt.name = "RawList"
        rt.is_synthetic = True
        rt.rust_needs_lifetime = True
    elif field.kind == "inlineBuf":
        rt = RustType(None, rt)
        rt.name = f"InlineBuf<[{rt.inner.inner.name}; {rt.inner.ir.array_length}]>"
        rt.is_synthetic = True
        field.name = f"{field.name}_buf"
        rs.has_inline_bufs = True

    rs.fields.append(RustField(field, rt))

def init_struct(st: ir.Struct):
    rs = RustStruct(st)
    for field in st.fields:
        init_fields(rs, field)
    structs[st.name] = rs

    if rs.ir.is_callback or rs.ir.is_input or rs.ir.is_interface or rs.ir.name in raw_types:
        rs.is_raw = True
        types[rs.ir.name].is_raw = True

    return rs

def init_enum(en: ir.Enum):
    re = RustEnum(en)
    enums[en.name] = re

    #HACK
    seen_values = set()

    for val in en.values:
        ev = file.enum_values[val]
        if ev.value in seen_values: continue
        seen_values.add(ev.value)
        rv = RustEnumValue(ev)
        re.values.append(rv)
        enum_values[val] = rv
    return en

def init_function(fn: ir.Function):
    rf = RustFunction(fn)
    functions[fn.name] = rf
    for arg_ix, arg in enumerate(fn.arguments):
        if arg.kind == "stringPointer":
            rf.args.append(RustArgument(arg, "string", arg_ix))
        elif arg.kind == "stringLength":
            pass
        elif arg.kind == "arrayPointer":
            ra = RustArgument(arg, "slice", arg_ix)
            ra.type = ra.type.inner
            rf.args.append(ra)
        elif arg.kind == "arrayLength":
            pass
        elif arg.kind == "blobPointer":
            rf.args.append(RustArgument(arg, "blob", arg_ix))
        elif arg.kind == "blobSize":
            pass
        elif arg.kind == "error":
            pass
        elif arg.kind == "panic":
            pass
        else:
            arg_type = types[arg.type]
            while arg_type.inner:
                arg_type = arg_type.inner
            if arg_type.is_raw:
                rf.is_raw = True
            rf.args.append(RustArgument(arg, arg.kind, arg_ix))

def init_file():
    for name in file.types:
        init_type(file.types[name])
    for name in file.structs:
        init_struct(file.structs[name])

    propagate_lifetimes()

    for name in file.enums:
        init_enum(file.enums[name])
    for name in file.functions:
        init_function(file.functions[name])

g_outfile = None
g_indent = 0

def emit(line=""):
    global g_indent
    global g_outfile
    if line:
        print("    " * g_indent + line, file=g_outfile)
    else:
        print("", file=g_outfile)

def indent(delta=1):
    global g_indent
    g_indent += delta

def unindent(delta=1):
    global g_indent
    g_indent -= delta

def emit_lines(extra: str):
    for line in extra.splitlines():
        emit(line)

def emit_struct(rs: RustStruct):
    if rs.ir.is_list: return

    lifetime = ""
    typ = types[rs.ir.name]
    if typ.needs_lifetime:
        lifetime = "<'a>"

    emit()
    emit(f"#[repr(C)]")
    if rs.ir.is_pod:
        emit(f"#[derive(Clone, Copy)]")
    if rs.ir.name in default_derive_types or rs.ir.is_pod or rs.ir.is_input:
        emit(f"#[derive(Default)]")
    if rs.ir.is_pod:
        emit(f"#[derive(Debug)]")
    emit(f"pub struct {rs.name}{lifetime} {{")
    indent()

    for field in rs.fields:
        prefix = ""
        if (not field.ir.private and field.ir.kind not in ("inlineBuf", "inlineBufLength")) or rs.ir.is_input:
            prefix = "pub "
        lifetime = "a"

        emit(f"{prefix}{field.name}: {field.type.fmt_member(lifetime)},")

    unindent()
    emit("}")

    if rs.ir.is_callback or rs.ir.is_interface:
        emit()
        emit(f"impl Default for {rs.name} {{")
        indent()
        emit("fn default() -> Self {")
        indent()
        emit(f"{rs.name} {{")
        indent()
        for field in rs.fields:
            emit(f"{field.name}: {field.type.fmt_raw_default()},")
        unindent()
        emit("}")
        unindent()
        emit("}")
        unindent()
        emit("}")

    if rs.ir.vertex_attrib_type:
        attrib_type = types[rs.ir.vertex_attrib_type]
        emit()
        emit(f"impl Index<usize> for {rs.name} {{")
        indent()
        emit(f"type Output = {attrib_type.name};")
        emit(f"fn index(&self, index: usize) -> &{attrib_type.name} {{")
        indent()
        emit(f"&self.values[self.indices[index] as usize]")
        unindent()
        emit("}")
        unindent()
        emit("}")

    if rs.has_inline_bufs:
        emit()
        emit(f"impl {rs.name} {{")
        indent()
        for field in rs.fields:
            if field.ir.kind != "inlineBuf": continue
            assert field.name.endswith("_buf")
            irt = field.type.inner.inner
            n = field.type.inner.ir.array_length
            base_name = field.name[:-4]
            len_name = ""
            for len_field in rs.fields:
                if len_field.ir.kind == "inlineBufLength" and len_field.name.startswith(base_name):
                    len_name = len_field.name
                    break
            emit(f"pub fn {base_name}(&self) -> &str {{")
            indent()
            emit("unsafe {")
            indent()
            emit(f"let buf: &[mem::MaybeUninit<{irt.name}>; {n}] = mem::transmute(&self.{base_name}_buf);")
            emit(f"str::from_utf8(mem::transmute(&buf[..self.{len_name}])).unwrap()")
            unindent()
            emit("}")
            unindent()
            emit("}")
        unindent()
        emit("}")

def emit_input_callback(rs: RustStruct):
    sig = callback_signatures[rs.ir.name]

    emit()
    emit(f"pub enum {rs.rust_name}<'a> {{")
    indent()
    emit("Unset,")
    emit(f"Mut(&'a mut dyn FnMut{sig}),")
    emit(f"Ref(&'a dyn Fn{sig}),")
    emit(f"Raw(Unsafe<{rs.name}>),")
    unindent()
    emit("}")

    emit()
    emit(f"impl<'a> Default for {rs.rust_name}<'a> {{")
    indent()
    emit("fn default() -> Self { Self::Unset }")
    unindent()
    emit("}")

    emit()
    emit(f"impl {rs.name} {{")
    indent()

    emit(f"fn from_func<F: FnMut{sig}>(arg: &mut F) -> Self {{")
    indent()
    emit(f"{rs.name} {{")
    indent()
    emit(f"fn_: Some(call_{rs.ir.short_name}::<F>),")
    emit(f"user: arg as *mut F as *mut c_void,")
    unindent()
    emit("}")
    unindent()
    emit("}")

    unindent()
    emit("}")

    emit()
    emit(f"impl {rs.rust_name}<'_> {{")
    indent()

    emit()
    emit(f"fn from_rust(&self) -> {rs.name} {{")
    indent()
    emit("match self {")
    indent()
    emit(f"{rs.rust_name}::Unset => Default::default(),")
    emit(f"_ => panic!(\"required mutable\"),")
    unindent()
    emit("}")
    unindent()
    emit("}")

    emit()
    emit(f"fn from_rust_mut(&mut self) -> {rs.name} {{")
    indent()
    emit("match self {")
    indent()
    emit(f"{rs.rust_name}::Unset => Default::default(),")
    emit(f"{rs.rust_name}::Ref(f) => {rs.name}::from_func(f),")
    emit(f"{rs.rust_name}::Mut(f) => {rs.name}::from_func(f),")
    emit(f"{rs.rust_name}::Raw(raw) => raw.take(),")
    unindent()
    emit("}")
    unindent()
    emit("}")

    unindent()
    emit("}")

def emit_input_struct(rs: RustStruct):
    if rs.ir.is_callback:
        emit_input_callback(rs)
    if not rs.ir.is_input: return

    typ = types[rs.ir.name]
    needs_lifetime = typ.needs_lifetime

    for field in rs.fields:
        if field.ir.private: continue
        if field.type.kind == "struct":
            frs = structs[field.type.ir.key]
            if frs.ir.is_callback or frs.ir.name in ("ufbx_string", "ufbx_blob") or frs.ir.is_list:
                needs_lifetime = True
        elif field.type.rust_needs_lifetime:
            needs_lifetime = True

    lifetime = ""
    lifetime_a = ""
    lt_a = ""
    typ = types[rs.ir.name]
    if needs_lifetime:
        lifetime = "a"
        lifetime_a = "<'a>"
        lt_a = "'a "

    emit()
    emit(f"#[derive(Default)]")
    emit(f"pub struct {rs.rust_name}{lifetime_a} {{")
    indent()

    for field in rs.fields:
        if field.ir.private: continue
        prefix = "pub "
        emit(f"{prefix}{field.name}: {field.type.fmt_input(lifetime)},")

    unindent()
    emit("}")

    emit()
    emit(f"impl{lifetime_a} FromRust for {rs.rust_name}{lifetime_a} {{")
    indent()

    emit(f"type Result = {rs.name};")

    for mut in ("", "mut "):
        mut_us = "_mut" if mut  else ""
        emit(f"#[allow(unused, unused_variables, dead_code)]")
        emit(f"fn from_rust{mut_us}(&{mut}self, arena: &mut Arena) -> Self::Result {{")
        indent()
        emit(f"{rs.name} {{")
        indent()

        for field in rs.fields:
            if field.ir.private:
                emit(f"{field.name}: 0,")
                continue

            has_from = False
            has_arena = False
            if field.type.kind == "struct":
                frs = structs[field.type.ir.key]
                if frs.ir.is_callback or frs.ir.is_input or frs.ir.is_interface or frs.ir.is_list:
                    has_from = True
                    if frs.ir.is_input or frs.ir.is_list:
                        has_arena = True
            elif field.type.name in ("RawString", "RawBlob", "RawList"):
                has_from = True
                has_arena = True

            if has_from:
                if has_arena:
                    emit(f"{field.name}: self.{field.name}.from_rust{mut_us}(arena),")
                else:
                    emit(f"{field.name}: self.{field.name}.from_rust{mut_us}(),")
            elif field.type.kind == "unsafe":
                if mut:
                    emit(f"{field.name}: self.{field.name}.take(),")
                else:
                    emit(f"{field.name}: panic!(\"required mutable\"),")
            else:
                emit(f"{field.name}: self.{field.name},")

        unindent()
        emit("}")
        unindent()
        emit("}")

    unindent()
    emit("}")

def emit_flag(re: RustEnum):
    emit()
    emit(f"#[repr(transparent)]")
    emit(f"#[derive(Clone, Copy)]")
    emit(f"pub struct {re.name}(u32);")
    emit(f"impl {re.name} {{")
    indent()
    emit(f"pub const NONE: {re.name} = {re.name}(0);")
    num_values = 0
    for value in re.values:
        if value.ir.auxiliary: continue
        emit(f"pub const {value.name}: {re.name} = {re.name}(0x{value.value:x});")
        num_values += 1
    unindent()
    emit("}")

    emit()
    names_name = f"{re.name.upper()}_NAMES"
    emit(f"const {names_name}: [(&'static str, u32); {num_values}] = [")
    indent()
    for value in re.values:
        if value.ir.auxiliary: continue
        emit(f"(\"{value.name}\", 0x{value.value:x}),")
    unindent()
    emit("];")

    emit()
    emit(f"impl {re.name} {{")
    indent()
    emit("pub fn any(self) -> bool { self.0 != 0 }")
    emit("pub fn has_any(self, bits: Self) -> bool { (self.0 & bits.0) != 0 }")
    emit("pub fn has_all(self, bits: Self) -> bool { (self.0 & bits.0) == bits.0 }")
    unindent()
    emit("}")

    emit(f"impl Default for {re.name} {{")
    indent()
    emit(f"fn default() -> Self {{ Self(0) }}")
    unindent()
    emit("}")

    emit(f"impl Debug for {re.name} {{")
    indent()
    emit("fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {")
    indent()
    emit(f"format_flags(f, &{names_name}, self.0)")
    unindent()
    emit("}")
    unindent()
    emit("}")

    ops = [
        ("BitAnd", ["type Output = Self;", "fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) }"]),
        ("BitAndAssign", ["fn bitand_assign(&mut self, rhs: Self) { *self = Self(self.0 & rhs.0) }"]),
        ("BitOr", ["type Output = Self;", "fn bitor(self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) }"]),
        ("BitOrAssign", ["fn bitor_assign(&mut self, rhs: Self) { *self = Self(self.0 | rhs.0) }"]),
        ("BitXor", ["type Output = Self;", "fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) }"]),
        ("BitXorAssign", ["fn bitxor_assign(&mut self, rhs: Self) { *self = Self(self.0 ^ rhs.0) }"]),
    ]

    for op, impl in ops:
        emit(f"impl {op} for {re.name} {{")
        indent()
        for line in impl:
            emit(line)
        unindent()
        emit("}")


def emit_enum(re: RustEnum):
    if re.ir.flag:
        emit_flag(re)
        return

    emit()
    emit(f"#[repr(u32)]")
    emit(f"#[derive(Clone, Copy, PartialEq, Eq, Debug)]")
    emit(f"pub enum {re.name} {{")
    indent()

    for value in re.values:
        if value.ir.auxiliary: continue
        emit(f"{value.name} = {value.value},")

    unindent()
    emit("}")

    emit()
    emit(f"impl Default for {re.name} {{")
    indent()
    emit(f"fn default() -> Self {{ Self::{re.values[0].name} }}")
    unindent()
    emit("}")

def fmt_ffi_type(typ: ir.Type, lifetime: str):
    if typ.kind == "pointer":
        inner = file.types[typ.inner]
        mut = "const " if typ.is_const else "mut "
        return f"*{mut}{fmt_ffi_type(inner, lifetime)}"
    elif typ.key == "ufbx_string":
        return f"String"
    elif types[typ.key].is_list:
        rtyp = types[typ.key]
        list_type = "RefList" if rtyp.is_ref_list else "List"
        return f"{list_type}<{rtyp.inner.fmt_member(lifetime)}>"
    elif typ.key in primitive_types:
        return primitive_types[typ.key]
    else:
        rt = types[typ.key]
        return rt.fmt_arg(lifetime)

def fmt_ffi_arg(arg: ir.Argument, lifetime: str):
    typ = file.types[arg.type]
    name = arg.name
    if name == "type":
        name = "type_"
    if not arg.return_ref:
        lifetime = ""
    return f"{name}: {fmt_ffi_type(typ, lifetime)}"

def emit_ffi_function(fn: ir.Function):
    if fn.is_inline: return

    needs_ref = False

    lt = "<'a>" if needs_ref else ""
    lifetime = "a" if needs_ref else ""

    args = ", ".join(fmt_ffi_arg(arg, lifetime) for arg in fn.arguments)
    if fn.return_type == "void":
        emit(f"pub fn {fn.name}{lt}({args});")
    else:
        ret = fmt_ffi_type(file.types[fn.return_type], lifetime)
        emit(f"pub fn {fn.name}{lt}({args}) -> {ret};")

def emit_ffi_global(gl: ir.Global):
    typ = file.types[gl.type]
    if typ.kind != "const":
        return
    typ = file.types[typ.inner]
    gt = fmt_ffi_type(typ, "")
    emit(f"pub static {gl.name}: {gt};")

def emit_arg_pass(args: List[str], ra: RustArgument):
    if ra.kind == "string":
        if ra.is_const:
            args.append(f"{ra.name}.as_ptr()")
        else:
            args.append(f"{ra.name}.as_mut_ptr()")
        args.append(f"{ra.name}.len()")
    elif ra.kind == "slice":
        if ra.is_const:
            args.append(f"{ra.name}.as_ptr()")
        else:
            args.append(f"{ra.name}.as_mut_ptr()")
        args.append(f"{ra.name}.len()")
    elif ra.kind == "blob":
        if ra.is_const:
            args.append(f"{ra.name}.as_ptr() as *const c_void")
        else:
            args.append(f"{ra.name}.as_mut_ptr() as *mut c_void")
        args.append(f"{ra.name}.len()")
    elif ra.type.ir.kind == "pointer":
        args.append(f"{ra.name} as {ra.type.fmt_raw()}")
    elif ra.type.is_list:
        args.append(f"List::from_slice({ra.name})")
    else:
        args.append(ra.name)

def emit_function(rf: RustFunction, non_raw: bool = False):
    if rf.ir.is_inline: return
    if rf.ir.is_ffi: return
    if rf.ir.catch_name: return
    if rf.ir.len_name: return
    if rf.ir.kind in { "retain", "free" }: return
    rf.emitted = True

    if rf.ir.name in override_functions:
        emit()
        emit_lines(override_functions[rf.ir.name].strip())
        return

    is_raw = rf.is_raw and not non_raw

    needs_ref = False
    if rf.return_type.kind == "pointer":
        needs_ref = True
    elif rf.return_type.is_string or rf.return_type.is_list:
        needs_ref = True

    lt = "<'a>" if needs_ref else ""
    lifetime = "a" if needs_ref else ""

    arg_str = ", ".join(
        arg.fmt_arg(lifetime, non_raw,
            force_mut = non_raw and (rf.ir.name, ix) in force_mut_args
        ) for ix, arg in enumerate(rf.args))

    arg_pass = []
    for arg in rf.args:
        emit_arg_pass(arg_pass, arg)

    ret = ""
    if not rf.return_type.is_void:
        rt = rf.return_type.fmt_arg(lifetime, force_const=True)
        if rf.ir.nullable_return:
            rt = f"Option<{rt}>"
        ret = f" -> {rt}"

    is_unsafe = False

    emit()
    if is_raw:
        emit(f"pub unsafe fn {rf.name}_raw{lt}({arg_str}){ret} {{")
        is_unsafe = True
    else:
        unsafe_fn = ""
        if rf.ir.is_unsafe:
            unsafe_fn = "unsafe "
            is_unsafe = True
        emit(f"pub {unsafe_fn}fn {rf.name}{lt}({arg_str}){ret} {{")
    indent()

    unsafe = "" if is_unsafe else "unsafe "

    if non_raw:
        has_arena = False
        for arg in rf.args:
            if arg.is_raw:
                use_arena = True
                use_mut = True
                leaf = arg.type.get_leaf()
                if leaf and leaf.ir and leaf.ir.kind == "struct":
                    rs = file.structs[leaf.ir.key]
                    if rs.is_interface:
                        use_arena = False
                if arg.kind == "slice":
                    use_mut = False
                if use_arena:
                    if not has_arena:
                        has_arena = True
                        emit(f"let mut arena = Arena::new();")
                    if use_mut:
                        emit(f"let mut {arg.name}_mut = {arg.name};")
                        emit(f"let {arg.name}_raw = {arg.name}_mut.from_rust_mut(&mut arena);")
                    else:
                        emit(f"let {arg.name}_raw = {arg.name}.from_rust_mut(&mut arena);")
                else:
                    if use_mut:
                        emit(f"let mut {arg.name}_mut = {arg.name};")
                        emit(f"let {arg.name}_raw = {arg.name}_mut.from_rust_mut();")
                    else:
                        emit(f"let {arg.name}_raw = {arg.name}.from_rust_mut();")
        params = []
        for arg in rf.args:
            if arg.is_raw:
                mut = "" if arg.is_const else "mut "
                params.append(f"&{mut}{arg.name}_raw")
            else:
                params.append(arg.name)
        params_str = ", ".join(params)
        emit(f"{unsafe}{{ {rf.name}_raw({params_str}) }}")
    else:
        if rf.ir.has_error:
            emit(f"let mut error: Error = Error::default();")
            arg_pass.append("&mut error")
        if rf.ir.has_panic:
            emit(f"let mut panic: Panic = Default::default();")
            arg_pass.insert(0, "&mut panic")

        arg_pass_str = ", ".join(arg_pass)
        if not rf.return_type.is_void:
            emit(f"let result = {unsafe}{{ {rf.ir.name}({arg_pass_str}) }};")
        else:
            emit(f"{unsafe}{{ {rf.ir.name}({arg_pass_str}) }};")

        if rf.ir.has_panic:
            emit(f"if panic.did_panic {{")
            indent()
            emit(f"panic!(\"ufbx::{rf.name}() {{}}\", panic.message());")
            unindent()
            emit("}")
        if rf.ir.has_error:
            emit(f"if error.type_ != ErrorType::None {{")
            indent()
            emit(f"return Err(error)")
            unindent()
            emit("}")

        if not rf.return_type.is_void:
            res = "result"
            if rf.ir.alloc_type:
                alloc_type = alloc_types[rf.ir.alloc_type]
                res = f"{alloc_type}::new({res})"
            elif rf.return_type.is_list:
                res = f"{unsafe}{{ {res}.as_static_ref() }}"
            elif rf.return_type.kind == "pointer":
                if rf.ir.nullable_return:
                    res = f"if result.is_null() {{ None }} else {{ {unsafe}{{ Some(&*{res}) }} }}"
                else:
                    res = f"{unsafe}{{ &*{res} }}"
            if rf.ir.has_error:
                res = f"Ok({res})"

            emit(res)

    unindent()
    emit("}")

    if rf.is_raw and not non_raw:
        if rf.ir.name not in ignore_non_raw:
            emit_function(rf, non_raw=True)

def emit_global(gl: ir.Global):
    typ = file.types[gl.type]
    if typ.kind != "const": return
    typ = file.types[typ.inner]
    if typ.base_name in ("ufbx_string", "ufbx_blob"): return

    gt = fmt_ffi_type(typ, "")
    name = get_global_name(gl)
    emit(f"pub fn {name}() -> {gt} {{ unsafe {{ {gl.name} }} }}")

def emit_struct_impl(rs: RustStruct):
    if not rs.ir: return
    if not rs.ir.member_functions and not rs.ir.member_globals: return

    members = []
    member_globals = []
    for name in rs.ir.member_functions:
        rf = functions[name]
        if rf.emitted:
            members.append((file.member_functions[name], rf))

    for name in rs.ir.member_globals:
        mg = file.member_globals[name]
        gl = file.globals[name]
        typ = file.types[gl.type]
        if typ.kind != "const": continue
        typ = file.types[typ.inner]
        if typ.base_name in ("ufbx_string", "ufbx_blob"): continue
        member_globals.append((mg, gl, typ))

    if not members and not member_globals: return

    emit()
    emit(f"impl {rs.name} {{")
    indent()

    for mg, gl, typ in member_globals:
        gt = fmt_ffi_type(typ, "")
        name = mg.member_name
        emit(f"pub fn {name}() -> {gt} {{ unsafe {{ {gl.name} }} }}")

    for mf, rf in members:
        if mf.func in override_member_functions:
            emit()
            emit_lines(override_member_functions[mf.func].strip())
            continue

        func = file.functions[mf.func]
        name = get_member_func_name(func, mf.member_name)

        non_raw = rf.is_raw

        needs_ref = False
        if rf.return_type.kind == "pointer":
            needs_ref = True
        elif rf.return_type.is_string or rf.return_type.is_list:
            needs_ref = True

        lt = "<'a>" if needs_ref else ""
        lifetime = "a" if needs_ref else ""
        self_lt = "'a " if needs_ref else ""

        args = [arg for arg in rf.args if arg.original_index != mf.self_index]
        arg_fmt = [arg.fmt_arg(lifetime, non_raw) for arg in args]
        arg_fmt.insert(0, f"&{self_lt}self")
        arg_str = ", ".join(arg_fmt)

        ret = ""
        if not rf.return_type.is_void:
            rt = rf.return_type.fmt_arg(lifetime, force_const=True)
            if rf.ir.nullable_return:
                rt = f"Option<{rt}>"
            ret = f" -> {rt}"

        emit()
        emit(f"pub fn {name}{lt}({arg_str}){ret} {{")
        indent()

        pass_args = []
        for arg in rf.args:
            if arg.original_index == mf.self_index:
                pass_args.append("&self")
            else:
                pass_args.append(arg.name)
        pass_str = ", ".join(pass_args)
        emit(f"{rf.name}({pass_str})")

        unindent()
        emit("}")

    unindent()
    emit("}")

def emit_element_data():
    emit()
    emit("pub enum ElementData<'a> {")
    indent()

    for name in file.element_types:
        typ = types[name]
        emit(f"{typ.name}(&'a {typ.name}),")

    unindent()
    emit("}")

    emit()
    emit("impl Element {")
    indent()
    emit("pub fn as_data(&self) -> ElementData<'_> {")
    indent()
    emit("unsafe {")
    indent()
    emit("match self.type_ {")
    indent()

    for name in file.element_types:
        typ = types[name]
        emit(f"ElementType::{typ.name} => ElementData::{typ.name}(&*(self as *const _ as *const {typ.name})),")

    unindent()
    emit("}")
    unindent()
    emit("}")
    unindent()
    emit("}")
    unindent()
    emit("}")

def emit_file():
    emit_lines(uses)

    rust_uses = []
    for rs in structs.values():
        if rs.ir.is_interface:
            rust_uses.append(rs.rust_name)
        if rs.ir.is_callback:
            rust_uses.append(f"call_{rs.ir.short_name}")
    rust_uses_str = ", ".join(rust_uses)
    emit(f"use crate::prelude::{{{rust_uses_str}}};")

    for decl in file.declarations:
        if decl.name in ignore_types: continue
        if decl.kind == "struct":
            emit_struct(structs[decl.name])
        elif decl.kind == "enum":
            emit_enum(enums[decl.name])

    for decl in file.declarations:
        if decl.name in ignore_types: continue
        if decl.kind == "struct":
            emit_input_struct(structs[decl.name])

    emit()
    emit("pub type Result<T> = result::Result<T, Error>;")

    emit()
    emit(f"#[link(name=\"ufbx\")]")
    emit("extern \"C\" {")
    indent()

    for decl in file.declarations:
        if decl.kind == "function":
            emit_ffi_function(file.functions[decl.name])
        elif decl.kind == "global":
            emit_ffi_global(file.globals[decl.name])

    unindent()
    emit("}")

    emit_lines(post_ffi)

    for decl in file.declarations:
        if decl.kind == "function":
            emit_function(functions[decl.name])

    for decl in file.declarations:
        if decl.kind == "global":
            emit_global(file.globals[decl.name])

    for decl in file.declarations:
        if decl.kind == "struct":
            emit_struct_impl(structs[decl.name])

    emit_element_data()

    emit()

if __name__ == "__main__":

    parser = argparse.ArgumentParser("gen_rust.py")
    parser.add_argument("-i", help="Input ufbx_typed.json file")
    parser.add_argument("-o", help="Output path")
    argv = parser.parse_args()
    g_argv = argv

    src_path = os.path.dirname(os.path.realpath(__file__))

    input_file = argv.i
    if not input_file:
        input_file = os.path.join(src_path, "build", "ufbx_typed.json")

    output_path = argv.o
    if not output_path:
        output_path = os.path.join(src_path, "..", "src")

    with open(input_file, "rt") as f:
        file = ir.from_json(ir.File, json.load(f))

    if not os.path.exists(output_path):
        os.makedirs(output_path, exist_ok=True)

    with open(os.path.join(output_path, "generated.rs"), "wt", encoding="utf-8") as f:
        g_outfile = f
        init_file()
        emit_file()
