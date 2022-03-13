from re import L
from typing import Optional, List

import ufbx_ir as ir
import argparse
import os
import json

uses = r"""
use std::ffi::{c_void};
use std::{marker, result, ptr};
use std::ops::{Deref, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, FnMut, Index};
use crate::prelude::{Real, List, Ref, RefList, String, Blob, RawString, Unsafe, assert_to_panic};
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

pub struct GeometryCacheRoot {
    cache: *mut GeometryCache,
    _marker: marker::PhantomData<GeometryCache>,
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

impl GeometryCacheRoot {
    fn new(cache: *mut GeometryCache) -> GeometryCacheRoot {
        GeometryCacheRoot {
            cache: cache,
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

impl Drop for GeometryCacheRoot {
    fn drop(&mut self) {
        unsafe { ufbx_free_geometry_cache(self.cache) }
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

impl Clone for GeometryCacheRoot {
    fn clone(&self) -> Self {
        unsafe { ufbx_retain_geometry_cache(self.cache) }
        GeometryCacheRoot::new(self.cache)
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

impl Deref for GeometryCacheRoot {
    type Target = GeometryCache;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.cache }
    }
}

unsafe impl Send for SceneRoot {}
unsafe impl Sync for SceneRoot {}

unsafe impl Send for MeshRoot {}
unsafe impl Sync for MeshRoot {}

unsafe impl Send for GeometryCacheRoot {}
unsafe impl Sync for GeometryCacheRoot {}

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
    "geometryCache": "GeometryCacheRoot",
}

callback_signatures = {
    "ufbx_open_file_cb": "(&str) -> Option<Stream>",
    "ufbx_progress_cb": "(&Progress) -> ProgressResult",
}

primitive_types = {
    "void": "c_void",
    "char": "u8",
    "uint8_t": "i32",
    "int8_t": "u32",
    "int32_t": "i32",
    "uint32_t": "u32",
    "int64_t": "i64",
    "uint64_t": "u64",
    "float": "f32",
    "double": "f64",
    "size_t": "usize",
    "ptrdiff_t": "isize",
    "bool": "bool",
    "ufbx_real": "Real",
}

default_derive_types = {
    "ufbx_error_frame",
    "ufbx_error_type",
    "ufbx_error",
}

ignore_types = {
    "ufbx_string",
    "ufbx_blob",
}

ignore_non_raw = {
    "ufbx_open_file",
}

def get_struct_name(st: ir.Struct):
    name = ir.to_pascal(st.short_name)
    if st.is_input or st.is_callback or st.is_interface:
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
    return fn.short_name

class RustType:
    def __init__(self, irt: Optional[ir.Type], inner: Optional["RustType"]):
        self.ir = irt
        self.needs_lifetime = False
        self.is_list = False
        self.is_ref_list = False
        self.is_result = False
        self.is_void = False
        self.is_synthetic = False
        self.is_function = False
        self.is_raw = False
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
        else:
            return self.name

    def fmt_member(self, lifetime=""):
        if self.is_result:
            return f"Result<{self.inner.fmt_member(lifetime)}>"
        elif self.is_function:
            return self.fmt_raw()
        elif self.is_list:
            list_type = "RefList" if self.is_ref_list else "List"
            lt = f"'{lifetime}, " if lifetime else ""
            return f"{list_type}<{self.inner.fmt_member(lifetime)}>"
        elif self.kind == "array":
            num = self.ir.array_length
            return f"[{self.inner.fmt_member(lifetime)}; {num}]"
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

    def fmt_arg(self, lifetime="", force_const=False):
        if self.is_result:
            return f"Result<{self.inner.fmt_arg(lifetime)}>"
        elif self.is_function:
            return self.fmt_raw()
        elif self.is_list:
            list_type = "RefList" if self.is_ref_list else "List"
            return f"{list_type}<{self.inner.fmt_member(lifetime)}>"
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
        else:
            if self.needs_lifetime:
                lt = f"<'{lifetime}>" if lifetime else ""
                return f"{self.name}{lt}"
            else:
                return self.name

    def fmt_input(self, lifetime=""):
        if self.is_result:
            return f"Result<{self.inner.fmt_input(lifetime)}>"
        elif self.is_function:
            return self.fmt_raw()
        elif self.is_list:
            list_type = "RefList" if self.is_ref_list else "List"
            lt = f"'{lifetime}, " if lifetime else ""
            return f"{list_type}<{self.inner.fmt_input(lifetime)}>"
        elif self.is_synthetic and self.name == "RawString":
            lt = f"'{lifetime} " if lifetime else ""
            return f"Option<&{lt}str>"
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
        elif self.kind == "struct":
            rs = structs[self.ir.key]
            if rs.ir.is_input or rs.ir.is_interface:
                return rs.rust_name
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
    def __init__(self, arg: ir.Argument, kind: str):
        self.ir = arg
        self.num_ir = None
        self.name = get_arg_name(arg)
        self.type = init_type(file.types[arg.type])
        self.is_const = self.type.ir.is_const
        self.kind = kind
        leaf = self.type.get_leaf()
        self.is_raw = leaf.is_raw

    def fmt_arg(self, lifetime: str, non_raw: bool = False) -> str:
        if not self.ir.return_ref:
            lifetime = ""
        if self.kind == "string":
            return f"{self.name}: &str"
        elif self.kind == "blob":
            mut = "" if self.is_const else "mut "
            return f"{self.name}: &{mut}[u8]"
        elif self.kind == "slice":
            mut = "" if self.is_const else "mut "
            return f"{self.name}: &{mut}[{self.type.fmt_arg(lifetime)}]"
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
        if typ.inner:
            inner = init_type(file.types[typ.inner])
            if inner.needs_lifetime:
                inner_lifetime = True
        rt = RustType(typ, inner)
        types[typ.key] = rt

        if inner_lifetime:
            rt.needs_lifetime = True
        if typ.key in lifetime_types:
            rt.needs_lifetime = True

    return types[typ.key]

def propagate_lifetimes():
    updated = True
    while updated:
        updated = False
        for rt in types.values():
            if rt.kind != "struct": continue
            if rt.needs_lifetime: continue
            rs = structs[rt.ir.key]
            for field in rs.fields:
                if field.type.needs_lifetime:
                    rt.needs_lifetime = True
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

    rs.fields.append(RustField(field, rt))

def init_struct(st: ir.Struct):
    rs = RustStruct(st)
    for field in st.fields:
        init_fields(rs, field)
    structs[st.name] = rs

    if rs.ir.is_callback or rs.ir.is_input or rs.ir.is_interface:
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
    for arg in fn.arguments:
        if arg.kind == "stringPointer":
            rf.args.append(RustArgument(arg, "string"))
        elif arg.kind == "stringLength":
            pass
        elif arg.kind == "arrayPointer":
            ra = RustArgument(arg, "slice")
            ra.type = ra.type.inner
            rf.args.append(ra)
        elif arg.kind == "arrayLength":
            pass
        elif arg.kind == "blobPointer":
            rf.args.append(RustArgument(arg, "blob"))
        elif arg.kind == "blobSize":
            pass
        elif arg.kind == "error":
            pass
        else:
            arg_type = types[arg.type]
            while arg_type.inner:
                arg_type = arg_type.inner
            if arg_type.is_raw:
                rf.is_raw = True
            rf.args.append(RustArgument(arg, arg.kind))

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
    if rs.ir.is_pod or rs.ir.is_callback or rs.ir.is_input or rs.ir.is_interface:
        emit(f"#[derive(Clone, Copy)]")
    if rs.ir.name in default_derive_types or rs.ir.is_pod or rs.ir.is_input:
        emit(f"#[derive(Default)]")
    emit(f"pub struct {rs.name}{lifetime} {{")
    indent()

    for field in rs.fields:
        prefix = ""
        if not field.ir.private or rs.ir.is_input:
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

def emit_input_callback(rs: RustStruct):
    sig = callback_signatures[rs.ir.name]

    emit()
    emit(f"pub enum {rs.rust_name}<'a> {{")
    indent()
    emit("None,")
    emit(f"Mut(&'a mut dyn FnMut{sig}),")
    emit(f"Ref(&'a dyn Fn{sig}),")
    emit(f"Raw(Unsafe<{rs.name}>),")
    unindent()
    emit("}")

    emit()
    emit(f"impl<'a> Default for {rs.rust_name}<'a> {{")
    indent()
    emit("fn default() -> Self { Self::None }")
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

    emit()
    emit(f"fn from_rust(arg: &mut {rs.rust_name}) -> Self {{")
    indent()
    emit("match arg {")
    indent()
    emit(f"{rs.rust_name}::None => Default::default(),")
    emit(f"{rs.rust_name}::Ref(f) => Self::from_func(f),")
    emit(f"{rs.rust_name}::Mut(f) => Self::from_func(f),")
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

    lifetime = ""
    typ = types[rs.ir.name]
    needs_lifetime = typ.needs_lifetime

    for field in rs.fields:
        if field.ir.private: continue
        if field.type.kind == "struct":
            frs = structs[field.type.ir.key]
            if frs.ir.is_callback or frs.ir.name == "ufbx_string":
                needs_lifetime = True

    if needs_lifetime:
        lifetime = "<'a>"

    emit()
    emit(f"#[derive(Default)]")
    emit(f"pub struct {rs.rust_name}{lifetime} {{")
    indent()

    for field in rs.fields:
        if field.ir.private: continue
        prefix = "pub "
        lifetime = "a"
        emit(f"{prefix}{field.name}: {field.type.fmt_input(lifetime)},")

    unindent()
    emit("}")

    emit()
    emit(f"impl {rs.name} {{")
    indent()

    emit(f"fn from_rust(arg: &mut {rs.rust_name}) -> Self {{")
    indent()
    emit(f"{rs.name} {{")
    indent()

    for field in rs.fields:
        if field.ir.private:
            emit(f"{field.name}: 0,")
            continue

        has_from = False
        if field.type.kind == "struct":
            frs = structs[field.type.ir.key]
            if frs.ir.is_callback or frs.ir.is_input or frs.ir.is_interface:
                has_from = True
        elif field.type.name == "RawString":
            has_from = True

        if has_from:
            emit(f"{field.name}: {field.type.name}::from_rust(&mut arg.{field.name}),")
        else:
            emit(f"{field.name}: arg.{field.name},")

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
    for value in re.values:
        emit(f"pub const {value.name}: {re.name} = {re.name}(0x{value.value:x});")
    unindent()
    emit("}")

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

    ret = types[fn.return_type]
    needs_ref = False

    lt = "<'a>" if needs_ref else ""
    lifetime = "a" if needs_ref else ""

    args = ", ".join(fmt_ffi_arg(arg, lifetime) for arg in fn.arguments)
    if fn.return_type == "void":
        emit(f"pub fn {fn.name}{lt}({args});")
    else:
        ret = fmt_ffi_type(file.types[fn.return_type], lifetime)
        emit(f"pub fn {fn.name}{lt}({args}) -> {ret};")

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
    else:
        args.append(ra.name)

def emit_function(rf: RustFunction, non_raw: bool = False):
    if rf.ir.is_inline: return
    if rf.ir.is_ffi: return
    if rf.ir.kind in { "retain", "free" }: return

    is_raw = rf.is_raw and not non_raw

    needs_ref = False
    if rf.return_type.kind == "pointer":
        needs_ref = True

    lt = "<'a>" if needs_ref else ""
    lifetime = "a" if needs_ref else ""

    arg_str = ", ".join(arg.fmt_arg(lifetime, non_raw) for arg in rf.args)

    arg_pass = []
    for arg in rf.args:
        emit_arg_pass(arg_pass, arg)

    ret = ""
    if not rf.return_type.is_void:
        ret = f" -> {rf.return_type.fmt_arg(lifetime, force_const=True)}"

    is_unsafe = False

    emit()
    if is_raw:
        emit(f"pub unsafe fn {rf.name}_raw{lt}({arg_str}){ret} {{")
        is_unsafe = True
    else:
        emit(f"pub fn {rf.name}{lt}({arg_str}){ret} {{")
    indent()

    unsafe = "" if is_unsafe else "unsafe "

    if non_raw:
        for arg in rf.args:
            if arg.is_raw:
                leaf = arg.type.get_leaf()
                emit(f"let mut {arg.name}_mut = {arg.name};")
                emit(f"let {arg.name}_raw = {leaf.name}::from_rust(&mut {arg.name}_mut);")
        params = []
        for arg in rf.args:
            if arg.is_raw:
                params.append(f"&{arg.name}_raw")
            else:
                params.append(arg.name)
        params_str = ", ".join(params)
        emit(f"{unsafe}{{ {rf.name}_raw({params_str}) }}")
    else:
        if rf.ir.has_error:
            emit(f"let mut error: Error = Error::default();")
            arg_pass.append("&mut error")

        arg_pass_str = ", ".join(arg_pass)
        if not rf.return_type.is_void:
            emit(f"let result = {unsafe}{{ {rf.ir.name}({arg_pass_str}) }};")
        else:
            emit(f"{unsafe}{{ {rf.ir.name}({arg_pass_str}) }};")

        emit("assert_to_panic();")

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
            elif rf.return_type.kind == "pointer":
                res = f"{unsafe}{{ &*{res} }}"
            if rf.ir.has_error:
                res = f"Ok({res})"

            emit(res)

    unindent()
    emit("}")

    if rf.is_raw and not non_raw:
        if rf.ir.name not in ignore_non_raw:
            emit_function(rf, non_raw=True)

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
    emit("pub fn as_data(&self) -> ElementData {")
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

    unindent()
    emit("}")

    emit_lines(post_ffi)

    for decl in file.declarations:
        if decl.kind == "function":
            emit_function(functions[decl.name])

    emit_element_data()

    emit()

if __name__ == "__main__":

    parser = argparse.ArgumentParser("gen_rust.py")
    parser.add_argument("-i", help="Input ufbx.json file")
    parser.add_argument("-o", help="Output path")
    argv = parser.parse_args()

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
