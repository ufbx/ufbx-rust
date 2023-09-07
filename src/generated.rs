use std::ffi::{c_void};
use std::{marker, result, ptr, mem, str};
use std::fmt::{self, Debug};
use std::ops::{Deref, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, FnMut, Index};
use crate::prelude::{Real, List, Ref, RefList, String, Blob, RawString, RawBlob, Unsafe, ExternalRef, InlineBuf, VertexStream, format_flags};
use crate::prelude::{Allocator, Stream, call_open_file_cb, call_close_memory_cb, call_progress_cb};

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Vec2 {
    pub x: Real,
    pub y: Real,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Vec3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Vec4 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub w: Real,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Quat {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub w: Real,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RotationOrder {
    Xyz = 0,
    Xzy = 1,
    Yzx = 2,
    Yxz = 3,
    Zxy = 4,
    Zyx = 5,
    Spheric = 6,
}

impl Default for RotationOrder {
    fn default() -> Self { Self::Xyz }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Matrix {
    pub m00: Real,
    pub m10: Real,
    pub m20: Real,
    pub m01: Real,
    pub m11: Real,
    pub m21: Real,
    pub m02: Real,
    pub m12: Real,
    pub m22: Real,
    pub m03: Real,
    pub m13: Real,
    pub m23: Real,
}

#[repr(C)]
pub struct VoidList {
    pub data: *mut c_void,
    pub count: usize,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DomValueType {
    Number = 0,
    String = 1,
    ArrayI8 = 2,
    ArrayI32 = 3,
    ArrayI64 = 4,
    ArrayF32 = 5,
    ArrayF64 = 6,
    ArrayRawString = 7,
    ArrayIgnored = 8,
}

impl Default for DomValueType {
    fn default() -> Self { Self::Number }
}

#[repr(C)]
pub struct DomValue {
    pub type_: DomValueType,
    pub value_str: String,
    pub value_blob: Blob,
    pub value_int: i64,
    pub value_float: f64,
}

#[repr(C)]
pub struct DomNode {
    pub name: String,
    pub children: RefList<DomNode>,
    pub values: List<DomValue>,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PropType {
    Unknown = 0,
    Boolean = 1,
    Integer = 2,
    Number = 3,
    Vector = 4,
    Color = 5,
    ColorWithAlpha = 6,
    String = 7,
    DateTime = 8,
    Translation = 9,
    Rotation = 10,
    Scaling = 11,
    Distance = 12,
    Compound = 13,
    Blob = 14,
    Reference = 15,
}

impl Default for PropType {
    fn default() -> Self { Self::Unknown }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PropFlags(u32);
impl PropFlags {
    pub const NONE: PropFlags = PropFlags(0);
    pub const ANIMATABLE: PropFlags = PropFlags(0x1);
    pub const USER_DEFINED: PropFlags = PropFlags(0x2);
    pub const HIDDEN: PropFlags = PropFlags(0x4);
    pub const LOCK_X: PropFlags = PropFlags(0x10);
    pub const LOCK_Y: PropFlags = PropFlags(0x20);
    pub const LOCK_Z: PropFlags = PropFlags(0x40);
    pub const LOCK_W: PropFlags = PropFlags(0x80);
    pub const MUTE_X: PropFlags = PropFlags(0x100);
    pub const MUTE_Y: PropFlags = PropFlags(0x200);
    pub const MUTE_Z: PropFlags = PropFlags(0x400);
    pub const MUTE_W: PropFlags = PropFlags(0x800);
    pub const SYNTHETIC: PropFlags = PropFlags(0x1000);
    pub const ANIMATED: PropFlags = PropFlags(0x2000);
    pub const NOT_FOUND: PropFlags = PropFlags(0x4000);
    pub const CONNECTED: PropFlags = PropFlags(0x8000);
    pub const NO_VALUE: PropFlags = PropFlags(0x10000);
    pub const OVERRIDDEN: PropFlags = PropFlags(0x20000);
    pub const VALUE_REAL: PropFlags = PropFlags(0x100000);
    pub const VALUE_VEC2: PropFlags = PropFlags(0x200000);
    pub const VALUE_VEC3: PropFlags = PropFlags(0x400000);
    pub const VALUE_VEC4: PropFlags = PropFlags(0x800000);
    pub const VALUE_INT: PropFlags = PropFlags(0x1000000);
    pub const VALUE_STR: PropFlags = PropFlags(0x2000000);
    pub const VALUE_BLOB: PropFlags = PropFlags(0x4000000);
}

const PROPFLAGS_NAMES: [(&'static str, u32); 24] = [
    ("ANIMATABLE", 0x1),
    ("USER_DEFINED", 0x2),
    ("HIDDEN", 0x4),
    ("LOCK_X", 0x10),
    ("LOCK_Y", 0x20),
    ("LOCK_Z", 0x40),
    ("LOCK_W", 0x80),
    ("MUTE_X", 0x100),
    ("MUTE_Y", 0x200),
    ("MUTE_Z", 0x400),
    ("MUTE_W", 0x800),
    ("SYNTHETIC", 0x1000),
    ("ANIMATED", 0x2000),
    ("NOT_FOUND", 0x4000),
    ("CONNECTED", 0x8000),
    ("NO_VALUE", 0x10000),
    ("OVERRIDDEN", 0x20000),
    ("VALUE_REAL", 0x100000),
    ("VALUE_VEC2", 0x200000),
    ("VALUE_VEC3", 0x400000),
    ("VALUE_VEC4", 0x800000),
    ("VALUE_INT", 0x1000000),
    ("VALUE_STR", 0x2000000),
    ("VALUE_BLOB", 0x4000000),
];

impl PropFlags {
    pub fn any(self) -> bool { self.0 != 0 }
    pub fn has_any(self, bits: Self) -> bool { (self.0 & bits.0) != 0 }
    pub fn has_all(self, bits: Self) -> bool { (self.0 & bits.0) == bits.0 }
}
impl Default for PropFlags {
    fn default() -> Self { Self(0) }
}
impl Debug for PropFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_flags(f, &PROPFLAGS_NAMES, self.0)
    }
}
impl BitAnd for PropFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) }
}
impl BitAndAssign for PropFlags {
    fn bitand_assign(&mut self, rhs: Self) { *self = Self(self.0 & rhs.0) }
}
impl BitOr for PropFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) }
}
impl BitOrAssign for PropFlags {
    fn bitor_assign(&mut self, rhs: Self) { *self = Self(self.0 | rhs.0) }
}
impl BitXor for PropFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) }
}
impl BitXorAssign for PropFlags {
    fn bitxor_assign(&mut self, rhs: Self) { *self = Self(self.0 ^ rhs.0) }
}

#[repr(C)]
pub struct Prop {
    pub name: String,
    _internal_key: u32,
    pub type_: PropType,
    pub flags: PropFlags,
    pub value_str: String,
    pub value_blob: Blob,
    pub value_int: i64,
    pub value_vec4: Vec4,
}

#[repr(C)]
pub struct Props {
    pub props: List<Prop>,
    pub num_animated: usize,
    pub defaults: Option<Ref<Props>>,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ElementType {
    Unknown = 0,
    Node = 1,
    Mesh = 2,
    Light = 3,
    Camera = 4,
    Bone = 5,
    Empty = 6,
    LineCurve = 7,
    NurbsCurve = 8,
    NurbsSurface = 9,
    NurbsTrimSurface = 10,
    NurbsTrimBoundary = 11,
    ProceduralGeometry = 12,
    StereoCamera = 13,
    CameraSwitcher = 14,
    Marker = 15,
    LodGroup = 16,
    SkinDeformer = 17,
    SkinCluster = 18,
    BlendDeformer = 19,
    BlendChannel = 20,
    BlendShape = 21,
    CacheDeformer = 22,
    CacheFile = 23,
    Material = 24,
    Texture = 25,
    Video = 26,
    Shader = 27,
    ShaderBinding = 28,
    AnimStack = 29,
    AnimLayer = 30,
    AnimValue = 31,
    AnimCurve = 32,
    DisplayLayer = 33,
    SelectionSet = 34,
    SelectionNode = 35,
    Character = 36,
    Constraint = 37,
    Pose = 38,
    MetadataObject = 39,
}

impl Default for ElementType {
    fn default() -> Self { Self::Unknown }
}

#[repr(C)]
pub struct Connection {
    pub src: Ref<Element>,
    pub dst: Ref<Element>,
    pub src_prop: String,
    pub dst_prop: String,
}

#[repr(C)]
pub struct Element {
    pub name: String,
    pub props: Props,
    pub element_id: u32,
    pub typed_id: u32,
    pub instances: RefList<Node>,
    pub type_: ElementType,
    pub connections_src: List<Connection>,
    pub connections_dst: List<Connection>,
    pub dom_node: Option<Ref<DomNode>>,
    pub scene: Ref<Scene>,
}

#[repr(C)]
pub struct Unknown {
    pub element: Element,
    pub type_: String,
    pub super_type: String,
    pub sub_type: String,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InheritType {
    NoShear = 0,
    Normal = 1,
    NoScale = 2,
}

impl Default for InheritType {
    fn default() -> Self { Self::NoShear }
}

#[repr(C)]
pub struct Node {
    pub element: Element,
    pub parent: Option<Ref<Node>>,
    pub children: RefList<Node>,
    pub mesh: Option<Ref<Mesh>>,
    pub light: Option<Ref<Light>>,
    pub camera: Option<Ref<Camera>>,
    pub attrib: Option<Ref<Element>>,
    pub geometry_transform_helper: Option<Ref<Node>>,
    pub attrib_type: ElementType,
    pub all_attribs: RefList<Element>,
    pub inherit_type: InheritType,
    pub local_transform: Transform,
    pub geometry_transform: Transform,
    pub rotation_order: RotationOrder,
    pub euler_rotation: Vec3,
    pub world_transform: Transform,
    pub node_to_parent: Matrix,
    pub node_to_world: Matrix,
    pub geometry_to_node: Matrix,
    pub geometry_to_world: Matrix,
    pub adjust_pre_rotation: Quat,
    pub adjust_pre_scale: Vec3,
    pub adjust_post_rotation: Quat,
    pub materials: RefList<Material>,
    pub visible: bool,
    pub is_root: bool,
    pub has_geometry_transform: bool,
    pub has_adjust_transform: bool,
    pub is_geometry_transform_helper: bool,
    pub node_depth: u32,
}

#[repr(C)]
pub struct VertexAttrib {
    pub exists: bool,
    pub values: VoidList,
    pub indices: List<u32>,
    pub value_reals: usize,
    pub unique_per_vertex: bool,
}

#[repr(C)]
pub struct VertexReal {
    pub exists: bool,
    pub values: List<Real>,
    pub indices: List<u32>,
    pub value_reals: usize,
    pub unique_per_vertex: bool,
}

impl Index<usize> for VertexReal {
    type Output = Real;
    fn index(&self, index: usize) -> &Real {
        &self.values[self.indices[index] as usize]
    }
}

#[repr(C)]
pub struct VertexVec2 {
    pub exists: bool,
    pub values: List<Vec2>,
    pub indices: List<u32>,
    pub value_reals: usize,
    pub unique_per_vertex: bool,
}

impl Index<usize> for VertexVec2 {
    type Output = Vec2;
    fn index(&self, index: usize) -> &Vec2 {
        &self.values[self.indices[index] as usize]
    }
}

#[repr(C)]
pub struct VertexVec3 {
    pub exists: bool,
    pub values: List<Vec3>,
    pub indices: List<u32>,
    pub value_reals: usize,
    pub unique_per_vertex: bool,
}

impl Index<usize> for VertexVec3 {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Vec3 {
        &self.values[self.indices[index] as usize]
    }
}

#[repr(C)]
pub struct VertexVec4 {
    pub exists: bool,
    pub values: List<Vec4>,
    pub indices: List<u32>,
    pub value_reals: usize,
    pub unique_per_vertex: bool,
}

impl Index<usize> for VertexVec4 {
    type Output = Vec4;
    fn index(&self, index: usize) -> &Vec4 {
        &self.values[self.indices[index] as usize]
    }
}

#[repr(C)]
pub struct UvSet {
    pub name: String,
    pub index: u32,
    pub vertex_uv: VertexVec2,
    pub vertex_tangent: VertexVec3,
    pub vertex_bitangent: VertexVec3,
}

#[repr(C)]
pub struct ColorSet {
    pub name: String,
    pub index: u32,
    pub vertex_color: VertexVec4,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Edge {
    pub indices: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Face {
    pub index_begin: u32,
    pub num_indices: u32,
}

#[repr(C)]
pub struct MeshPart {
    pub index: u32,
    pub num_faces: usize,
    pub num_triangles: usize,
    pub num_empty_faces: usize,
    pub num_point_faces: usize,
    pub num_line_faces: usize,
    pub face_indices: List<u32>,
}

#[repr(C)]
pub struct FaceGroup {
    pub id: i32,
    pub name: String,
}

#[repr(C)]
pub struct SubdivisionWeightRange {
    pub weight_begin: u32,
    pub num_weights: u32,
}

#[repr(C)]
pub struct SubdivisionWeight {
    pub weight: Real,
    pub index: u32,
}

#[repr(C)]
pub struct SubdivisionResult {
    pub result_memory_used: usize,
    pub temp_memory_used: usize,
    pub result_allocs: usize,
    pub temp_allocs: usize,
    pub source_vertex_ranges: List<SubdivisionWeightRange>,
    pub source_vertex_weights: List<SubdivisionWeight>,
    pub skin_cluster_ranges: List<SubdivisionWeightRange>,
    pub skin_cluster_weights: List<SubdivisionWeight>,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SubdivisionDisplayMode {
    Disabled = 0,
    Hull = 1,
    HullAndSmooth = 2,
    Smooth = 3,
}

impl Default for SubdivisionDisplayMode {
    fn default() -> Self { Self::Disabled }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SubdivisionBoundary {
    Default = 0,
    Legacy = 1,
    SharpCorners = 2,
    SharpNone = 3,
    SharpBoundary = 4,
    SharpInterior = 5,
}

impl Default for SubdivisionBoundary {
    fn default() -> Self { Self::Default }
}

#[repr(C)]
pub struct Mesh {
    pub element: Element,
    pub num_vertices: usize,
    pub num_indices: usize,
    pub num_faces: usize,
    pub num_triangles: usize,
    pub num_edges: usize,
    pub max_face_triangles: usize,
    pub num_empty_faces: usize,
    pub num_point_faces: usize,
    pub num_line_faces: usize,
    pub faces: List<Face>,
    pub face_smoothing: List<bool>,
    pub face_material: List<u32>,
    pub face_group: List<u32>,
    pub face_hole: List<bool>,
    pub edges: List<Edge>,
    pub edge_smoothing: List<bool>,
    pub edge_crease: List<Real>,
    pub edge_visibility: List<bool>,
    pub vertex_indices: List<u32>,
    pub vertices: List<Vec3>,
    pub vertex_first_index: List<u32>,
    pub vertex_position: VertexVec3,
    pub vertex_normal: VertexVec3,
    pub vertex_uv: VertexVec2,
    pub vertex_tangent: VertexVec3,
    pub vertex_bitangent: VertexVec3,
    pub vertex_color: VertexVec4,
    pub vertex_crease: VertexReal,
    pub uv_sets: List<UvSet>,
    pub color_sets: List<ColorSet>,
    pub materials: RefList<Material>,
    pub face_groups: List<FaceGroup>,
    pub material_parts: List<MeshPart>,
    pub face_group_parts: List<MeshPart>,
    pub skinned_is_local: bool,
    pub skinned_position: VertexVec3,
    pub skinned_normal: VertexVec3,
    pub skin_deformers: RefList<SkinDeformer>,
    pub blend_deformers: RefList<BlendDeformer>,
    pub cache_deformers: RefList<CacheDeformer>,
    pub all_deformers: RefList<Element>,
    pub subdivision_preview_levels: u32,
    pub subdivision_render_levels: u32,
    pub subdivision_display_mode: SubdivisionDisplayMode,
    pub subdivision_boundary: SubdivisionBoundary,
    pub subdivision_uv_boundary: SubdivisionBoundary,
    pub generated_normals: bool,
    pub subdivision_evaluated: bool,
    pub subdivision_result: Option<Ref<SubdivisionResult>>,
    pub from_tessellated_nurbs: bool,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LightType {
    Point = 0,
    Directional = 1,
    Spot = 2,
    Area = 3,
    Volume = 4,
}

impl Default for LightType {
    fn default() -> Self { Self::Point }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LightDecay {
    None = 0,
    Linear = 1,
    Quadratic = 2,
    Cubic = 3,
}

impl Default for LightDecay {
    fn default() -> Self { Self::None }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LightAreaShape {
    Rectangle = 0,
    Sphere = 1,
}

impl Default for LightAreaShape {
    fn default() -> Self { Self::Rectangle }
}

#[repr(C)]
pub struct Light {
    pub element: Element,
    pub color: Vec3,
    pub intensity: Real,
    pub local_direction: Vec3,
    pub type_: LightType,
    pub decay: LightDecay,
    pub area_shape: LightAreaShape,
    pub inner_angle: Real,
    pub outer_angle: Real,
    pub cast_light: bool,
    pub cast_shadows: bool,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProjectionMode {
    Perspective = 0,
    Orthographic = 1,
}

impl Default for ProjectionMode {
    fn default() -> Self { Self::Perspective }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AspectMode {
    WindowSize = 0,
    FixedRatio = 1,
    FixedResolution = 2,
    FixedWidth = 3,
    FixedHeight = 4,
}

impl Default for AspectMode {
    fn default() -> Self { Self::WindowSize }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ApertureMode {
    HorizontalAndVertical = 0,
    Horizontal = 1,
    Vertical = 2,
    FocalLength = 3,
}

impl Default for ApertureMode {
    fn default() -> Self { Self::HorizontalAndVertical }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GateFit {
    None = 0,
    Vertical = 1,
    Horizontal = 2,
    Fill = 3,
    Overscan = 4,
    Stretch = 5,
}

impl Default for GateFit {
    fn default() -> Self { Self::None }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ApertureFormat {
    Custom = 0,
    E16MmTheatrical = 1,
    Super16Mm = 2,
    E35MmAcademy = 3,
    E35MmTvProjection = 4,
    E35MmFullAperture = 5,
    E35Mm185Projection = 6,
    E35MmAnamorphic = 7,
    E70MmProjection = 8,
    Vistavision = 9,
    Dynavision = 10,
    Imax = 11,
}

impl Default for ApertureFormat {
    fn default() -> Self { Self::Custom }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CoordinateAxis {
    PositiveX = 0,
    NegativeX = 1,
    PositiveY = 2,
    NegativeY = 3,
    PositiveZ = 4,
    NegativeZ = 5,
    Unknown = 6,
}

impl Default for CoordinateAxis {
    fn default() -> Self { Self::PositiveX }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct CoordinateAxes {
    pub right: CoordinateAxis,
    pub up: CoordinateAxis,
    pub front: CoordinateAxis,
}

#[repr(C)]
pub struct Camera {
    pub element: Element,
    pub projection_mode: ProjectionMode,
    pub resolution_is_pixels: bool,
    pub resolution: Vec2,
    pub field_of_view_deg: Vec2,
    pub field_of_view_tan: Vec2,
    pub orthographic_extent: Real,
    pub orthographic_size: Vec2,
    pub projection_plane: Vec2,
    pub aspect_ratio: Real,
    pub near_plane: Real,
    pub far_plane: Real,
    pub projection_axes: CoordinateAxes,
    pub aspect_mode: AspectMode,
    pub aperture_mode: ApertureMode,
    pub gate_fit: GateFit,
    pub aperture_format: ApertureFormat,
    pub focal_length_mm: Real,
    pub film_size_inch: Vec2,
    pub aperture_size_inch: Vec2,
    pub squeeze_ratio: Real,
}

#[repr(C)]
pub struct Bone {
    pub element: Element,
    pub radius: Real,
    pub relative_length: Real,
    pub is_root: bool,
}

#[repr(C)]
pub struct Empty {
    pub element: Element,
}

#[repr(C)]
pub struct LineSegment {
    pub index_begin: u32,
    pub num_indices: u32,
}

#[repr(C)]
pub struct LineCurve {
    pub element: Element,
    pub color: Vec3,
    pub control_points: List<Vec3>,
    pub point_indices: List<u32>,
    pub segments: List<LineSegment>,
    pub from_tessellated_nurbs: bool,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NurbsTopology {
    Open = 0,
    Periodic = 1,
    Closed = 2,
}

impl Default for NurbsTopology {
    fn default() -> Self { Self::Open }
}

#[repr(C)]
pub struct NurbsBasis {
    pub order: u32,
    pub topology: NurbsTopology,
    pub knot_vector: List<Real>,
    pub t_min: Real,
    pub t_max: Real,
    pub spans: List<Real>,
    pub is_2d: bool,
    pub num_wrap_control_points: usize,
    pub valid: bool,
}

#[repr(C)]
pub struct NurbsCurve {
    pub element: Element,
    pub basis: NurbsBasis,
    pub control_points: List<Vec4>,
}

#[repr(C)]
pub struct NurbsSurface {
    pub element: Element,
    pub basis_u: NurbsBasis,
    pub basis_v: NurbsBasis,
    pub num_control_points_u: usize,
    pub num_control_points_v: usize,
    pub control_points: List<Vec4>,
    pub span_subdivision_u: u32,
    pub span_subdivision_v: u32,
    pub flip_normals: bool,
    pub material: Option<Ref<Material>>,
}

#[repr(C)]
pub struct NurbsTrimSurface {
    pub element: Element,
}

#[repr(C)]
pub struct NurbsTrimBoundary {
    pub element: Element,
}

#[repr(C)]
pub struct ProceduralGeometry {
    pub element: Element,
}

#[repr(C)]
pub struct StereoCamera {
    pub element: Element,
    pub left: Ref<Camera>,
    pub right: Ref<Camera>,
}

#[repr(C)]
pub struct CameraSwitcher {
    pub element: Element,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MarkerType {
    Unknown = 0,
    FkEffector = 1,
    IkEffector = 2,
}

impl Default for MarkerType {
    fn default() -> Self { Self::Unknown }
}

#[repr(C)]
pub struct Marker {
    pub element: Element,
    pub type_: MarkerType,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LodDisplay {
    UseLod = 0,
    Show = 1,
    Hide = 2,
}

impl Default for LodDisplay {
    fn default() -> Self { Self::UseLod }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct LodLevel {
    pub distance: Real,
    pub display: LodDisplay,
}

#[repr(C)]
pub struct LodGroup {
    pub element: Element,
    pub relative_distances: bool,
    pub lod_levels: List<LodLevel>,
    pub ignore_parent_transform: bool,
    pub use_distance_limit: bool,
    pub distance_limit_min: Real,
    pub distance_limit_max: Real,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SkinningMethod {
    Linear = 0,
    Rigid = 1,
    DualQuaternion = 2,
    BlendedDqLinear = 3,
}

impl Default for SkinningMethod {
    fn default() -> Self { Self::Linear }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct SkinVertex {
    pub weight_begin: u32,
    pub num_weights: u32,
    pub dq_weight: Real,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct SkinWeight {
    pub cluster_index: u32,
    pub weight: Real,
}

#[repr(C)]
pub struct SkinDeformer {
    pub element: Element,
    pub skinning_method: SkinningMethod,
    pub clusters: RefList<SkinCluster>,
    pub vertices: List<SkinVertex>,
    pub weights: List<SkinWeight>,
    pub max_weights_per_vertex: usize,
    pub num_dq_weights: usize,
    pub dq_vertices: List<u32>,
    pub dq_weights: List<Real>,
}

#[repr(C)]
pub struct SkinCluster {
    pub element: Element,
    pub bone_node: Option<Ref<Node>>,
    pub geometry_to_bone: Matrix,
    pub mesh_node_to_bone: Matrix,
    pub bind_to_world: Matrix,
    pub geometry_to_world: Matrix,
    pub geometry_to_world_transform: Transform,
    pub num_weights: usize,
    pub vertices: List<u32>,
    pub weights: List<Real>,
}

#[repr(C)]
pub struct BlendDeformer {
    pub element: Element,
    pub channels: RefList<BlendChannel>,
}

#[repr(C)]
pub struct BlendKeyframe {
    pub shape: Ref<BlendShape>,
    pub target_weight: Real,
    pub effective_weight: Real,
}

#[repr(C)]
pub struct BlendChannel {
    pub element: Element,
    pub weight: Real,
    pub keyframes: List<BlendKeyframe>,
    pub target_shape: Ref<BlendShape>,
}

#[repr(C)]
pub struct BlendShape {
    pub element: Element,
    pub num_offsets: usize,
    pub offset_vertices: List<u32>,
    pub position_offsets: List<Vec3>,
    pub normal_offsets: List<Vec3>,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CacheFileFormat {
    Unknown = 0,
    Pc2 = 1,
    Mc = 2,
}

impl Default for CacheFileFormat {
    fn default() -> Self { Self::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CacheDataFormat {
    Unknown = 0,
    RealFloat = 1,
    Vec3Float = 2,
    RealDouble = 3,
    Vec3Double = 4,
}

impl Default for CacheDataFormat {
    fn default() -> Self { Self::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CacheDataEncoding {
    Unknown = 0,
    LittleEndian = 1,
    BigEndian = 2,
}

impl Default for CacheDataEncoding {
    fn default() -> Self { Self::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CacheInterpretation {
    Unknown = 0,
    Points = 1,
    VertexPosition = 2,
    VertexNormal = 3,
}

impl Default for CacheInterpretation {
    fn default() -> Self { Self::Unknown }
}

#[repr(C)]
pub struct CacheFrame {
    pub channel: String,
    pub time: f64,
    pub filename: String,
    pub file_format: CacheFileFormat,
    pub data_format: CacheDataFormat,
    pub data_encoding: CacheDataEncoding,
    pub data_offset: u64,
    pub data_count: u32,
    pub data_element_bytes: u32,
    pub data_total_bytes: u64,
}

#[repr(C)]
pub struct CacheChannel {
    pub name: String,
    pub interpretation: CacheInterpretation,
    pub interpretation_name: String,
    pub frames: List<CacheFrame>,
}

#[repr(C)]
pub struct GeometryCache {
    pub root_filename: String,
    pub channels: List<CacheChannel>,
    pub frames: List<CacheFrame>,
    pub extra_info: List<String>,
}

#[repr(C)]
pub struct CacheDeformer {
    pub element: Element,
    pub channel: String,
    pub file: Option<Ref<CacheFile>>,
    pub external_cache: Option<Ref<GeometryCache>>,
    pub external_channel: Option<Ref<CacheChannel>>,
}

#[repr(C)]
pub struct CacheFile {
    pub element: Element,
    pub filename: String,
    pub absolute_filename: String,
    pub relative_filename: String,
    pub raw_filename: Blob,
    pub raw_absolute_filename: Blob,
    pub raw_relative_filename: Blob,
    pub format: CacheFileFormat,
    pub external_cache: Option<Ref<GeometryCache>>,
}

#[repr(C)]
pub struct MaterialMap {
    pub value_vec4: Vec4,
    pub value_int: i64,
    pub texture: Option<Ref<Texture>>,
    pub has_value: bool,
    pub texture_enabled: bool,
    pub feature_disabled: bool,
    pub value_components: i32,
}

#[repr(C)]
pub struct MaterialFeatureInfo {
    pub enabled: bool,
    pub is_explicit: bool,
}

#[repr(C)]
pub struct MaterialTexture {
    pub material_prop: String,
    pub shader_prop: String,
    pub texture: Ref<Texture>,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShaderType {
    Unknown = 0,
    FbxLambert = 1,
    FbxPhong = 2,
    OslStandardSurface = 3,
    ArnoldStandardSurface = 4,
    E3DsMaxPhysicalMaterial = 5,
    E3DsMaxPbrMetalRough = 6,
    E3DsMaxPbrSpecGloss = 7,
    GltfMaterial = 8,
    ShaderfxGraph = 9,
    BlenderPhong = 10,
    WavefrontMtl = 11,
}

impl Default for ShaderType {
    fn default() -> Self { Self::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MaterialFbxMap {
    DiffuseFactor = 0,
    DiffuseColor = 1,
    SpecularFactor = 2,
    SpecularColor = 3,
    SpecularExponent = 4,
    ReflectionFactor = 5,
    ReflectionColor = 6,
    TransparencyFactor = 7,
    TransparencyColor = 8,
    EmissionFactor = 9,
    EmissionColor = 10,
    AmbientFactor = 11,
    AmbientColor = 12,
    NormalMap = 13,
    Bump = 14,
    BumpFactor = 15,
    DisplacementFactor = 16,
    Displacement = 17,
    VectorDisplacementFactor = 18,
    VectorDisplacement = 19,
}

impl Default for MaterialFbxMap {
    fn default() -> Self { Self::DiffuseFactor }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MaterialPbrMap {
    BaseFactor = 0,
    BaseColor = 1,
    Roughness = 2,
    Metalness = 3,
    DiffuseRoughness = 4,
    SpecularFactor = 5,
    SpecularColor = 6,
    SpecularIor = 7,
    SpecularAnisotropy = 8,
    SpecularRotation = 9,
    TransmissionFactor = 10,
    TransmissionColor = 11,
    TransmissionDepth = 12,
    TransmissionScatter = 13,
    TransmissionScatterAnisotropy = 14,
    TransmissionDispersion = 15,
    TransmissionRoughness = 16,
    TransmissionExtraRoughness = 17,
    TransmissionPriority = 18,
    TransmissionEnableInAov = 19,
    SubsurfaceFactor = 20,
    SubsurfaceColor = 21,
    SubsurfaceRadius = 22,
    SubsurfaceScale = 23,
    SubsurfaceAnisotropy = 24,
    SubsurfaceTintColor = 25,
    SubsurfaceType = 26,
    SheenFactor = 27,
    SheenColor = 28,
    SheenRoughness = 29,
    CoatFactor = 30,
    CoatColor = 31,
    CoatRoughness = 32,
    CoatIor = 33,
    CoatAnisotropy = 34,
    CoatRotation = 35,
    CoatNormal = 36,
    CoatAffectBaseColor = 37,
    CoatAffectBaseRoughness = 38,
    ThinFilmThickness = 39,
    ThinFilmIor = 40,
    EmissionFactor = 41,
    EmissionColor = 42,
    Opacity = 43,
    IndirectDiffuse = 44,
    IndirectSpecular = 45,
    NormalMap = 46,
    TangentMap = 47,
    DisplacementMap = 48,
    MatteFactor = 49,
    MatteColor = 50,
    AmbientOcclusion = 51,
    Glossiness = 52,
    CoatGlossiness = 53,
    TransmissionGlossiness = 54,
}

impl Default for MaterialPbrMap {
    fn default() -> Self { Self::BaseFactor }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MaterialFeature {
    Pbr = 0,
    Metalness = 1,
    Diffuse = 2,
    Specular = 3,
    Emission = 4,
    Transmission = 5,
    Coat = 6,
    Sheen = 7,
    Opacity = 8,
    AmbientOcclusion = 9,
    Matte = 10,
    Unlit = 11,
    Ior = 12,
    DiffuseRoughness = 13,
    TransmissionRoughness = 14,
    ThinWalled = 15,
    Caustics = 16,
    ExitToBackground = 17,
    InternalReflections = 18,
    DoubleSided = 19,
    RoughnessAsGlossiness = 20,
    CoatRoughnessAsGlossiness = 21,
    TransmissionRoughnessAsGlossiness = 22,
}

impl Default for MaterialFeature {
    fn default() -> Self { Self::Pbr }
}

#[repr(C)]
pub struct MaterialFbxMaps {
    pub diffuse_factor: MaterialMap,
    pub diffuse_color: MaterialMap,
    pub specular_factor: MaterialMap,
    pub specular_color: MaterialMap,
    pub specular_exponent: MaterialMap,
    pub reflection_factor: MaterialMap,
    pub reflection_color: MaterialMap,
    pub transparency_factor: MaterialMap,
    pub transparency_color: MaterialMap,
    pub emission_factor: MaterialMap,
    pub emission_color: MaterialMap,
    pub ambient_factor: MaterialMap,
    pub ambient_color: MaterialMap,
    pub normal_map: MaterialMap,
    pub bump: MaterialMap,
    pub bump_factor: MaterialMap,
    pub displacement_factor: MaterialMap,
    pub displacement: MaterialMap,
    pub vector_displacement_factor: MaterialMap,
    pub vector_displacement: MaterialMap,
}

#[repr(C)]
pub struct MaterialPbrMaps {
    pub base_factor: MaterialMap,
    pub base_color: MaterialMap,
    pub roughness: MaterialMap,
    pub metalness: MaterialMap,
    pub diffuse_roughness: MaterialMap,
    pub specular_factor: MaterialMap,
    pub specular_color: MaterialMap,
    pub specular_ior: MaterialMap,
    pub specular_anisotropy: MaterialMap,
    pub specular_rotation: MaterialMap,
    pub transmission_factor: MaterialMap,
    pub transmission_color: MaterialMap,
    pub transmission_depth: MaterialMap,
    pub transmission_scatter: MaterialMap,
    pub transmission_scatter_anisotropy: MaterialMap,
    pub transmission_dispersion: MaterialMap,
    pub transmission_roughness: MaterialMap,
    pub transmission_extra_roughness: MaterialMap,
    pub transmission_priority: MaterialMap,
    pub transmission_enable_in_aov: MaterialMap,
    pub subsurface_factor: MaterialMap,
    pub subsurface_color: MaterialMap,
    pub subsurface_radius: MaterialMap,
    pub subsurface_scale: MaterialMap,
    pub subsurface_anisotropy: MaterialMap,
    pub subsurface_tint_color: MaterialMap,
    pub subsurface_type: MaterialMap,
    pub sheen_factor: MaterialMap,
    pub sheen_color: MaterialMap,
    pub sheen_roughness: MaterialMap,
    pub coat_factor: MaterialMap,
    pub coat_color: MaterialMap,
    pub coat_roughness: MaterialMap,
    pub coat_ior: MaterialMap,
    pub coat_anisotropy: MaterialMap,
    pub coat_rotation: MaterialMap,
    pub coat_normal: MaterialMap,
    pub coat_affect_base_color: MaterialMap,
    pub coat_affect_base_roughness: MaterialMap,
    pub thin_film_thickness: MaterialMap,
    pub thin_film_ior: MaterialMap,
    pub emission_factor: MaterialMap,
    pub emission_color: MaterialMap,
    pub opacity: MaterialMap,
    pub indirect_diffuse: MaterialMap,
    pub indirect_specular: MaterialMap,
    pub normal_map: MaterialMap,
    pub tangent_map: MaterialMap,
    pub displacement_map: MaterialMap,
    pub matte_factor: MaterialMap,
    pub matte_color: MaterialMap,
    pub ambient_occlusion: MaterialMap,
    pub glossiness: MaterialMap,
    pub coat_glossiness: MaterialMap,
    pub transmission_glossiness: MaterialMap,
}

#[repr(C)]
pub struct MaterialFeatures {
    pub pbr: MaterialFeatureInfo,
    pub metalness: MaterialFeatureInfo,
    pub diffuse: MaterialFeatureInfo,
    pub specular: MaterialFeatureInfo,
    pub emission: MaterialFeatureInfo,
    pub transmission: MaterialFeatureInfo,
    pub coat: MaterialFeatureInfo,
    pub sheen: MaterialFeatureInfo,
    pub opacity: MaterialFeatureInfo,
    pub ambient_occlusion: MaterialFeatureInfo,
    pub matte: MaterialFeatureInfo,
    pub unlit: MaterialFeatureInfo,
    pub ior: MaterialFeatureInfo,
    pub diffuse_roughness: MaterialFeatureInfo,
    pub transmission_roughness: MaterialFeatureInfo,
    pub thin_walled: MaterialFeatureInfo,
    pub caustics: MaterialFeatureInfo,
    pub exit_to_background: MaterialFeatureInfo,
    pub internal_reflections: MaterialFeatureInfo,
    pub double_sided: MaterialFeatureInfo,
    pub roughness_as_glossiness: MaterialFeatureInfo,
    pub coat_roughness_as_glossiness: MaterialFeatureInfo,
    pub transmission_roughness_as_glossiness: MaterialFeatureInfo,
}

#[repr(C)]
pub struct Material {
    pub element: Element,
    pub fbx: MaterialFbxMaps,
    pub pbr: MaterialPbrMaps,
    pub features: MaterialFeatures,
    pub shader_type: ShaderType,
    pub shader: Option<Ref<Shader>>,
    pub shading_model_name: String,
    pub shader_prop_prefix: String,
    pub textures: List<MaterialTexture>,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextureType {
    File = 0,
    Layered = 1,
    Procedural = 2,
    Shader = 3,
}

impl Default for TextureType {
    fn default() -> Self { Self::File }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlendMode {
    Translucent = 0,
    Additive = 1,
    Multiply = 2,
    Multiply2X = 3,
    Over = 4,
    Replace = 5,
    Dissolve = 6,
    Darken = 7,
    ColorBurn = 8,
    LinearBurn = 9,
    DarkerColor = 10,
    Lighten = 11,
    Screen = 12,
    ColorDodge = 13,
    LinearDodge = 14,
    LighterColor = 15,
    SoftLight = 16,
    HardLight = 17,
    VividLight = 18,
    LinearLight = 19,
    PinLight = 20,
    HardMix = 21,
    Difference = 22,
    Exclusion = 23,
    Subtract = 24,
    Divide = 25,
    Hue = 26,
    Saturation = 27,
    Color = 28,
    Luminosity = 29,
    Overlay = 30,
}

impl Default for BlendMode {
    fn default() -> Self { Self::Translucent }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WrapMode {
    Repeat = 0,
    Clamp = 1,
}

impl Default for WrapMode {
    fn default() -> Self { Self::Repeat }
}

#[repr(C)]
pub struct TextureLayer {
    pub texture: Ref<Texture>,
    pub blend_mode: BlendMode,
    pub alpha: Real,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShaderTextureType {
    Unknown = 0,
    SelectOutput = 1,
    Osl = 2,
}

impl Default for ShaderTextureType {
    fn default() -> Self { Self::Unknown }
}

#[repr(C)]
pub struct ShaderTextureInput {
    pub name: String,
    pub value_vec4: Vec4,
    pub value_int: i64,
    pub value_str: String,
    pub value_blob: Blob,
    pub texture: Option<Ref<Texture>>,
    pub texture_output_index: i64,
    pub texture_enabled: bool,
    pub prop: Ref<Prop>,
    pub texture_prop: Option<Ref<Prop>>,
    pub texture_enabled_prop: Option<Ref<Prop>>,
}

#[repr(C)]
pub struct ShaderTexture {
    pub type_: ShaderTextureType,
    pub shader_name: String,
    pub shader_type_id: u64,
    pub inputs: List<ShaderTextureInput>,
    pub shader_source: String,
    pub raw_shader_source: Blob,
    pub main_texture: Ref<Texture>,
    pub main_texture_output_index: i64,
    pub prop_prefix: String,
}

#[repr(C)]
pub struct TextureFile {
    pub index: u32,
    pub filename: String,
    pub absolute_filename: String,
    pub relative_filename: String,
    pub raw_filename: Blob,
    pub raw_absolute_filename: Blob,
    pub raw_relative_filename: Blob,
    pub content: Blob,
}

#[repr(C)]
pub struct Texture {
    pub element: Element,
    pub type_: TextureType,
    pub filename: String,
    pub absolute_filename: String,
    pub relative_filename: String,
    pub raw_filename: Blob,
    pub raw_absolute_filename: Blob,
    pub raw_relative_filename: Blob,
    pub content: Blob,
    pub video: Option<Ref<Video>>,
    pub file_index: u32,
    pub has_file: bool,
    pub layers: List<TextureLayer>,
    pub shader: Option<Ref<ShaderTexture>>,
    pub file_textures: RefList<Texture>,
    pub uv_set: String,
    pub wrap_u: WrapMode,
    pub wrap_v: WrapMode,
    pub has_uv_transform: bool,
    pub uv_transform: Transform,
    pub texture_to_uv: Matrix,
    pub uv_to_texture: Matrix,
}

#[repr(C)]
pub struct Video {
    pub element: Element,
    pub filename: String,
    pub absolute_filename: String,
    pub relative_filename: String,
    pub raw_filename: Blob,
    pub raw_absolute_filename: Blob,
    pub raw_relative_filename: Blob,
    pub content: Blob,
}

#[repr(C)]
pub struct Shader {
    pub element: Element,
    pub type_: ShaderType,
    pub bindings: RefList<ShaderBinding>,
}

#[repr(C)]
pub struct ShaderPropBinding {
    pub shader_prop: String,
    pub material_prop: String,
}

#[repr(C)]
pub struct ShaderBinding {
    pub element: Element,
    pub prop_bindings: List<ShaderPropBinding>,
}

#[repr(C)]
pub struct PropOverride {
    pub element_id: u32,
    _internal_key: u32,
    pub prop_name: String,
    pub value: Vec4,
    pub value_str: String,
    pub value_int: i64,
}

#[repr(C)]
pub struct Anim {
    pub time_begin: f64,
    pub time_end: f64,
    pub layers: RefList<AnimLayer>,
    pub override_layer_weights: List<Real>,
    pub overrides: List<PropOverride>,
    pub ignore_connections: bool,
    pub custom: bool,
}

#[repr(C)]
pub struct AnimStack {
    pub element: Element,
    pub time_begin: f64,
    pub time_end: f64,
    pub layers: RefList<AnimLayer>,
    pub anim: Ref<Anim>,
}

#[repr(C)]
pub struct AnimProp {
    pub element: Ref<Element>,
    _internal_key: u32,
    pub prop_name: String,
    pub anim_value: Ref<AnimValue>,
}

#[repr(C)]
pub struct AnimLayer {
    pub element: Element,
    pub weight: Real,
    pub weight_is_animated: bool,
    pub blended: bool,
    pub additive: bool,
    pub compose_rotation: bool,
    pub compose_scale: bool,
    pub anim_values: RefList<AnimValue>,
    pub anim_props: List<AnimProp>,
    pub anim: Ref<Anim>,
    _min_element_id: u32,
    _max_element_id: u32,
    _element_id_bitmask: [u32; 4],
}

#[repr(C)]
pub struct AnimValue {
    pub element: Element,
    pub default_value: Vec3,
    pub curves: [Ref<AnimCurve>; 3],
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Interpolation {
    ConstantPrev = 0,
    ConstantNext = 1,
    Linear = 2,
    Cubic = 3,
}

impl Default for Interpolation {
    fn default() -> Self { Self::ConstantPrev }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Tangent {
    pub dx: f32,
    pub dy: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct Keyframe {
    pub time: f64,
    pub value: Real,
    pub interpolation: Interpolation,
    pub left: Tangent,
    pub right: Tangent,
}

#[repr(C)]
pub struct AnimCurve {
    pub element: Element,
    pub keyframes: List<Keyframe>,
}

#[repr(C)]
pub struct DisplayLayer {
    pub element: Element,
    pub nodes: RefList<Node>,
    pub visible: bool,
    pub frozen: bool,
    pub ui_color: Vec3,
}

#[repr(C)]
pub struct SelectionSet {
    pub element: Element,
    pub nodes: RefList<SelectionNode>,
}

#[repr(C)]
pub struct SelectionNode {
    pub element: Element,
    pub target_node: Option<Ref<Node>>,
    pub target_mesh: Option<Ref<Mesh>>,
    pub include_node: bool,
    pub vertices: List<u32>,
    pub edges: List<u32>,
    pub faces: List<u32>,
}

#[repr(C)]
pub struct Character {
    pub element: Element,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConstraintType {
    Unknown = 0,
    Aim = 1,
    Parent = 2,
    Position = 3,
    Rotation = 4,
    Scale = 5,
    SingleChainIk = 6,
}

impl Default for ConstraintType {
    fn default() -> Self { Self::Unknown }
}

#[repr(C)]
pub struct ConstraintTarget {
    pub node: Ref<Node>,
    pub weight: Real,
    pub transform: Transform,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConstraintAimUpType {
    Scene = 0,
    ToNode = 1,
    AlignNode = 2,
    Vector = 3,
    None = 4,
}

impl Default for ConstraintAimUpType {
    fn default() -> Self { Self::Scene }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ConstraintIkPoleType {
    Vector = 0,
    Node = 1,
}

impl Default for ConstraintIkPoleType {
    fn default() -> Self { Self::Vector }
}

#[repr(C)]
pub struct Constraint {
    pub element: Element,
    pub type_: ConstraintType,
    pub type_name: String,
    pub node: Option<Ref<Node>>,
    pub targets: List<ConstraintTarget>,
    pub weight: Real,
    pub active: bool,
    pub constrain_translation: [bool; 3],
    pub constrain_rotation: [bool; 3],
    pub constrain_scale: [bool; 3],
    pub transform_offset: Transform,
    pub aim_vector: Vec3,
    pub aim_up_type: ConstraintAimUpType,
    pub aim_up_node: Option<Ref<Node>>,
    pub aim_up_vector: Vec3,
    pub ik_effector: Option<Ref<Node>>,
    pub ik_end_node: Option<Ref<Node>>,
    pub ik_pole_vector: Vec3,
}

#[repr(C)]
pub struct BonePose {
    pub bone_node: Ref<Node>,
    pub bone_to_world: Matrix,
}

#[repr(C)]
pub struct Pose {
    pub element: Element,
    pub bind_pose: bool,
    pub bone_poses: List<BonePose>,
}

#[repr(C)]
pub struct MetadataObject {
    pub element: Element,
}

#[repr(C)]
pub struct NameElement {
    pub name: String,
    pub type_: ElementType,
    _internal_key: u32,
    pub element: Ref<Element>,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Exporter {
    Unknown = 0,
    FbxSdk = 1,
    BlenderBinary = 2,
    BlenderAscii = 3,
    MotionBuilder = 4,
    BcUnityExporter = 5,
}

impl Default for Exporter {
    fn default() -> Self { Self::Unknown }
}

#[repr(C)]
pub struct Application {
    pub vendor: String,
    pub name: String,
    pub version: String,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileFormat {
    Unknown = 0,
    Fbx = 1,
    Obj = 2,
    Mtl = 3,
}

impl Default for FileFormat {
    fn default() -> Self { Self::Unknown }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WarningType {
    MissingExternalFile = 0,
    ImplicitMtl = 1,
    TruncatedArray = 2,
    MissingGeometryData = 3,
    IndexClamped = 4,
    BadUnicode = 5,
    BadElementConnectedToRoot = 6,
    DuplicateObjectId = 7,
    EmptyFaceRemoved = 8,
    UnknownObjDirective = 9,
}

impl Default for WarningType {
    fn default() -> Self { Self::MissingExternalFile }
}

#[repr(C)]
pub struct Warning {
    pub type_: WarningType,
    pub description: String,
    pub element_id: u32,
    pub count: usize,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThumbnailFormat {
    Unknown = 0,
    Rgb24 = 1,
    Rgba32 = 2,
}

impl Default for ThumbnailFormat {
    fn default() -> Self { Self::Unknown }
}

#[repr(C)]
pub struct Thumbnail {
    pub props: Props,
    pub width: u32,
    pub height: u32,
    pub format: ThumbnailFormat,
    pub data: Blob,
}

#[repr(C)]
pub struct Metadata {
    pub warnings: List<Warning>,
    pub ascii: bool,
    pub version: u32,
    pub file_format: FileFormat,
    pub may_contain_no_index: bool,
    pub may_contain_missing_vertex_position: bool,
    pub may_contain_broken_elements: bool,
    pub is_unsafe: bool,
    pub has_warning: [bool; 10],
    pub creator: String,
    pub big_endian: bool,
    pub filename: String,
    pub relative_root: String,
    pub raw_filename: Blob,
    pub raw_relative_root: Blob,
    pub exporter: Exporter,
    pub exporter_version: u32,
    pub scene_props: Props,
    pub original_application: Application,
    pub latest_application: Application,
    pub thumbnail: Thumbnail,
    pub geometry_ignored: bool,
    pub animation_ignored: bool,
    pub embedded_ignored: bool,
    pub max_face_triangles: usize,
    pub result_memory_used: usize,
    pub temp_memory_used: usize,
    pub result_allocs: usize,
    pub temp_allocs: usize,
    pub element_buffer_size: usize,
    pub num_shader_textures: usize,
    pub bone_prop_size_unit: Real,
    pub bone_prop_limb_length_relative: bool,
    pub ktime_second: i64,
    pub original_file_path: String,
    pub raw_original_file_path: Blob,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TimeMode {
    Default = 0,
    E120Fps = 1,
    E100Fps = 2,
    E60Fps = 3,
    E50Fps = 4,
    E48Fps = 5,
    E30Fps = 6,
    E30FpsDrop = 7,
    NtscDropFrame = 8,
    NtscFullFrame = 9,
    Pal = 10,
    E24Fps = 11,
    E1000Fps = 12,
    FilmFullFrame = 13,
    Custom = 14,
    E96Fps = 15,
    E72Fps = 16,
    E5994Fps = 17,
}

impl Default for TimeMode {
    fn default() -> Self { Self::Default }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TimeProtocol {
    Smpte = 0,
    FrameCount = 1,
    Default = 2,
}

impl Default for TimeProtocol {
    fn default() -> Self { Self::Smpte }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnapMode {
    None = 0,
    Snap = 1,
    Play = 2,
    SnapAndPlay = 3,
}

impl Default for SnapMode {
    fn default() -> Self { Self::None }
}

#[repr(C)]
pub struct SceneSettings {
    pub props: Props,
    pub axes: CoordinateAxes,
    pub unit_meters: Real,
    pub frames_per_second: f64,
    pub ambient_color: Vec3,
    pub default_camera: String,
    pub time_mode: TimeMode,
    pub time_protocol: TimeProtocol,
    pub snap_mode: SnapMode,
    pub original_axis_up: CoordinateAxis,
    pub original_unit_meters: Real,
}

#[repr(C)]
pub struct Scene {
    pub metadata: Metadata,
    pub settings: SceneSettings,
    pub root_node: Ref<Node>,
    pub anim: Ref<Anim>,
    pub unknowns: RefList<Unknown>,
    pub nodes: RefList<Node>,
    pub meshes: RefList<Mesh>,
    pub lights: RefList<Light>,
    pub cameras: RefList<Camera>,
    pub bones: RefList<Bone>,
    pub empties: RefList<Empty>,
    pub line_curves: RefList<LineCurve>,
    pub nurbs_curves: RefList<NurbsCurve>,
    pub nurbs_surfaces: RefList<NurbsSurface>,
    pub nurbs_trim_surfaces: RefList<NurbsTrimSurface>,
    pub nurbs_trim_boundaries: RefList<NurbsTrimBoundary>,
    pub procedural_geometries: RefList<ProceduralGeometry>,
    pub stereo_cameras: RefList<StereoCamera>,
    pub camera_switchers: RefList<CameraSwitcher>,
    pub markers: RefList<Marker>,
    pub lod_groups: RefList<LodGroup>,
    pub skin_deformers: RefList<SkinDeformer>,
    pub skin_clusters: RefList<SkinCluster>,
    pub blend_deformers: RefList<BlendDeformer>,
    pub blend_channels: RefList<BlendChannel>,
    pub blend_shapes: RefList<BlendShape>,
    pub cache_deformers: RefList<CacheDeformer>,
    pub cache_files: RefList<CacheFile>,
    pub materials: RefList<Material>,
    pub textures: RefList<Texture>,
    pub videos: RefList<Video>,
    pub shaders: RefList<Shader>,
    pub shader_bindings: RefList<ShaderBinding>,
    pub anim_stacks: RefList<AnimStack>,
    pub anim_layers: RefList<AnimLayer>,
    pub anim_values: RefList<AnimValue>,
    pub anim_curves: RefList<AnimCurve>,
    pub display_layers: RefList<DisplayLayer>,
    pub selection_sets: RefList<SelectionSet>,
    pub selection_nodes: RefList<SelectionNode>,
    pub characters: RefList<Character>,
    pub constraints: RefList<Constraint>,
    pub poses: RefList<Pose>,
    pub metadata_objects: RefList<MetadataObject>,
    pub texture_files: List<TextureFile>,
    pub elements: RefList<Element>,
    pub connections_src: List<Connection>,
    pub connections_dst: List<Connection>,
    pub elements_by_name: List<NameElement>,
    pub dom_root: Option<Ref<DomNode>>,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct CurvePoint {
    pub valid: bool,
    pub position: Vec3,
    pub derivative: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct SurfacePoint {
    pub valid: bool,
    pub position: Vec3,
    pub derivative_u: Vec3,
    pub derivative_v: Vec3,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct TopoFlags(u32);
impl TopoFlags {
    pub const NONE: TopoFlags = TopoFlags(0);
    pub const NON_MANIFOLD: TopoFlags = TopoFlags(0x1);
}

const TOPOFLAGS_NAMES: [(&'static str, u32); 1] = [
    ("NON_MANIFOLD", 0x1),
];

impl TopoFlags {
    pub fn any(self) -> bool { self.0 != 0 }
    pub fn has_any(self, bits: Self) -> bool { (self.0 & bits.0) != 0 }
    pub fn has_all(self, bits: Self) -> bool { (self.0 & bits.0) == bits.0 }
}
impl Default for TopoFlags {
    fn default() -> Self { Self(0) }
}
impl Debug for TopoFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_flags(f, &TOPOFLAGS_NAMES, self.0)
    }
}
impl BitAnd for TopoFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output { Self(self.0 & rhs.0) }
}
impl BitAndAssign for TopoFlags {
    fn bitand_assign(&mut self, rhs: Self) { *self = Self(self.0 & rhs.0) }
}
impl BitOr for TopoFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) }
}
impl BitOrAssign for TopoFlags {
    fn bitor_assign(&mut self, rhs: Self) { *self = Self(self.0 | rhs.0) }
}
impl BitXor for TopoFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0 ^ rhs.0) }
}
impl BitXorAssign for TopoFlags {
    fn bitxor_assign(&mut self, rhs: Self) { *self = Self(self.0 ^ rhs.0) }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
#[derive(Debug)]
pub struct TopoEdge {
    pub index: u32,
    pub next: u32,
    pub prev: u32,
    pub twin: u32,
    pub face: u32,
    pub edge: u32,
    pub flags: TopoFlags,
}

#[repr(C)]
pub struct RawVertexStream {
    pub data: *mut c_void,
    pub vertex_count: usize,
    pub vertex_size: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawAllocator {
    pub alloc_fn: Option<unsafe extern "C" fn (*mut c_void, usize) -> *mut c_void>,
    pub realloc_fn: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, usize, usize) -> *mut c_void>,
    pub free_fn: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, usize)>,
    pub free_allocator_fn: Option<unsafe extern "C" fn (*mut c_void)>,
    pub user: *mut c_void,
}

impl Default for RawAllocator {
    fn default() -> Self {
        RawAllocator {
            alloc_fn: None,
            realloc_fn: None,
            free_fn: None,
            free_allocator_fn: None,
            user: ptr::null::<c_void>() as *mut c_void,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawAllocatorOpts {
    pub allocator: RawAllocator,
    pub memory_limit: usize,
    pub allocation_limit: usize,
    pub huge_threshold: usize,
    pub max_chunk_size: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawStream {
    pub read_fn: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, usize) -> usize>,
    pub skip_fn: Option<unsafe extern "C" fn (*mut c_void, usize) -> bool>,
    pub close_fn: Option<unsafe extern "C" fn (*mut c_void)>,
    pub user: *mut c_void,
}

impl Default for RawStream {
    fn default() -> Self {
        RawStream {
            read_fn: None,
            skip_fn: None,
            close_fn: None,
            user: ptr::null::<c_void>() as *mut c_void,
        }
    }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpenFileType {
    MainModel = 0,
    GeometryCache = 1,
    ObjMtl = 2,
}

impl Default for OpenFileType {
    fn default() -> Self { Self::MainModel }
}

#[repr(C)]
pub struct OpenFileInfo {
    pub type_: OpenFileType,
    pub temp_allocator: RawAllocator,
    pub original_filename: Blob,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawOpenFileCb {
    pub fn_: Option<unsafe extern "C" fn (*mut c_void, *mut RawStream, *const u8, usize, *const OpenFileInfo) -> bool>,
    pub user: *mut c_void,
}

impl Default for RawOpenFileCb {
    fn default() -> Self {
        RawOpenFileCb {
            fn_: None,
            user: ptr::null::<c_void>() as *mut c_void,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawCloseMemoryCb {
    pub fn_: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, usize)>,
    pub user: *mut c_void,
}

impl Default for RawCloseMemoryCb {
    fn default() -> Self {
        RawCloseMemoryCb {
            fn_: None,
            user: ptr::null::<c_void>() as *mut c_void,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawOpenMemoryOpts {
    pub _begin_zero: u32,
    pub allocator: RawAllocatorOpts,
    pub no_copy: bool,
    pub close_cb: RawCloseMemoryCb,
    pub _end_zero: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct ErrorFrame {
    pub source_line: u32,
    pub function: String,
    pub description: String,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ErrorType {
    None = 0,
    Unknown = 1,
    FileNotFound = 2,
    ExternalFileNotFound = 3,
    OutOfMemory = 4,
    MemoryLimit = 5,
    AllocationLimit = 6,
    TruncatedFile = 7,
    Io = 8,
    Cancelled = 9,
    UnrecognizedFileFormat = 10,
    UninitializedOptions = 11,
    ZeroVertexSize = 12,
    TruncatedVertexStream = 13,
    InvalidUtf8 = 14,
    FeatureDisabled = 15,
    BadNurbs = 16,
    BadIndex = 17,
    UnsafeOptions = 18,
    DuplicateOverride = 19,
}

impl Default for ErrorType {
    fn default() -> Self { Self::None }
}

#[repr(C)]
#[derive(Default)]
pub struct Error {
    pub type_: ErrorType,
    pub description: String,
    pub stack_size: u32,
    pub stack: [ErrorFrame; 8],
    info_length: usize,
    info_buf: InlineBuf<[u8; 256]>,
}

impl Error {
    pub fn info(&self) -> &str {
        unsafe {
            let buf: &[mem::MaybeUninit<u8>; 256] = mem::transmute(&self.info_buf);
            str::from_utf8(mem::transmute(&buf[..self.info_length])).unwrap()
        }
    }
}

#[repr(C)]
pub struct Progress {
    pub bytes_read: u64,
    pub bytes_total: u64,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProgressResult {
    Continue = 256,
    Cancel = 512,
}

impl Default for ProgressResult {
    fn default() -> Self { Self::Continue }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawProgressCb {
    pub fn_: Option<unsafe extern "C" fn (*mut c_void, *const Progress) -> ProgressResult>,
    pub user: *mut c_void,
}

impl Default for RawProgressCb {
    fn default() -> Self {
        RawProgressCb {
            fn_: None,
            user: ptr::null::<c_void>() as *mut c_void,
        }
    }
}

#[repr(C)]
pub struct InflateInput {
    pub total_size: usize,
    pub data: *const c_void,
    pub data_size: usize,
    pub buffer: *mut c_void,
    pub buffer_size: usize,
    pub read_fn: Option<unsafe extern "C" fn (*mut c_void, *mut c_void, usize) -> usize>,
    pub read_user: *mut c_void,
    pub progress_cb: RawProgressCb,
    pub progress_interval_hint: u64,
    pub progress_size_before: u64,
    pub progress_size_after: u64,
    pub no_header: bool,
    pub no_checksum: bool,
    pub internal_fast_bits: usize,
}

#[repr(C)]
pub struct InflateRetain {
    pub initialized: bool,
    pub data: [u64; 1024],
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IndexErrorHandling {
    Clamp = 0,
    NoIndex = 1,
    AbortLoading = 2,
    UnsafeIgnore = 3,
}

impl Default for IndexErrorHandling {
    fn default() -> Self { Self::Clamp }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnicodeErrorHandling {
    ReplacementCharacter = 0,
    Underscore = 1,
    QuestionMark = 2,
    Remove = 3,
    AbortLoading = 4,
    UnsafeIgnore = 5,
}

impl Default for UnicodeErrorHandling {
    fn default() -> Self { Self::ReplacementCharacter }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GeometryTransformHandling {
    Preserve = 0,
    HelperNodes = 1,
    ModifyGeometry = 2,
    ModifyGeometryNoFallback = 3,
}

impl Default for GeometryTransformHandling {
    fn default() -> Self { Self::Preserve }
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpaceConversion {
    TransformRoot = 0,
    AdjustTransforms = 1,
}

impl Default for SpaceConversion {
    fn default() -> Self { Self::TransformRoot }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawLoadOpts {
    pub _begin_zero: u32,
    pub temp_allocator: RawAllocatorOpts,
    pub result_allocator: RawAllocatorOpts,
    pub ignore_geometry: bool,
    pub ignore_animation: bool,
    pub ignore_embedded: bool,
    pub ignore_all_content: bool,
    pub evaluate_skinning: bool,
    pub evaluate_caches: bool,
    pub load_external_files: bool,
    pub ignore_missing_external_files: bool,
    pub skip_skin_vertices: bool,
    pub skip_mesh_parts: bool,
    pub clean_skin_weights: bool,
    pub disable_quirks: bool,
    pub strict: bool,
    pub allow_unsafe: bool,
    pub index_error_handling: IndexErrorHandling,
    pub connect_broken_elements: bool,
    pub allow_nodes_out_of_root: bool,
    pub allow_missing_vertex_position: bool,
    pub allow_empty_faces: bool,
    pub generate_missing_normals: bool,
    pub open_main_file_with_default: bool,
    pub path_separator: u8,
    pub file_size_estimate: u64,
    pub read_buffer_size: usize,
    pub filename: RawString,
    pub raw_filename: RawBlob,
    pub progress_cb: RawProgressCb,
    pub progress_interval_hint: u64,
    pub open_file_cb: RawOpenFileCb,
    pub geometry_transform_handling: GeometryTransformHandling,
    pub space_conversion: SpaceConversion,
    pub target_axes: CoordinateAxes,
    pub target_unit_meters: Real,
    pub target_camera_axes: CoordinateAxes,
    pub target_light_axes: CoordinateAxes,
    pub geometry_transform_helper_name: RawString,
    pub no_prop_unit_scaling: bool,
    pub no_anim_curve_unit_scaling: bool,
    pub normalize_normals: bool,
    pub normalize_tangents: bool,
    pub use_root_transform: bool,
    pub root_transform: Transform,
    pub unicode_error_handling: UnicodeErrorHandling,
    pub retain_dom: bool,
    pub file_format: FileFormat,
    pub file_format_lookahead: usize,
    pub no_format_from_content: bool,
    pub no_format_from_extension: bool,
    pub obj_search_mtl_by_filename: bool,
    pub obj_merge_objects: bool,
    pub obj_merge_groups: bool,
    pub obj_split_groups: bool,
    pub obj_mtl_path: RawString,
    pub obj_mtl_data: RawBlob,
    pub _end_zero: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawEvaluateOpts {
    pub _begin_zero: u32,
    pub temp_allocator: RawAllocatorOpts,
    pub result_allocator: RawAllocatorOpts,
    pub evaluate_skinning: bool,
    pub evaluate_caches: bool,
    pub load_external_files: bool,
    pub open_file_cb: RawOpenFileCb,
    pub _end_zero: u32,
}

#[repr(C)]
pub struct PropOverrideDesc {
    pub element_id: u32,
    pub prop_name: String,
    pub value: Vec4,
    pub value_str: String,
    pub value_int: i64,
}

#[repr(C)]
pub struct AnimOpts {
    _begin_zero: u32,
    pub layer_ids: List<u32>,
    pub override_layer_weights: List<Real>,
    pub overrides: List<PropOverrideDesc>,
    pub ignore_connections: bool,
    pub result_allocator: RawAllocatorOpts,
    _end_zero: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawTessellateCurveOpts {
    pub _begin_zero: u32,
    pub temp_allocator: RawAllocatorOpts,
    pub result_allocator: RawAllocatorOpts,
    pub span_subdivision: u32,
    pub _end_zero: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawTessellateSurfaceOpts {
    pub _begin_zero: u32,
    pub temp_allocator: RawAllocatorOpts,
    pub result_allocator: RawAllocatorOpts,
    pub span_subdivision_u: u32,
    pub span_subdivision_v: u32,
    pub skip_mesh_parts: bool,
    pub _end_zero: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawSubdivideOpts {
    pub _begin_zero: u32,
    pub temp_allocator: RawAllocatorOpts,
    pub result_allocator: RawAllocatorOpts,
    pub boundary: SubdivisionBoundary,
    pub uv_boundary: SubdivisionBoundary,
    pub ignore_normals: bool,
    pub interpolate_normals: bool,
    pub interpolate_tangents: bool,
    pub evaluate_source_vertices: bool,
    pub max_source_vertices: usize,
    pub evaluate_skin_weights: bool,
    pub max_skin_weights: usize,
    pub skin_deformer_index: usize,
    pub _end_zero: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawGeometryCacheOpts {
    pub _begin_zero: u32,
    pub temp_allocator: RawAllocatorOpts,
    pub result_allocator: RawAllocatorOpts,
    pub open_file_cb: RawOpenFileCb,
    pub frames_per_second: f64,
    pub _end_zero: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawGeometryCacheDataOpts {
    pub _begin_zero: u32,
    pub open_file_cb: RawOpenFileCb,
    pub additive: bool,
    pub use_weight: bool,
    pub weight: Real,
    pub _end_zero: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct Panic {
    pub did_panic: bool,
    message_length: usize,
    message_buf: InlineBuf<[u8; 128]>,
}

impl Panic {
    pub fn message(&self) -> &str {
        unsafe {
            let buf: &[mem::MaybeUninit<u8>; 128] = mem::transmute(&self.message_buf);
            str::from_utf8(mem::transmute(&buf[..self.message_length])).unwrap()
        }
    }
}

#[derive(Default)]
pub struct AllocatorOpts {
    pub allocator: Allocator,
    pub memory_limit: usize,
    pub allocation_limit: usize,
    pub huge_threshold: usize,
    pub max_chunk_size: usize,
}

impl RawAllocatorOpts {
    pub fn from_rust(arg: &mut AllocatorOpts) -> Self {
        RawAllocatorOpts {
            allocator: RawAllocator::from_rust(&mut arg.allocator),
            memory_limit: arg.memory_limit,
            allocation_limit: arg.allocation_limit,
            huge_threshold: arg.huge_threshold,
            max_chunk_size: arg.max_chunk_size,
        }
    }
}

pub enum OpenFileCb<'a> {
    None,
    Mut(&'a mut dyn FnMut(&str, &OpenFileInfo) -> Option<Stream>),
    Ref(&'a dyn Fn(&str, &OpenFileInfo) -> Option<Stream>),
    Raw(Unsafe<RawOpenFileCb>),
}

impl<'a> Default for OpenFileCb<'a> {
    fn default() -> Self { Self::None }
}

impl RawOpenFileCb {
    fn from_func<F: FnMut(&str, &OpenFileInfo) -> Option<Stream>>(arg: &mut F) -> Self {
        RawOpenFileCb {
            fn_: Some(call_open_file_cb::<F>),
            user: arg as *mut F as *mut c_void,
        }
    }

    fn from_rust(arg: &mut OpenFileCb) -> Self {
        match arg {
            OpenFileCb::None => Default::default(),
            OpenFileCb::Ref(f) => Self::from_func(f),
            OpenFileCb::Mut(f) => Self::from_func(f),
            OpenFileCb::Raw(raw) => raw.take(),
        }
    }
}

pub enum CloseMemoryCb<'a> {
    None,
    Mut(&'a mut dyn FnMut(*mut c_void, usize) -> ()),
    Ref(&'a dyn Fn(*mut c_void, usize) -> ()),
    Raw(Unsafe<RawCloseMemoryCb>),
}

impl<'a> Default for CloseMemoryCb<'a> {
    fn default() -> Self { Self::None }
}

impl RawCloseMemoryCb {
    fn from_func<F: FnMut(*mut c_void, usize) -> ()>(arg: &mut F) -> Self {
        RawCloseMemoryCb {
            fn_: Some(call_close_memory_cb::<F>),
            user: arg as *mut F as *mut c_void,
        }
    }

    fn from_rust(arg: &mut CloseMemoryCb) -> Self {
        match arg {
            CloseMemoryCb::None => Default::default(),
            CloseMemoryCb::Ref(f) => Self::from_func(f),
            CloseMemoryCb::Mut(f) => Self::from_func(f),
            CloseMemoryCb::Raw(raw) => raw.take(),
        }
    }
}

#[derive(Default)]
pub struct OpenMemoryOpts<'a> {
    pub allocator: AllocatorOpts,
    pub no_copy: Unsafe<bool>,
    pub close_cb: CloseMemoryCb<'a>,
}

impl RawOpenMemoryOpts {
    pub fn from_rust(arg: &mut OpenMemoryOpts) -> Self {
        RawOpenMemoryOpts {
            _begin_zero: 0,
            allocator: RawAllocatorOpts::from_rust(&mut arg.allocator),
            no_copy: arg.no_copy.take(),
            close_cb: RawCloseMemoryCb::from_rust(&mut arg.close_cb),
            _end_zero: 0,
        }
    }
}

pub enum ProgressCb<'a> {
    None,
    Mut(&'a mut dyn FnMut(&Progress) -> ProgressResult),
    Ref(&'a dyn Fn(&Progress) -> ProgressResult),
    Raw(Unsafe<RawProgressCb>),
}

impl<'a> Default for ProgressCb<'a> {
    fn default() -> Self { Self::None }
}

impl RawProgressCb {
    fn from_func<F: FnMut(&Progress) -> ProgressResult>(arg: &mut F) -> Self {
        RawProgressCb {
            fn_: Some(call_progress_cb::<F>),
            user: arg as *mut F as *mut c_void,
        }
    }

    fn from_rust(arg: &mut ProgressCb) -> Self {
        match arg {
            ProgressCb::None => Default::default(),
            ProgressCb::Ref(f) => Self::from_func(f),
            ProgressCb::Mut(f) => Self::from_func(f),
            ProgressCb::Raw(raw) => raw.take(),
        }
    }
}

#[derive(Default)]
pub struct LoadOpts<'a> {
    pub temp_allocator: AllocatorOpts,
    pub result_allocator: AllocatorOpts,
    pub ignore_geometry: bool,
    pub ignore_animation: bool,
    pub ignore_embedded: bool,
    pub ignore_all_content: bool,
    pub evaluate_skinning: bool,
    pub evaluate_caches: bool,
    pub load_external_files: bool,
    pub ignore_missing_external_files: bool,
    pub skip_skin_vertices: bool,
    pub skip_mesh_parts: bool,
    pub clean_skin_weights: bool,
    pub disable_quirks: bool,
    pub strict: bool,
    pub allow_unsafe: Unsafe<bool>,
    pub index_error_handling: IndexErrorHandling,
    pub connect_broken_elements: bool,
    pub allow_nodes_out_of_root: bool,
    pub allow_missing_vertex_position: bool,
    pub allow_empty_faces: bool,
    pub generate_missing_normals: bool,
    pub open_main_file_with_default: bool,
    pub path_separator: u8,
    pub file_size_estimate: u64,
    pub read_buffer_size: usize,
    pub filename: Option<&'a str>,
    pub raw_filename: Option<&'a [u8]>,
    pub progress_cb: ProgressCb<'a>,
    pub progress_interval_hint: u64,
    pub open_file_cb: OpenFileCb<'a>,
    pub geometry_transform_handling: GeometryTransformHandling,
    pub space_conversion: SpaceConversion,
    pub target_axes: CoordinateAxes,
    pub target_unit_meters: Real,
    pub target_camera_axes: CoordinateAxes,
    pub target_light_axes: CoordinateAxes,
    pub geometry_transform_helper_name: Option<&'a str>,
    pub no_prop_unit_scaling: bool,
    pub no_anim_curve_unit_scaling: bool,
    pub normalize_normals: bool,
    pub normalize_tangents: bool,
    pub use_root_transform: bool,
    pub root_transform: Transform,
    pub unicode_error_handling: UnicodeErrorHandling,
    pub retain_dom: bool,
    pub file_format: FileFormat,
    pub file_format_lookahead: usize,
    pub no_format_from_content: bool,
    pub no_format_from_extension: bool,
    pub obj_search_mtl_by_filename: bool,
    pub obj_merge_objects: bool,
    pub obj_merge_groups: bool,
    pub obj_split_groups: bool,
    pub obj_mtl_path: Option<&'a str>,
    pub obj_mtl_data: Option<&'a [u8]>,
}

impl RawLoadOpts {
    pub fn from_rust(arg: &mut LoadOpts) -> Self {
        RawLoadOpts {
            _begin_zero: 0,
            temp_allocator: RawAllocatorOpts::from_rust(&mut arg.temp_allocator),
            result_allocator: RawAllocatorOpts::from_rust(&mut arg.result_allocator),
            ignore_geometry: arg.ignore_geometry,
            ignore_animation: arg.ignore_animation,
            ignore_embedded: arg.ignore_embedded,
            ignore_all_content: arg.ignore_all_content,
            evaluate_skinning: arg.evaluate_skinning,
            evaluate_caches: arg.evaluate_caches,
            load_external_files: arg.load_external_files,
            ignore_missing_external_files: arg.ignore_missing_external_files,
            skip_skin_vertices: arg.skip_skin_vertices,
            skip_mesh_parts: arg.skip_mesh_parts,
            clean_skin_weights: arg.clean_skin_weights,
            disable_quirks: arg.disable_quirks,
            strict: arg.strict,
            allow_unsafe: arg.allow_unsafe.take(),
            index_error_handling: arg.index_error_handling,
            connect_broken_elements: arg.connect_broken_elements,
            allow_nodes_out_of_root: arg.allow_nodes_out_of_root,
            allow_missing_vertex_position: arg.allow_missing_vertex_position,
            allow_empty_faces: arg.allow_empty_faces,
            generate_missing_normals: arg.generate_missing_normals,
            open_main_file_with_default: arg.open_main_file_with_default,
            path_separator: arg.path_separator,
            file_size_estimate: arg.file_size_estimate,
            read_buffer_size: arg.read_buffer_size,
            filename: RawString::from_rust(&mut arg.filename),
            raw_filename: RawBlob::from_rust(&mut arg.raw_filename),
            progress_cb: RawProgressCb::from_rust(&mut arg.progress_cb),
            progress_interval_hint: arg.progress_interval_hint,
            open_file_cb: RawOpenFileCb::from_rust(&mut arg.open_file_cb),
            geometry_transform_handling: arg.geometry_transform_handling,
            space_conversion: arg.space_conversion,
            target_axes: arg.target_axes,
            target_unit_meters: arg.target_unit_meters,
            target_camera_axes: arg.target_camera_axes,
            target_light_axes: arg.target_light_axes,
            geometry_transform_helper_name: RawString::from_rust(&mut arg.geometry_transform_helper_name),
            no_prop_unit_scaling: arg.no_prop_unit_scaling,
            no_anim_curve_unit_scaling: arg.no_anim_curve_unit_scaling,
            normalize_normals: arg.normalize_normals,
            normalize_tangents: arg.normalize_tangents,
            use_root_transform: arg.use_root_transform,
            root_transform: arg.root_transform,
            unicode_error_handling: arg.unicode_error_handling,
            retain_dom: arg.retain_dom,
            file_format: arg.file_format,
            file_format_lookahead: arg.file_format_lookahead,
            no_format_from_content: arg.no_format_from_content,
            no_format_from_extension: arg.no_format_from_extension,
            obj_search_mtl_by_filename: arg.obj_search_mtl_by_filename,
            obj_merge_objects: arg.obj_merge_objects,
            obj_merge_groups: arg.obj_merge_groups,
            obj_split_groups: arg.obj_split_groups,
            obj_mtl_path: RawString::from_rust(&mut arg.obj_mtl_path),
            obj_mtl_data: RawBlob::from_rust(&mut arg.obj_mtl_data),
            _end_zero: 0,
        }
    }
}

#[derive(Default)]
pub struct EvaluateOpts<'a> {
    pub temp_allocator: AllocatorOpts,
    pub result_allocator: AllocatorOpts,
    pub evaluate_skinning: bool,
    pub evaluate_caches: bool,
    pub load_external_files: bool,
    pub open_file_cb: OpenFileCb<'a>,
}

impl RawEvaluateOpts {
    pub fn from_rust(arg: &mut EvaluateOpts) -> Self {
        RawEvaluateOpts {
            _begin_zero: 0,
            temp_allocator: RawAllocatorOpts::from_rust(&mut arg.temp_allocator),
            result_allocator: RawAllocatorOpts::from_rust(&mut arg.result_allocator),
            evaluate_skinning: arg.evaluate_skinning,
            evaluate_caches: arg.evaluate_caches,
            load_external_files: arg.load_external_files,
            open_file_cb: RawOpenFileCb::from_rust(&mut arg.open_file_cb),
            _end_zero: 0,
        }
    }
}

#[derive(Default)]
pub struct TessellateCurveOpts {
    pub temp_allocator: AllocatorOpts,
    pub result_allocator: AllocatorOpts,
    pub span_subdivision: u32,
}

impl RawTessellateCurveOpts {
    pub fn from_rust(arg: &mut TessellateCurveOpts) -> Self {
        RawTessellateCurveOpts {
            _begin_zero: 0,
            temp_allocator: RawAllocatorOpts::from_rust(&mut arg.temp_allocator),
            result_allocator: RawAllocatorOpts::from_rust(&mut arg.result_allocator),
            span_subdivision: arg.span_subdivision,
            _end_zero: 0,
        }
    }
}

#[derive(Default)]
pub struct TessellateSurfaceOpts {
    pub temp_allocator: AllocatorOpts,
    pub result_allocator: AllocatorOpts,
    pub span_subdivision_u: u32,
    pub span_subdivision_v: u32,
    pub skip_mesh_parts: bool,
}

impl RawTessellateSurfaceOpts {
    pub fn from_rust(arg: &mut TessellateSurfaceOpts) -> Self {
        RawTessellateSurfaceOpts {
            _begin_zero: 0,
            temp_allocator: RawAllocatorOpts::from_rust(&mut arg.temp_allocator),
            result_allocator: RawAllocatorOpts::from_rust(&mut arg.result_allocator),
            span_subdivision_u: arg.span_subdivision_u,
            span_subdivision_v: arg.span_subdivision_v,
            skip_mesh_parts: arg.skip_mesh_parts,
            _end_zero: 0,
        }
    }
}

#[derive(Default)]
pub struct SubdivideOpts {
    pub temp_allocator: AllocatorOpts,
    pub result_allocator: AllocatorOpts,
    pub boundary: SubdivisionBoundary,
    pub uv_boundary: SubdivisionBoundary,
    pub ignore_normals: bool,
    pub interpolate_normals: bool,
    pub interpolate_tangents: bool,
    pub evaluate_source_vertices: bool,
    pub max_source_vertices: usize,
    pub evaluate_skin_weights: bool,
    pub max_skin_weights: usize,
    pub skin_deformer_index: usize,
}

impl RawSubdivideOpts {
    pub fn from_rust(arg: &mut SubdivideOpts) -> Self {
        RawSubdivideOpts {
            _begin_zero: 0,
            temp_allocator: RawAllocatorOpts::from_rust(&mut arg.temp_allocator),
            result_allocator: RawAllocatorOpts::from_rust(&mut arg.result_allocator),
            boundary: arg.boundary,
            uv_boundary: arg.uv_boundary,
            ignore_normals: arg.ignore_normals,
            interpolate_normals: arg.interpolate_normals,
            interpolate_tangents: arg.interpolate_tangents,
            evaluate_source_vertices: arg.evaluate_source_vertices,
            max_source_vertices: arg.max_source_vertices,
            evaluate_skin_weights: arg.evaluate_skin_weights,
            max_skin_weights: arg.max_skin_weights,
            skin_deformer_index: arg.skin_deformer_index,
            _end_zero: 0,
        }
    }
}

#[derive(Default)]
pub struct GeometryCacheOpts<'a> {
    pub temp_allocator: AllocatorOpts,
    pub result_allocator: AllocatorOpts,
    pub open_file_cb: OpenFileCb<'a>,
    pub frames_per_second: f64,
}

impl RawGeometryCacheOpts {
    pub fn from_rust(arg: &mut GeometryCacheOpts) -> Self {
        RawGeometryCacheOpts {
            _begin_zero: 0,
            temp_allocator: RawAllocatorOpts::from_rust(&mut arg.temp_allocator),
            result_allocator: RawAllocatorOpts::from_rust(&mut arg.result_allocator),
            open_file_cb: RawOpenFileCb::from_rust(&mut arg.open_file_cb),
            frames_per_second: arg.frames_per_second,
            _end_zero: 0,
        }
    }
}

#[derive(Default)]
pub struct GeometryCacheDataOpts<'a> {
    pub open_file_cb: OpenFileCb<'a>,
    pub additive: bool,
    pub use_weight: bool,
    pub weight: Real,
}

impl RawGeometryCacheDataOpts {
    pub fn from_rust(arg: &mut GeometryCacheDataOpts) -> Self {
        RawGeometryCacheDataOpts {
            _begin_zero: 0,
            open_file_cb: RawOpenFileCb::from_rust(&mut arg.open_file_cb),
            additive: arg.additive,
            use_weight: arg.use_weight,
            weight: arg.weight,
            _end_zero: 0,
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

#[link(name="ufbx")]
extern "C" {
    pub static ufbx_empty_string: String;
    pub static ufbx_empty_blob: Blob;
    pub static ufbx_identity_matrix: Matrix;
    pub static ufbx_identity_transform: Transform;
    pub static ufbx_zero_vec2: Vec2;
    pub static ufbx_zero_vec3: Vec3;
    pub static ufbx_zero_vec4: Vec4;
    pub static ufbx_identity_quat: Quat;
    pub static ufbx_axes_right_handed_y_up: CoordinateAxes;
    pub static ufbx_axes_right_handed_z_up: CoordinateAxes;
    pub static ufbx_axes_left_handed_y_up: CoordinateAxes;
    pub static ufbx_axes_left_handed_z_up: CoordinateAxes;
    pub static ufbx_source_version: u32;
    pub fn ufbx_is_thread_safe() -> bool;
    pub fn ufbx_load_memory(data: *const c_void, data_size: usize, opts: *const RawLoadOpts, error: *mut Error) -> *mut Scene;
    pub fn ufbx_load_file(filename: *const u8, opts: *const RawLoadOpts, error: *mut Error) -> *mut Scene;
    pub fn ufbx_load_file_len(filename: *const u8, filename_len: usize, opts: *const RawLoadOpts, error: *mut Error) -> *mut Scene;
    pub fn ufbx_load_stdio(file: *mut c_void, opts: *const RawLoadOpts, error: *mut Error) -> *mut Scene;
    pub fn ufbx_load_stdio_prefix(file: *mut c_void, prefix: *const c_void, prefix_size: usize, opts: *const RawLoadOpts, error: *mut Error) -> *mut Scene;
    pub fn ufbx_load_stream(stream: *const RawStream, opts: *const RawLoadOpts, error: *mut Error) -> *mut Scene;
    pub fn ufbx_load_stream_prefix(stream: *const RawStream, prefix: *const c_void, prefix_size: usize, opts: *const RawLoadOpts, error: *mut Error) -> *mut Scene;
    pub fn ufbx_free_scene(scene: *mut Scene);
    pub fn ufbx_retain_scene(scene: *mut Scene);
    pub fn ufbx_format_error(dst: *mut u8, dst_size: usize, error: *const Error) -> usize;
    pub fn ufbx_find_prop_len(props: *const Props, name: *const u8, name_len: usize) -> *mut Prop;
    pub fn ufbx_find_real_len(props: *const Props, name: *const u8, name_len: usize, def: Real) -> Real;
    pub fn ufbx_find_vec3_len(props: *const Props, name: *const u8, name_len: usize, def: Vec3) -> Vec3;
    pub fn ufbx_find_int_len(props: *const Props, name: *const u8, name_len: usize, def: i64) -> i64;
    pub fn ufbx_find_bool_len(props: *const Props, name: *const u8, name_len: usize, def: bool) -> bool;
    pub fn ufbx_find_string_len(props: *const Props, name: *const u8, name_len: usize, def: String) -> String;
    pub fn ufbx_find_blob_len(props: *const Props, name: *const u8, name_len: usize, def: Blob) -> Blob;
    pub fn ufbx_find_prop_concat(props: *const Props, parts: *const String, num_parts: usize) -> *mut Prop;
    pub fn ufbx_get_prop_element(element: *const Element, prop: *const Prop, type_: ElementType) -> *mut Element;
    pub fn ufbx_find_prop_element_len(element: *const Element, name: *const u8, name_len: usize, type_: ElementType) -> *mut Element;
    pub fn ufbx_find_element_len(scene: *const Scene, type_: ElementType, name: *const u8, name_len: usize) -> *mut Element;
    pub fn ufbx_find_node_len(scene: *const Scene, name: *const u8, name_len: usize) -> *mut Node;
    pub fn ufbx_find_anim_stack_len(scene: *const Scene, name: *const u8, name_len: usize) -> *mut AnimStack;
    pub fn ufbx_find_material_len(scene: *const Scene, name: *const u8, name_len: usize) -> *mut Material;
    pub fn ufbx_find_anim_prop_len(layer: *const AnimLayer, element: *const Element, prop: *const u8, prop_len: usize) -> *mut AnimProp;
    pub fn ufbx_find_anim_props(layer: *const AnimLayer, element: *const Element) -> List<AnimProp>;
    pub fn ufbx_get_compatible_matrix_for_normals(node: *const Node) -> Matrix;
    pub fn ufbx_inflate(dst: *mut c_void, dst_size: usize, input: *const InflateInput, retain: *mut InflateRetain) -> isize;
    pub fn ufbx_open_file(stream: *mut RawStream, path: *const u8, path_len: usize) -> bool;
    pub fn ufbx_default_open_file(user: *mut c_void, stream: *mut RawStream, path: *const u8, path_len: usize, info: *const OpenFileInfo) -> bool;
    pub fn ufbx_open_memory(stream: *mut RawStream, data: *const c_void, data_size: usize, opts: *const RawOpenMemoryOpts, error: *mut Error) -> bool;
    pub fn ufbx_evaluate_curve(curve: *const AnimCurve, time: f64, default_value: Real) -> Real;
    pub fn ufbx_evaluate_anim_value_real(anim_value: *const AnimValue, time: f64) -> Real;
    pub fn ufbx_evaluate_anim_value_vec2(anim_value: *const AnimValue, time: f64) -> Vec2;
    pub fn ufbx_evaluate_anim_value_vec3(anim_value: *const AnimValue, time: f64) -> Vec3;
    pub fn ufbx_evaluate_prop_len(anim: *const Anim, element: *const Element, name: *const u8, name_len: usize, time: f64) -> Prop;
    pub fn ufbx_evaluate_props(anim: *const Anim, element: *const Element, time: f64, buffer: *mut Prop, buffer_size: usize) -> Props;
    pub fn ufbx_evaluate_transform(anim: *const Anim, node: *const Node, time: f64) -> Transform;
    pub fn ufbx_evaluate_blend_weight(anim: *const Anim, channel: *const BlendChannel, time: f64) -> Real;
    pub fn ufbx_evaluate_scene(scene: *const Scene, anim: *const Anim, time: f64, opts: *const RawEvaluateOpts, error: *mut Error) -> *mut Scene;
    pub fn ufbx_create_anim(scene: *const Scene, opts: *const AnimOpts, error: *mut Error) -> *mut Anim;
    pub fn ufbx_retain_anim(anim: *mut Anim);
    pub fn ufbx_free_anim(anim: *mut Anim);
    pub fn ufbx_find_prop_texture_len(material: *const Material, name: *const u8, name_len: usize) -> *mut Texture;
    pub fn ufbx_find_shader_prop_len(shader: *const Shader, name: *const u8, name_len: usize) -> String;
    pub fn ufbx_find_shader_prop_bindings_len(shader: *const Shader, name: *const u8, name_len: usize) -> List<ShaderPropBinding>;
    pub fn ufbx_find_shader_texture_input_len(shader: *const ShaderTexture, name: *const u8, name_len: usize) -> *mut ShaderTextureInput;
    pub fn ufbx_coordinate_axes_valid(axes: CoordinateAxes) -> bool;
    pub fn ufbx_quat_dot(a: Quat, b: Quat) -> Real;
    pub fn ufbx_quat_mul(a: Quat, b: Quat) -> Quat;
    pub fn ufbx_quat_normalize(q: Quat) -> Quat;
    pub fn ufbx_quat_fix_antipodal(q: Quat, reference: Quat) -> Quat;
    pub fn ufbx_quat_slerp(a: Quat, b: Quat, t: Real) -> Quat;
    pub fn ufbx_quat_rotate_vec3(q: Quat, v: Vec3) -> Vec3;
    pub fn ufbx_quat_to_euler(q: Quat, order: RotationOrder) -> Vec3;
    pub fn ufbx_euler_to_quat(v: Vec3, order: RotationOrder) -> Quat;
    pub fn ufbx_matrix_mul(a: *const Matrix, b: *const Matrix) -> Matrix;
    pub fn ufbx_matrix_determinant(m: *const Matrix) -> Real;
    pub fn ufbx_matrix_invert(m: *const Matrix) -> Matrix;
    pub fn ufbx_matrix_for_normals(m: *const Matrix) -> Matrix;
    pub fn ufbx_transform_position(m: *const Matrix, v: Vec3) -> Vec3;
    pub fn ufbx_transform_direction(m: *const Matrix, v: Vec3) -> Vec3;
    pub fn ufbx_transform_to_matrix(t: *const Transform) -> Matrix;
    pub fn ufbx_matrix_to_transform(m: *const Matrix) -> Transform;
    pub fn ufbx_catch_get_skin_vertex_matrix(panic: *mut Panic, skin: *const SkinDeformer, vertex: usize, fallback: *const Matrix) -> Matrix;
    pub fn ufbx_get_blend_shape_vertex_offset(shape: *const BlendShape, vertex: usize) -> Vec3;
    pub fn ufbx_get_blend_vertex_offset(blend: *const BlendDeformer, vertex: usize) -> Vec3;
    pub fn ufbx_add_blend_shape_vertex_offsets(shape: *const BlendShape, vertices: *mut Vec3, num_vertices: usize, weight: Real);
    pub fn ufbx_add_blend_vertex_offsets(blend: *const BlendDeformer, vertices: *mut Vec3, num_vertices: usize, weight: Real);
    pub fn ufbx_evaluate_nurbs_basis(basis: *const NurbsBasis, u: Real, weights: *mut Real, num_weights: usize, derivatives: *mut Real, num_derivatives: usize) -> usize;
    pub fn ufbx_evaluate_nurbs_curve(curve: *const NurbsCurve, u: Real) -> CurvePoint;
    pub fn ufbx_evaluate_nurbs_surface(surface: *const NurbsSurface, u: Real, v: Real) -> SurfacePoint;
    pub fn ufbx_tessellate_nurbs_curve(curve: *const NurbsCurve, opts: *const RawTessellateCurveOpts, error: *mut Error) -> *mut LineCurve;
    pub fn ufbx_tessellate_nurbs_surface(surface: *const NurbsSurface, opts: *const RawTessellateSurfaceOpts, error: *mut Error) -> *mut Mesh;
    pub fn ufbx_free_line_curve(curve: *mut LineCurve);
    pub fn ufbx_retain_line_curve(curve: *mut LineCurve);
    pub fn ufbx_find_face_index(mesh: *mut Mesh, index: usize) -> u32;
    pub fn ufbx_catch_triangulate_face(panic: *mut Panic, indices: *mut u32, num_indices: usize, mesh: *const Mesh, face: Face) -> u32;
    pub fn ufbx_catch_compute_topology(panic: *mut Panic, mesh: *const Mesh, topo: *mut TopoEdge, num_topo: usize);
    pub fn ufbx_catch_topo_next_vertex_edge(panic: *mut Panic, topo: *const TopoEdge, num_topo: usize, index: u32) -> u32;
    pub fn ufbx_catch_topo_prev_vertex_edge(panic: *mut Panic, topo: *const TopoEdge, num_topo: usize, index: u32) -> u32;
    pub fn ufbx_catch_get_weighted_face_normal(panic: *mut Panic, positions: *const VertexVec3, face: Face) -> Vec3;
    pub fn ufbx_catch_generate_normal_mapping(panic: *mut Panic, mesh: *const Mesh, topo: *const TopoEdge, num_topo: usize, normal_indices: *mut u32, num_normal_indices: usize, assume_smooth: bool) -> usize;
    pub fn ufbx_generate_normal_mapping(mesh: *const Mesh, topo: *const TopoEdge, num_topo: usize, normal_indices: *mut u32, num_normal_indices: usize, assume_smooth: bool) -> usize;
    pub fn ufbx_catch_compute_normals(panic: *mut Panic, mesh: *const Mesh, positions: *const VertexVec3, normal_indices: *const u32, num_normal_indices: usize, normals: *mut Vec3, num_normals: usize);
    pub fn ufbx_compute_normals(mesh: *const Mesh, positions: *const VertexVec3, normal_indices: *const u32, num_normal_indices: usize, normals: *mut Vec3, num_normals: usize);
    pub fn ufbx_subdivide_mesh(mesh: *const Mesh, level: usize, opts: *const RawSubdivideOpts, error: *mut Error) -> *mut Mesh;
    pub fn ufbx_free_mesh(mesh: *mut Mesh);
    pub fn ufbx_retain_mesh(mesh: *mut Mesh);
    pub fn ufbx_load_geometry_cache(filename: *const u8, opts: *const RawGeometryCacheOpts, error: *mut Error) -> *mut GeometryCache;
    pub fn ufbx_load_geometry_cache_len(filename: *const u8, filename_len: usize, opts: *const RawGeometryCacheOpts, error: *mut Error) -> *mut GeometryCache;
    pub fn ufbx_free_geometry_cache(cache: *mut GeometryCache);
    pub fn ufbx_retain_geometry_cache(cache: *mut GeometryCache);
    pub fn ufbx_read_geometry_cache_real(frame: *const CacheFrame, data: *mut Real, num_data: usize, opts: *const RawGeometryCacheDataOpts) -> usize;
    pub fn ufbx_sample_geometry_cache_real(channel: *const CacheChannel, time: f64, data: *mut Real, num_data: usize, opts: *const RawGeometryCacheDataOpts) -> usize;
    pub fn ufbx_read_geometry_cache_vec3(frame: *const CacheFrame, data: *mut Vec3, num_data: usize, opts: *const RawGeometryCacheDataOpts) -> usize;
    pub fn ufbx_sample_geometry_cache_vec3(channel: *const CacheChannel, time: f64, data: *mut Vec3, num_data: usize, opts: *const RawGeometryCacheDataOpts) -> usize;
    pub fn ufbx_dom_find_len(parent: *const DomNode, name: *const u8, name_len: usize) -> *mut DomNode;
    pub fn ufbx_generate_indices(streams: *const RawVertexStream, num_streams: usize, indices: *mut u32, num_indices: usize, allocator: *const RawAllocatorOpts, error: *mut Error) -> usize;
    pub fn ufbx_catch_get_vertex_real(panic: *mut Panic, v: *const VertexReal, index: usize) -> Real;
    pub fn ufbx_catch_get_vertex_vec2(panic: *mut Panic, v: *const VertexVec2, index: usize) -> Vec2;
    pub fn ufbx_catch_get_vertex_vec3(panic: *mut Panic, v: *const VertexVec3, index: usize) -> Vec3;
    pub fn ufbx_catch_get_vertex_vec4(panic: *mut Panic, v: *const VertexVec4, index: usize) -> Vec4;
    pub fn ufbx_get_triangulate_face_num_indices(face: Face) -> usize;
    pub fn ufbx_as_unknown(element: *const Element) -> *mut Unknown;
    pub fn ufbx_as_node(element: *const Element) -> *mut Node;
    pub fn ufbx_as_mesh(element: *const Element) -> *mut Mesh;
    pub fn ufbx_as_light(element: *const Element) -> *mut Light;
    pub fn ufbx_as_camera(element: *const Element) -> *mut Camera;
    pub fn ufbx_as_bone(element: *const Element) -> *mut Bone;
    pub fn ufbx_as_empty(element: *const Element) -> *mut Empty;
    pub fn ufbx_as_line_curve(element: *const Element) -> *mut LineCurve;
    pub fn ufbx_as_nurbs_curve(element: *const Element) -> *mut NurbsCurve;
    pub fn ufbx_as_nurbs_surface(element: *const Element) -> *mut NurbsSurface;
    pub fn ufbx_as_nurbs_trim_surface(element: *const Element) -> *mut NurbsTrimSurface;
    pub fn ufbx_as_nurbs_trim_boundary(element: *const Element) -> *mut NurbsTrimBoundary;
    pub fn ufbx_as_procedural_geometry(element: *const Element) -> *mut ProceduralGeometry;
    pub fn ufbx_as_stereo_camera(element: *const Element) -> *mut StereoCamera;
    pub fn ufbx_as_camera_switcher(element: *const Element) -> *mut CameraSwitcher;
    pub fn ufbx_as_marker(element: *const Element) -> *mut Marker;
    pub fn ufbx_as_lod_group(element: *const Element) -> *mut LodGroup;
    pub fn ufbx_as_skin_deformer(element: *const Element) -> *mut SkinDeformer;
    pub fn ufbx_as_skin_cluster(element: *const Element) -> *mut SkinCluster;
    pub fn ufbx_as_blend_deformer(element: *const Element) -> *mut BlendDeformer;
    pub fn ufbx_as_blend_channel(element: *const Element) -> *mut BlendChannel;
    pub fn ufbx_as_blend_shape(element: *const Element) -> *mut BlendShape;
    pub fn ufbx_as_cache_deformer(element: *const Element) -> *mut CacheDeformer;
    pub fn ufbx_as_cache_file(element: *const Element) -> *mut CacheFile;
    pub fn ufbx_as_material(element: *const Element) -> *mut Material;
    pub fn ufbx_as_texture(element: *const Element) -> *mut Texture;
    pub fn ufbx_as_video(element: *const Element) -> *mut Video;
    pub fn ufbx_as_shader(element: *const Element) -> *mut Shader;
    pub fn ufbx_as_shader_binding(element: *const Element) -> *mut ShaderBinding;
    pub fn ufbx_as_anim_stack(element: *const Element) -> *mut AnimStack;
    pub fn ufbx_as_anim_layer(element: *const Element) -> *mut AnimLayer;
    pub fn ufbx_as_anim_value(element: *const Element) -> *mut AnimValue;
    pub fn ufbx_as_anim_curve(element: *const Element) -> *mut AnimCurve;
    pub fn ufbx_as_display_layer(element: *const Element) -> *mut DisplayLayer;
    pub fn ufbx_as_selection_set(element: *const Element) -> *mut SelectionSet;
    pub fn ufbx_as_selection_node(element: *const Element) -> *mut SelectionNode;
    pub fn ufbx_as_character(element: *const Element) -> *mut Character;
    pub fn ufbx_as_constraint(element: *const Element) -> *mut Constraint;
    pub fn ufbx_as_pose(element: *const Element) -> *mut Pose;
    pub fn ufbx_as_metadata_object(element: *const Element) -> *mut MetadataObject;
    pub fn ufbx_ffi_find_int_len(retval: *mut i64, props: *const Props, name: *const u8, name_len: usize, def: *const i64);
    pub fn ufbx_ffi_find_vec3_len(retval: *mut Vec3, props: *const Props, name: *const u8, name_len: usize, def: *const Vec3);
    pub fn ufbx_ffi_find_string_len(retval: *mut String, props: *const Props, name: *const u8, name_len: usize, def: *const String);
    pub fn ufbx_ffi_find_anim_props(retval: *mut List<AnimProp>, layer: *const AnimLayer, element: *const Element);
    pub fn ufbx_ffi_get_compatible_matrix_for_normals(retval: *mut Matrix, node: *const Node);
    pub fn ufbx_ffi_evaluate_anim_value_vec2(retval: *mut Vec2, anim_value: *const AnimValue, time: f64);
    pub fn ufbx_ffi_evaluate_anim_value_vec3(retval: *mut Vec3, anim_value: *const AnimValue, time: f64);
    pub fn ufbx_ffi_evaluate_prop_len(retval: *mut Prop, anim: *const Anim, element: *const Element, name: *const u8, name_len: usize, time: f64);
    pub fn ufbx_ffi_evaluate_props(retval: *mut Props, anim: *const Anim, element: *mut Element, time: f64, buffer: *mut Prop, buffer_size: usize);
    pub fn ufbx_ffi_evaluate_transform(retval: *mut Transform, anim: *const Anim, node: *const Node, time: f64);
    pub fn ufbx_ffi_evaluate_blend_weight(anim: *const Anim, channel: *const BlendChannel, time: f64) -> Real;
    pub fn ufbx_ffi_quat_mul(retval: *mut Quat, a: *const Quat, b: *const Quat);
    pub fn ufbx_ffi_quat_normalize(retval: *mut Quat, q: *const Quat);
    pub fn ufbx_ffi_quat_fix_antipodal(retval: *mut Quat, q: *const Quat, reference: *const Quat);
    pub fn ufbx_ffi_quat_slerp(retval: *mut Quat, a: *const Quat, b: *const Quat, t: Real);
    pub fn ufbx_ffi_quat_rotate_vec3(retval: *mut Vec3, q: *const Quat, v: *const Vec3);
    pub fn ufbx_ffi_quat_to_euler(retval: *mut Vec3, q: *const Quat, order: RotationOrder);
    pub fn ufbx_ffi_euler_to_quat(retval: *mut Quat, v: *const Vec3, order: RotationOrder);
    pub fn ufbx_ffi_matrix_mul(retval: *mut Matrix, a: *const Matrix, b: *const Matrix);
    pub fn ufbx_ffi_matrix_invert(retval: *mut Matrix, m: *const Matrix);
    pub fn ufbx_ffi_matrix_for_normals(retval: *mut Matrix, m: *const Matrix);
    pub fn ufbx_ffi_transform_position(retval: *mut Vec3, m: *const Matrix, v: *const Vec3);
    pub fn ufbx_ffi_transform_direction(retval: *mut Vec3, m: *const Matrix, v: *const Vec3);
    pub fn ufbx_ffi_transform_to_matrix(retval: *mut Matrix, t: *const Transform);
    pub fn ufbx_ffi_matrix_to_transform(retval: *mut Transform, m: *const Matrix);
    pub fn ufbx_ffi_get_skin_vertex_matrix(retval: *mut Matrix, skin: *const SkinDeformer, vertex: usize, fallback: *const Matrix);
    pub fn ufbx_ffi_get_blend_shape_vertex_offset(retval: *mut Vec3, shape: *const BlendShape, vertex: usize);
    pub fn ufbx_ffi_get_blend_vertex_offset(retval: *mut Vec3, blend: *const BlendDeformer, vertex: usize);
    pub fn ufbx_ffi_evaluate_nurbs_curve(retval: *mut CurvePoint, curve: *const NurbsCurve, u: Real);
    pub fn ufbx_ffi_evaluate_nurbs_surface(retval: *mut SurfacePoint, surface: *const NurbsSurface, u: Real, v: Real);
    pub fn ufbx_ffi_get_weighted_face_normal(retval: *mut Vec3, positions: *const VertexVec3, face: *const Face);
    pub fn ufbx_ffi_get_triangulate_face_num_indices(face: *const Face) -> usize;
    pub fn ufbx_ffi_triangulate_face(indices: *mut u32, num_indices: usize, mesh: *const Mesh, face: *const Face) -> u32;
}
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

pub fn is_thread_safe() -> bool {
    let result = unsafe { ufbx_is_thread_safe() };
    result
}

pub unsafe fn load_memory_raw(data: &[u8], opts: &RawLoadOpts) -> Result<SceneRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_memory(data.as_ptr() as *const c_void, data.len(), opts as *const RawLoadOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(SceneRoot::new(result))
}

pub fn load_memory(data: &[u8], opts: LoadOpts) -> Result<SceneRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawLoadOpts::from_rust(&mut opts_mut);
    unsafe { load_memory_raw(data, &opts_raw) }
}

pub unsafe fn load_file_raw(filename: &str, opts: &RawLoadOpts) -> Result<SceneRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_file_len(filename.as_ptr(), filename.len(), opts as *const RawLoadOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(SceneRoot::new(result))
}

pub fn load_file(filename: &str, opts: LoadOpts) -> Result<SceneRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawLoadOpts::from_rust(&mut opts_mut);
    unsafe { load_file_raw(filename, &opts_raw) }
}

pub unsafe fn load_stdio_raw(file: *mut c_void, opts: &RawLoadOpts) -> Result<SceneRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_stdio(file as *mut c_void, opts as *const RawLoadOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(SceneRoot::new(result))
}

pub fn load_stdio(file: *mut c_void, opts: LoadOpts) -> Result<SceneRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawLoadOpts::from_rust(&mut opts_mut);
    unsafe { load_stdio_raw(file, &opts_raw) }
}

pub unsafe fn load_stdio_prefix_raw(file: *mut c_void, prefix: &[u8], opts: &RawLoadOpts) -> Result<SceneRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_stdio_prefix(file as *mut c_void, prefix.as_ptr() as *const c_void, prefix.len(), opts as *const RawLoadOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(SceneRoot::new(result))
}

pub fn load_stdio_prefix(file: *mut c_void, prefix: &[u8], opts: LoadOpts) -> Result<SceneRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawLoadOpts::from_rust(&mut opts_mut);
    unsafe { load_stdio_prefix_raw(file, prefix, &opts_raw) }
}

pub unsafe fn load_stream_raw(stream: &RawStream, opts: &RawLoadOpts) -> Result<SceneRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_stream(stream as *const RawStream, opts as *const RawLoadOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(SceneRoot::new(result))
}

pub fn load_stream(stream: Stream, opts: LoadOpts) -> Result<SceneRoot> {
    let mut stream_mut = stream;
    let stream_raw = RawStream::from_rust(&mut stream_mut);
    let mut opts_mut = opts;
    let opts_raw = RawLoadOpts::from_rust(&mut opts_mut);
    unsafe { load_stream_raw(&stream_raw, &opts_raw) }
}

pub unsafe fn load_stream_prefix_raw(stream: &RawStream, prefix: &[u8], opts: &RawLoadOpts) -> Result<SceneRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_stream_prefix(stream as *const RawStream, prefix.as_ptr() as *const c_void, prefix.len(), opts as *const RawLoadOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(SceneRoot::new(result))
}

pub fn load_stream_prefix(stream: Stream, prefix: &[u8], opts: LoadOpts) -> Result<SceneRoot> {
    let mut stream_mut = stream;
    let stream_raw = RawStream::from_rust(&mut stream_mut);
    let mut opts_mut = opts;
    let opts_raw = RawLoadOpts::from_rust(&mut opts_mut);
    unsafe { load_stream_prefix_raw(&stream_raw, prefix, &opts_raw) }
}

pub fn format_error(dst: &mut [u8], error: &Error) -> usize {
    let result = unsafe { ufbx_format_error(dst.as_mut_ptr(), dst.len(), error as *const Error) };
    result
}

pub fn find_prop<'a>(props: &'a Props, name: &str) -> Option<&'a Prop> {
    let result = unsafe { ufbx_find_prop_len(props as *const Props, name.as_ptr(), name.len()) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

// TODO: Property find functions

// TODO: Property find functions

// TODO: Property find functions

// TODO: Property find functions

// TODO: Property find functions

pub fn find_blob(props: &Props, name: &str, def: Blob) -> Blob {
    let result = unsafe { ufbx_find_blob_len(props as *const Props, name.as_ptr(), name.len(), def) };
    result
}

// TODO: ufbx_find_prop_concat()

pub fn get_prop_element<'a>(element: &'a Element, prop: &Prop, type_: ElementType) -> Option<&'a Element> {
    let result = unsafe { ufbx_get_prop_element(element as *const Element, prop as *const Prop, type_) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn find_prop_element<'a>(element: &'a Element, name: &str, type_: ElementType) -> Option<&'a Element> {
    let result = unsafe { ufbx_find_prop_element_len(element as *const Element, name.as_ptr(), name.len(), type_) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn find_element<'a>(scene: &'a Scene, type_: ElementType, name: &str) -> Option<&'a Element> {
    let result = unsafe { ufbx_find_element_len(scene as *const Scene, type_, name.as_ptr(), name.len()) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn find_node<'a>(scene: &'a Scene, name: &str) -> Option<&'a Node> {
    let result = unsafe { ufbx_find_node_len(scene as *const Scene, name.as_ptr(), name.len()) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn find_anim_stack<'a>(scene: &'a Scene, name: &str) -> Option<&'a AnimStack> {
    let result = unsafe { ufbx_find_anim_stack_len(scene as *const Scene, name.as_ptr(), name.len()) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn find_material<'a>(scene: &'a Scene, name: &str) -> Option<&'a Material> {
    let result = unsafe { ufbx_find_material_len(scene as *const Scene, name.as_ptr(), name.len()) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn find_anim_prop<'a>(layer: &'a AnimLayer, element: &'a Element, prop: &str) -> Option<&'a AnimProp> {
    let result = unsafe { ufbx_find_anim_prop_len(layer as *const AnimLayer, element as *const Element, prop.as_ptr(), prop.len()) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn find_anim_props<'a>(layer: &'a AnimLayer, element: &'a Element) -> &'a [AnimProp] {
    let result = unsafe { ufbx_find_anim_props(layer as *const AnimLayer, element as *const Element) };
    unsafe { result.as_static_ref() }
}

pub fn get_compatible_matrix_for_normals(node: &Node) -> Matrix {
    let result = unsafe { ufbx_get_compatible_matrix_for_normals(node as *const Node) };
    result
}

pub fn inflate(dst: &mut [u8], input: &InflateInput, retain: &mut InflateRetain) -> isize {
    let result = unsafe { ufbx_inflate(dst.as_mut_ptr() as *mut c_void, dst.len(), input as *const InflateInput, retain as *mut InflateRetain) };
    result
}

pub unsafe fn open_file_raw(stream: &mut RawStream, path: &str) -> bool {
    let result = { ufbx_open_file(stream as *mut RawStream, path.as_ptr(), path.len()) };
    result
}

pub unsafe fn default_open_file_raw(user: *mut c_void, stream: &mut RawStream, path: &str, info: &OpenFileInfo) -> bool {
    let result = { ufbx_default_open_file(user as *mut c_void, stream as *mut RawStream, path.as_ptr(), path.len(), info as *const OpenFileInfo) };
    result
}

pub unsafe fn open_memory_raw(stream: &mut RawStream, data: &[u8], opts: &RawOpenMemoryOpts) -> Result<bool> {
    let mut error: Error = Error::default();
    let result = { ufbx_open_memory(stream as *mut RawStream, data.as_ptr() as *const c_void, data.len(), opts as *const RawOpenMemoryOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(result)
}

pub fn evaluate_curve(curve: &AnimCurve, time: f64, default_value: Real) -> Real {
    let result = unsafe { ufbx_evaluate_curve(curve as *const AnimCurve, time, default_value) };
    result
}

pub fn evaluate_anim_value_real(anim_value: &AnimValue, time: f64) -> Real {
    let result = unsafe { ufbx_evaluate_anim_value_real(anim_value as *const AnimValue, time) };
    result
}

pub fn evaluate_anim_value_vec2(anim_value: &AnimValue, time: f64) -> Vec2 {
    let result = unsafe { ufbx_evaluate_anim_value_vec2(anim_value as *const AnimValue, time) };
    result
}

pub fn evaluate_anim_value_vec3(anim_value: &AnimValue, time: f64) -> Vec3 {
    let result = unsafe { ufbx_evaluate_anim_value_vec3(anim_value as *const AnimValue, time) };
    result
}

pub fn evaluate_prop<'a, 'b>(anim: &'a Anim, element: &'a Element, name: &'b str, time: f64) -> ExternalRef<'b, Prop>
    where 'a: 'b
{
    let result = unsafe { ufbx_evaluate_prop_len(anim as *const Anim, element as *const Element, name.as_ptr(), name.len(), time) };
    unsafe { ExternalRef::new(result) }
}

pub fn evaluate_props<'a, 'b>(anim: &'a Anim, element: &'a Element, time: f64, buffer: &'b mut [ExternalRef<'b, Prop>]) -> ExternalRef<'b, Props>
    where 'a: 'b
{
    let result = unsafe { ufbx_evaluate_props(anim as *const Anim, element as *const Element, time, buffer.as_ptr() as *mut Prop, buffer.len()) };
    unsafe { ExternalRef::new(result) }
}

pub fn evaluate_transform(anim: &Anim, node: &Node, time: f64) -> Transform {
    let result = unsafe { ufbx_evaluate_transform(anim as *const Anim, node as *const Node, time) };
    result
}

pub fn evaluate_blend_weight(anim: &Anim, channel: &BlendChannel, time: f64) -> Real {
    let result = unsafe { ufbx_evaluate_blend_weight(anim as *const Anim, channel as *const BlendChannel, time) };
    result
}

pub unsafe fn evaluate_scene_raw(scene: &Scene, anim: &Anim, time: f64, opts: &RawEvaluateOpts) -> Result<SceneRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_evaluate_scene(scene as *const Scene, anim as *const Anim, time, opts as *const RawEvaluateOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(SceneRoot::new(result))
}

pub fn evaluate_scene(scene: &Scene, anim: &Anim, time: f64, opts: EvaluateOpts) -> Result<SceneRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawEvaluateOpts::from_rust(&mut opts_mut);
    unsafe { evaluate_scene_raw(scene, anim, time, &opts_raw) }
}

pub fn create_anim(scene: &Scene, opts: &AnimOpts) -> Result<AnimRoot> {
    let mut error: Error = Error::default();
    let result = unsafe { ufbx_create_anim(scene as *const Scene, opts as *const AnimOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(AnimRoot::new(result))
}

pub fn find_prop_texture<'a>(material: &'a Material, name: &str) -> Option<&'a Texture> {
    let result = unsafe { ufbx_find_prop_texture_len(material as *const Material, name.as_ptr(), name.len()) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn find_shader_prop<'a>(shader: &'a Shader, name: &'a str) -> &'a str {
    let result = unsafe { ufbx_find_shader_prop_len(shader as *const Shader, name.as_ptr(), name.len()) };
    unsafe { result.as_static_ref() }
}

pub fn find_shader_prop_bindings<'a>(shader: &'a Shader, name: &str) -> &'a [ShaderPropBinding] {
    let result = unsafe { ufbx_find_shader_prop_bindings_len(shader as *const Shader, name.as_ptr(), name.len()) };
    unsafe { result.as_static_ref() }
}

pub fn find_shader_texture_input<'a>(shader: &ShaderTexture, name: &str) -> Option<&'a ShaderTextureInput> {
    let result = unsafe { ufbx_find_shader_texture_input_len(shader as *const ShaderTexture, name.as_ptr(), name.len()) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn coordinate_axes_valid(axes: CoordinateAxes) -> bool {
    let result = unsafe { ufbx_coordinate_axes_valid(axes) };
    result
}

pub fn quat_dot(a: Quat, b: Quat) -> Real {
    let result = unsafe { ufbx_quat_dot(a, b) };
    result
}

pub fn quat_mul(a: Quat, b: Quat) -> Quat {
    let result = unsafe { ufbx_quat_mul(a, b) };
    result
}

pub fn quat_normalize(q: Quat) -> Quat {
    let result = unsafe { ufbx_quat_normalize(q) };
    result
}

pub fn quat_fix_antipodal(q: Quat, reference: Quat) -> Quat {
    let result = unsafe { ufbx_quat_fix_antipodal(q, reference) };
    result
}

pub fn quat_slerp(a: Quat, b: Quat, t: Real) -> Quat {
    let result = unsafe { ufbx_quat_slerp(a, b, t) };
    result
}

pub fn quat_rotate_vec3(q: Quat, v: Vec3) -> Vec3 {
    let result = unsafe { ufbx_quat_rotate_vec3(q, v) };
    result
}

pub fn quat_to_euler(q: Quat, order: RotationOrder) -> Vec3 {
    let result = unsafe { ufbx_quat_to_euler(q, order) };
    result
}

pub fn euler_to_quat(v: Vec3, order: RotationOrder) -> Quat {
    let result = unsafe { ufbx_euler_to_quat(v, order) };
    result
}

pub fn matrix_mul(a: &Matrix, b: &Matrix) -> Matrix {
    let result = unsafe { ufbx_matrix_mul(a as *const Matrix, b as *const Matrix) };
    result
}

pub fn matrix_determinant(m: &Matrix) -> Real {
    let result = unsafe { ufbx_matrix_determinant(m as *const Matrix) };
    result
}

pub fn matrix_invert(m: &Matrix) -> Matrix {
    let result = unsafe { ufbx_matrix_invert(m as *const Matrix) };
    result
}

pub fn matrix_for_normals(m: &Matrix) -> Matrix {
    let result = unsafe { ufbx_matrix_for_normals(m as *const Matrix) };
    result
}

pub fn transform_position(m: &Matrix, v: Vec3) -> Vec3 {
    let result = unsafe { ufbx_transform_position(m as *const Matrix, v) };
    result
}

pub fn transform_direction(m: &Matrix, v: Vec3) -> Vec3 {
    let result = unsafe { ufbx_transform_direction(m as *const Matrix, v) };
    result
}

pub fn transform_to_matrix(t: &Transform) -> Matrix {
    let result = unsafe { ufbx_transform_to_matrix(t as *const Transform) };
    result
}

pub fn matrix_to_transform(m: &Matrix) -> Transform {
    let result = unsafe { ufbx_matrix_to_transform(m as *const Matrix) };
    result
}

pub fn get_skin_vertex_matrix(skin: &SkinDeformer, vertex: usize, fallback: &Matrix) -> Matrix {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_get_skin_vertex_matrix(&mut panic, skin as *const SkinDeformer, vertex, fallback as *const Matrix) };
    if panic.did_panic {
        panic!("ufbx::get_skin_vertex_matrix() {}", panic.message());
    }
    result
}

pub fn get_blend_shape_vertex_offset(shape: &BlendShape, vertex: usize) -> Vec3 {
    let result = unsafe { ufbx_get_blend_shape_vertex_offset(shape as *const BlendShape, vertex) };
    result
}

pub fn get_blend_vertex_offset(blend: &BlendDeformer, vertex: usize) -> Vec3 {
    let result = unsafe { ufbx_get_blend_vertex_offset(blend as *const BlendDeformer, vertex) };
    result
}

pub fn add_blend_shape_vertex_offsets(shape: &BlendShape, vertices: &mut [Vec3], weight: Real) {
    unsafe { ufbx_add_blend_shape_vertex_offsets(shape as *const BlendShape, vertices.as_mut_ptr(), vertices.len(), weight) };
}

pub fn add_blend_vertex_offsets(blend: &BlendDeformer, vertices: &mut [Vec3], weight: Real) {
    unsafe { ufbx_add_blend_vertex_offsets(blend as *const BlendDeformer, vertices.as_mut_ptr(), vertices.len(), weight) };
}

pub fn evaluate_nurbs_basis(basis: &NurbsBasis, u: Real, weights: &mut [Real], derivatives: &mut [Real]) -> usize {
    let result = unsafe { ufbx_evaluate_nurbs_basis(basis as *const NurbsBasis, u, weights.as_mut_ptr(), weights.len(), derivatives.as_mut_ptr(), derivatives.len()) };
    result
}

pub fn evaluate_nurbs_curve(curve: &NurbsCurve, u: Real) -> CurvePoint {
    let result = unsafe { ufbx_evaluate_nurbs_curve(curve as *const NurbsCurve, u) };
    result
}

pub fn evaluate_nurbs_surface(surface: &NurbsSurface, u: Real, v: Real) -> SurfacePoint {
    let result = unsafe { ufbx_evaluate_nurbs_surface(surface as *const NurbsSurface, u, v) };
    result
}

pub unsafe fn tessellate_nurbs_curve_raw(curve: &NurbsCurve, opts: &RawTessellateCurveOpts) -> Result<LineCurveRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_tessellate_nurbs_curve(curve as *const NurbsCurve, opts as *const RawTessellateCurveOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(LineCurveRoot::new(result))
}

pub fn tessellate_nurbs_curve(curve: &NurbsCurve, opts: TessellateCurveOpts) -> Result<LineCurveRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawTessellateCurveOpts::from_rust(&mut opts_mut);
    unsafe { tessellate_nurbs_curve_raw(curve, &opts_raw) }
}

pub unsafe fn tessellate_nurbs_surface_raw(surface: &NurbsSurface, opts: &RawTessellateSurfaceOpts) -> Result<MeshRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_tessellate_nurbs_surface(surface as *const NurbsSurface, opts as *const RawTessellateSurfaceOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(MeshRoot::new(result))
}

pub fn tessellate_nurbs_surface(surface: &NurbsSurface, opts: TessellateSurfaceOpts) -> Result<MeshRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawTessellateSurfaceOpts::from_rust(&mut opts_mut);
    unsafe { tessellate_nurbs_surface_raw(surface, &opts_raw) }
}

pub fn find_face_index(mesh: &mut Mesh, index: usize) -> u32 {
    let result = unsafe { ufbx_find_face_index(mesh as *mut Mesh, index) };
    result
}

pub fn triangulate_face(indices: &mut [u32], mesh: &Mesh, face: Face) -> u32 {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_triangulate_face(&mut panic, indices.as_mut_ptr(), indices.len(), mesh as *const Mesh, face) };
    if panic.did_panic {
        panic!("ufbx::triangulate_face() {}", panic.message());
    }
    result
}

pub fn compute_topology(mesh: &Mesh, topo: &mut [TopoEdge]) {
    let mut panic: Panic = Default::default();
    unsafe { ufbx_catch_compute_topology(&mut panic, mesh as *const Mesh, topo.as_mut_ptr(), topo.len()) };
    if panic.did_panic {
        panic!("ufbx::compute_topology() {}", panic.message());
    }
}

pub fn topo_next_vertex_edge(topo: &[TopoEdge], index: u32) -> u32 {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_topo_next_vertex_edge(&mut panic, topo.as_ptr(), topo.len(), index) };
    if panic.did_panic {
        panic!("ufbx::topo_next_vertex_edge() {}", panic.message());
    }
    result
}

pub fn topo_prev_vertex_edge(topo: &[TopoEdge], index: u32) -> u32 {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_topo_prev_vertex_edge(&mut panic, topo.as_ptr(), topo.len(), index) };
    if panic.did_panic {
        panic!("ufbx::topo_prev_vertex_edge() {}", panic.message());
    }
    result
}

pub fn get_weighted_face_normal(positions: &VertexVec3, face: Face) -> Vec3 {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_get_weighted_face_normal(&mut panic, positions as *const VertexVec3, face) };
    if panic.did_panic {
        panic!("ufbx::get_weighted_face_normal() {}", panic.message());
    }
    result
}

pub fn generate_normal_mapping(mesh: &Mesh, topo: &[TopoEdge], normal_indices: &mut [u32], assume_smooth: bool) -> usize {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_generate_normal_mapping(&mut panic, mesh as *const Mesh, topo.as_ptr(), topo.len(), normal_indices.as_mut_ptr(), normal_indices.len(), assume_smooth) };
    if panic.did_panic {
        panic!("ufbx::generate_normal_mapping() {}", panic.message());
    }
    result
}

pub fn compute_normals(mesh: &Mesh, positions: &VertexVec3, normal_indices: &[u32], normals: &mut [Vec3]) {
    let mut panic: Panic = Default::default();
    unsafe { ufbx_catch_compute_normals(&mut panic, mesh as *const Mesh, positions as *const VertexVec3, normal_indices.as_ptr(), normal_indices.len(), normals.as_mut_ptr(), normals.len()) };
    if panic.did_panic {
        panic!("ufbx::compute_normals() {}", panic.message());
    }
}

pub unsafe fn subdivide_mesh_raw(mesh: &Mesh, level: usize, opts: &RawSubdivideOpts) -> Result<MeshRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_subdivide_mesh(mesh as *const Mesh, level, opts as *const RawSubdivideOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(MeshRoot::new(result))
}

pub fn subdivide_mesh(mesh: &Mesh, level: usize, opts: SubdivideOpts) -> Result<MeshRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawSubdivideOpts::from_rust(&mut opts_mut);
    unsafe { subdivide_mesh_raw(mesh, level, &opts_raw) }
}

pub unsafe fn load_geometry_cache_raw(filename: &str, opts: &RawGeometryCacheOpts) -> Result<GeometryCacheRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_geometry_cache_len(filename.as_ptr(), filename.len(), opts as *const RawGeometryCacheOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(GeometryCacheRoot::new(result))
}

pub fn load_geometry_cache(filename: &str, opts: GeometryCacheOpts) -> Result<GeometryCacheRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawGeometryCacheOpts::from_rust(&mut opts_mut);
    unsafe { load_geometry_cache_raw(filename, &opts_raw) }
}

pub unsafe fn read_geometry_cache_real_raw(frame: &CacheFrame, data: &mut [Real], opts: &RawGeometryCacheDataOpts) -> usize {
    let result = { ufbx_read_geometry_cache_real(frame as *const CacheFrame, data.as_mut_ptr(), data.len(), opts as *const RawGeometryCacheDataOpts) };
    result
}

pub fn read_geometry_cache_real(frame: &CacheFrame, data: &mut [Real], opts: GeometryCacheDataOpts) -> usize {
    let mut opts_mut = opts;
    let opts_raw = RawGeometryCacheDataOpts::from_rust(&mut opts_mut);
    unsafe { read_geometry_cache_real_raw(frame, data, &opts_raw) }
}

pub unsafe fn sample_geometry_cache_real_raw(channel: &CacheChannel, time: f64, data: &mut [Real], opts: &RawGeometryCacheDataOpts) -> usize {
    let result = { ufbx_sample_geometry_cache_real(channel as *const CacheChannel, time, data.as_mut_ptr(), data.len(), opts as *const RawGeometryCacheDataOpts) };
    result
}

pub fn sample_geometry_cache_real(channel: &CacheChannel, time: f64, data: &mut [Real], opts: GeometryCacheDataOpts) -> usize {
    let mut opts_mut = opts;
    let opts_raw = RawGeometryCacheDataOpts::from_rust(&mut opts_mut);
    unsafe { sample_geometry_cache_real_raw(channel, time, data, &opts_raw) }
}

pub unsafe fn read_geometry_cache_vec3_raw(frame: &CacheFrame, data: &mut [Vec3], opts: &RawGeometryCacheDataOpts) -> usize {
    let result = { ufbx_read_geometry_cache_vec3(frame as *const CacheFrame, data.as_mut_ptr(), data.len(), opts as *const RawGeometryCacheDataOpts) };
    result
}

pub fn read_geometry_cache_vec3(frame: &CacheFrame, data: &mut [Vec3], opts: GeometryCacheDataOpts) -> usize {
    let mut opts_mut = opts;
    let opts_raw = RawGeometryCacheDataOpts::from_rust(&mut opts_mut);
    unsafe { read_geometry_cache_vec3_raw(frame, data, &opts_raw) }
}

pub unsafe fn sample_geometry_cache_vec3_raw(channel: &CacheChannel, time: f64, data: &mut [Vec3], opts: &RawGeometryCacheDataOpts) -> usize {
    let result = { ufbx_sample_geometry_cache_vec3(channel as *const CacheChannel, time, data.as_mut_ptr(), data.len(), opts as *const RawGeometryCacheDataOpts) };
    result
}

pub fn sample_geometry_cache_vec3(channel: &CacheChannel, time: f64, data: &mut [Vec3], opts: GeometryCacheDataOpts) -> usize {
    let mut opts_mut = opts;
    let opts_raw = RawGeometryCacheDataOpts::from_rust(&mut opts_mut);
    unsafe { sample_geometry_cache_vec3_raw(channel, time, data, &opts_raw) }
}

pub fn dom_find<'a>(parent: &DomNode, name: &str) -> Option<&'a DomNode> {
    let result = unsafe { ufbx_dom_find_len(parent as *const DomNode, name.as_ptr(), name.len()) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub unsafe fn generate_indices_raw(streams: &[RawVertexStream], indices: &mut [u32], allocator: &RawAllocatorOpts) -> Result<usize> {
    let mut error: Error = Error::default();
    let result = { ufbx_generate_indices(streams.as_ptr(), streams.len(), indices.as_mut_ptr(), indices.len(), allocator as *const RawAllocatorOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(result)
}

pub fn generate_indices(streams: &mut [VertexStream], indices: &mut [u32], allocator: AllocatorOpts) -> Result<usize> {
    let mut streams_mut = streams;
    let streams_raw = RawVertexStream::from_rust(&mut streams_mut);
    let mut allocator_mut = allocator;
    let allocator_raw = RawAllocatorOpts::from_rust(&mut allocator_mut);
    unsafe { generate_indices_raw(&streams_raw, indices, &allocator_raw) }
}

pub fn get_vertex_real(v: &VertexReal, index: usize) -> Real {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_get_vertex_real(&mut panic, v as *const VertexReal, index) };
    if panic.did_panic {
        panic!("ufbx::get_vertex_real() {}", panic.message());
    }
    result
}

pub fn get_vertex_vec2(v: &VertexVec2, index: usize) -> Vec2 {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_get_vertex_vec2(&mut panic, v as *const VertexVec2, index) };
    if panic.did_panic {
        panic!("ufbx::get_vertex_vec2() {}", panic.message());
    }
    result
}

pub fn get_vertex_vec3(v: &VertexVec3, index: usize) -> Vec3 {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_get_vertex_vec3(&mut panic, v as *const VertexVec3, index) };
    if panic.did_panic {
        panic!("ufbx::get_vertex_vec3() {}", panic.message());
    }
    result
}

pub fn get_vertex_vec4(v: &VertexVec4, index: usize) -> Vec4 {
    let mut panic: Panic = Default::default();
    let result = unsafe { ufbx_catch_get_vertex_vec4(&mut panic, v as *const VertexVec4, index) };
    if panic.did_panic {
        panic!("ufbx::get_vertex_vec4() {}", panic.message());
    }
    result
}

pub fn get_triangulate_face_num_indices(face: Face) -> usize {
    let result = unsafe { ufbx_get_triangulate_face_num_indices(face) };
    result
}

pub fn as_unknown<'a>(element: &'a Element) -> Option<&'a Unknown> {
    let result = unsafe { ufbx_as_unknown(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_node<'a>(element: &'a Element) -> Option<&'a Node> {
    let result = unsafe { ufbx_as_node(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_mesh<'a>(element: &'a Element) -> Option<&'a Mesh> {
    let result = unsafe { ufbx_as_mesh(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_light<'a>(element: &'a Element) -> Option<&'a Light> {
    let result = unsafe { ufbx_as_light(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_camera<'a>(element: &'a Element) -> Option<&'a Camera> {
    let result = unsafe { ufbx_as_camera(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_bone<'a>(element: &'a Element) -> Option<&'a Bone> {
    let result = unsafe { ufbx_as_bone(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_empty<'a>(element: &'a Element) -> Option<&'a Empty> {
    let result = unsafe { ufbx_as_empty(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_line_curve<'a>(element: &'a Element) -> Option<&'a LineCurve> {
    let result = unsafe { ufbx_as_line_curve(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_nurbs_curve<'a>(element: &'a Element) -> Option<&'a NurbsCurve> {
    let result = unsafe { ufbx_as_nurbs_curve(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_nurbs_surface<'a>(element: &'a Element) -> Option<&'a NurbsSurface> {
    let result = unsafe { ufbx_as_nurbs_surface(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_nurbs_trim_surface<'a>(element: &'a Element) -> Option<&'a NurbsTrimSurface> {
    let result = unsafe { ufbx_as_nurbs_trim_surface(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_nurbs_trim_boundary<'a>(element: &'a Element) -> Option<&'a NurbsTrimBoundary> {
    let result = unsafe { ufbx_as_nurbs_trim_boundary(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_procedural_geometry<'a>(element: &'a Element) -> Option<&'a ProceduralGeometry> {
    let result = unsafe { ufbx_as_procedural_geometry(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_stereo_camera<'a>(element: &'a Element) -> Option<&'a StereoCamera> {
    let result = unsafe { ufbx_as_stereo_camera(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_camera_switcher<'a>(element: &'a Element) -> Option<&'a CameraSwitcher> {
    let result = unsafe { ufbx_as_camera_switcher(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_marker<'a>(element: &'a Element) -> Option<&'a Marker> {
    let result = unsafe { ufbx_as_marker(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_lod_group<'a>(element: &'a Element) -> Option<&'a LodGroup> {
    let result = unsafe { ufbx_as_lod_group(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_skin_deformer<'a>(element: &'a Element) -> Option<&'a SkinDeformer> {
    let result = unsafe { ufbx_as_skin_deformer(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_skin_cluster<'a>(element: &'a Element) -> Option<&'a SkinCluster> {
    let result = unsafe { ufbx_as_skin_cluster(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_blend_deformer<'a>(element: &'a Element) -> Option<&'a BlendDeformer> {
    let result = unsafe { ufbx_as_blend_deformer(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_blend_channel<'a>(element: &'a Element) -> Option<&'a BlendChannel> {
    let result = unsafe { ufbx_as_blend_channel(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_blend_shape<'a>(element: &'a Element) -> Option<&'a BlendShape> {
    let result = unsafe { ufbx_as_blend_shape(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_cache_deformer<'a>(element: &'a Element) -> Option<&'a CacheDeformer> {
    let result = unsafe { ufbx_as_cache_deformer(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_cache_file<'a>(element: &'a Element) -> Option<&'a CacheFile> {
    let result = unsafe { ufbx_as_cache_file(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_material<'a>(element: &'a Element) -> Option<&'a Material> {
    let result = unsafe { ufbx_as_material(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_texture<'a>(element: &'a Element) -> Option<&'a Texture> {
    let result = unsafe { ufbx_as_texture(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_video<'a>(element: &'a Element) -> Option<&'a Video> {
    let result = unsafe { ufbx_as_video(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_shader<'a>(element: &'a Element) -> Option<&'a Shader> {
    let result = unsafe { ufbx_as_shader(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_shader_binding<'a>(element: &'a Element) -> Option<&'a ShaderBinding> {
    let result = unsafe { ufbx_as_shader_binding(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_anim_stack<'a>(element: &'a Element) -> Option<&'a AnimStack> {
    let result = unsafe { ufbx_as_anim_stack(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_anim_layer<'a>(element: &'a Element) -> Option<&'a AnimLayer> {
    let result = unsafe { ufbx_as_anim_layer(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_anim_value<'a>(element: &'a Element) -> Option<&'a AnimValue> {
    let result = unsafe { ufbx_as_anim_value(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_anim_curve<'a>(element: &'a Element) -> Option<&'a AnimCurve> {
    let result = unsafe { ufbx_as_anim_curve(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_display_layer<'a>(element: &'a Element) -> Option<&'a DisplayLayer> {
    let result = unsafe { ufbx_as_display_layer(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_selection_set<'a>(element: &'a Element) -> Option<&'a SelectionSet> {
    let result = unsafe { ufbx_as_selection_set(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_selection_node<'a>(element: &'a Element) -> Option<&'a SelectionNode> {
    let result = unsafe { ufbx_as_selection_node(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_character<'a>(element: &'a Element) -> Option<&'a Character> {
    let result = unsafe { ufbx_as_character(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_constraint<'a>(element: &'a Element) -> Option<&'a Constraint> {
    let result = unsafe { ufbx_as_constraint(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_pose<'a>(element: &'a Element) -> Option<&'a Pose> {
    let result = unsafe { ufbx_as_pose(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}

pub fn as_metadata_object<'a>(element: &'a Element) -> Option<&'a MetadataObject> {
    let result = unsafe { ufbx_as_metadata_object(element as *const Element) };
    if result.is_null() { None } else { unsafe { Some(&*result) } }
}
pub fn identity_matrix() -> Matrix { unsafe { ufbx_identity_matrix } }
pub fn identity_transform() -> Transform { unsafe { ufbx_identity_transform } }
pub fn zero_vec2() -> Vec2 { unsafe { ufbx_zero_vec2 } }
pub fn zero_vec3() -> Vec3 { unsafe { ufbx_zero_vec3 } }
pub fn zero_vec4() -> Vec4 { unsafe { ufbx_zero_vec4 } }
pub fn identity_quat() -> Quat { unsafe { ufbx_identity_quat } }
pub fn axes_right_handed_y_up() -> CoordinateAxes { unsafe { ufbx_axes_right_handed_y_up } }
pub fn axes_right_handed_z_up() -> CoordinateAxes { unsafe { ufbx_axes_right_handed_z_up } }
pub fn axes_left_handed_y_up() -> CoordinateAxes { unsafe { ufbx_axes_left_handed_y_up } }
pub fn axes_left_handed_z_up() -> CoordinateAxes { unsafe { ufbx_axes_left_handed_z_up } }
pub fn source_version() -> u32 { unsafe { ufbx_source_version } }

impl Vec2 {
    pub fn zero() -> Vec2 { unsafe { ufbx_zero_vec2 } }
}

impl Vec3 {
    pub fn zero() -> Vec3 { unsafe { ufbx_zero_vec3 } }
}

impl Vec4 {
    pub fn zero() -> Vec4 { unsafe { ufbx_zero_vec4 } }
}

impl Quat {
    pub fn identity() -> Quat { unsafe { ufbx_identity_quat } }
}

impl Transform {
    pub fn identity() -> Transform { unsafe { ufbx_identity_transform } }
}

impl Matrix {
    pub fn identity() -> Matrix { unsafe { ufbx_identity_matrix } }
}

impl DomNode {

    pub fn find<'a>(&'a self, name: &str) -> Option<&'a DomNode> {
        dom_find(&self, name)
    }
}

impl Props {

    pub fn find_prop<'a>(&'a self, name: &str) -> Option<&'a Prop> {
        find_prop(&self, name)
    }

    // TODO: find_real()

    // TODO: find_vec3()

    // TODO: find_int()

    // TODO: find_bool()

    // TODO: find_string()
}

impl Node {

    pub fn get_compatible_matrix_for_normals(&self) -> Matrix {
        get_compatible_matrix_for_normals(&self)
    }

    pub fn evaluate_transform(&self, anim: &Anim, time: f64) -> Transform {
        evaluate_transform(anim, &self, time)
    }
}

impl Mesh {

    pub fn triangulate_face(&self, indices: &mut [u32], face: Face) -> u32 {
        triangulate_face(indices, &self, face)
    }

    pub fn subdivide(&self, level: usize, opts: SubdivideOpts) -> Result<MeshRoot> {
        subdivide_mesh(&self, level, opts)
    }
}

impl CoordinateAxes {
    pub fn right_handed_y_up() -> CoordinateAxes { unsafe { ufbx_axes_right_handed_y_up } }
    pub fn right_handed_z_up() -> CoordinateAxes { unsafe { ufbx_axes_right_handed_z_up } }
    pub fn left_handed_y_up() -> CoordinateAxes { unsafe { ufbx_axes_left_handed_y_up } }
    pub fn left_handed_z_up() -> CoordinateAxes { unsafe { ufbx_axes_left_handed_z_up } }
}

impl NurbsBasis {

    pub fn evaluate(&self, u: Real, weights: &mut [Real], derivatives: &mut [Real]) -> usize {
        evaluate_nurbs_basis(&self, u, weights, derivatives)
    }
}

impl NurbsCurve {

    pub fn evaluate(&self, u: Real) -> CurvePoint {
        evaluate_nurbs_curve(&self, u)
    }

    pub fn tessellate(&self, opts: TessellateCurveOpts) -> Result<LineCurveRoot> {
        tessellate_nurbs_curve(&self, opts)
    }
}

impl NurbsSurface {

    pub fn evaluate(&self, u: Real, v: Real) -> SurfacePoint {
        evaluate_nurbs_surface(&self, u, v)
    }

    pub fn tessellate(&self, opts: TessellateSurfaceOpts) -> Result<MeshRoot> {
        tessellate_nurbs_surface(&self, opts)
    }
}

impl SkinDeformer {

    pub fn get_skin_vertex_matrix(&self, vertex: usize, fallback: &Matrix) -> Matrix {
        get_skin_vertex_matrix(&self, vertex, fallback)
    }
}

impl BlendDeformer {

    pub fn get_vertex_offset(&self, vertex: usize) -> Vec3 {
        get_blend_vertex_offset(&self, vertex)
    }

    pub fn add_vertex_offsets(&self, vertices: &mut [Vec3], weight: Real) {
        add_blend_vertex_offsets(&self, vertices, weight)
    }
}

impl BlendChannel {

    pub fn evaluate_blend_weight(&self, anim: &Anim, time: f64) -> Real {
        evaluate_blend_weight(anim, &self, time)
    }
}

impl BlendShape {

    pub fn get_vertex_offset(&self, vertex: usize) -> Vec3 {
        get_blend_shape_vertex_offset(&self, vertex)
    }

    pub fn add_vertex_offsets(&self, vertices: &mut [Vec3], weight: Real) {
        add_blend_shape_vertex_offsets(&self, vertices, weight)
    }
}

impl CacheFrame {

    pub fn read_real(&self, data: &mut [Real], opts: GeometryCacheDataOpts) -> usize {
        read_geometry_cache_real(&self, data, opts)
    }

    pub fn read_vec3(&self, data: &mut [Vec3], opts: GeometryCacheDataOpts) -> usize {
        read_geometry_cache_vec3(&self, data, opts)
    }
}

impl CacheChannel {

    pub fn sample_real(&self, time: f64, data: &mut [Real], opts: GeometryCacheDataOpts) -> usize {
        sample_geometry_cache_real(&self, time, data, opts)
    }

    pub fn sample_vec3(&self, time: f64, data: &mut [Vec3], opts: GeometryCacheDataOpts) -> usize {
        sample_geometry_cache_vec3(&self, time, data, opts)
    }
}

impl Material {

    pub fn find_prop_texture<'a>(&'a self, name: &str) -> Option<&'a Texture> {
        find_prop_texture(&self, name)
    }
}

impl Shader {

    pub fn find_shader_prop<'a>(&'a self, name: &'a str) -> &'a str {
        find_shader_prop(self, name)
    }
}

impl AnimLayer {

    pub fn find_anim_prop<'a>(&'a self, element: &'a Element, prop: &str) -> Option<&'a AnimProp> {
        find_anim_prop(&self, element, prop)
    }

    pub fn find_anim_props<'a>(&'a self, element: &'a Element) -> &'a [AnimProp] {
        find_anim_props(&self, element)
    }
}

impl AnimValue {

    pub fn evaluate_real(&self, time: f64) -> Real {
        evaluate_anim_value_real(&self, time)
    }

    pub fn evaluate_vec2(&self, time: f64) -> Vec2 {
        evaluate_anim_value_vec2(&self, time)
    }

    pub fn evaluate_vec3(&self, time: f64) -> Vec3 {
        evaluate_anim_value_vec3(&self, time)
    }
}

impl AnimCurve {

    pub fn evaluate(&self, time: f64, default_value: Real) -> Real {
        evaluate_curve(&self, time, default_value)
    }
}

impl Scene {

    pub fn find_element<'a>(&'a self, type_: ElementType, name: &str) -> Option<&'a Element> {
        find_element(&self, type_, name)
    }

    pub fn find_node<'a>(&'a self, name: &str) -> Option<&'a Node> {
        find_node(&self, name)
    }

    pub fn find_anim_stack<'a>(&'a self, name: &str) -> Option<&'a AnimStack> {
        find_anim_stack(&self, name)
    }

    pub fn evaluate(&self, anim: &Anim, time: f64, opts: EvaluateOpts) -> Result<SceneRoot> {
        evaluate_scene(&self, anim, time, opts)
    }
}

pub enum ElementData<'a> {
    Unknown(&'a Unknown),
    Node(&'a Node),
    Mesh(&'a Mesh),
    Light(&'a Light),
    Camera(&'a Camera),
    Bone(&'a Bone),
    Empty(&'a Empty),
    LineCurve(&'a LineCurve),
    NurbsCurve(&'a NurbsCurve),
    NurbsSurface(&'a NurbsSurface),
    NurbsTrimSurface(&'a NurbsTrimSurface),
    NurbsTrimBoundary(&'a NurbsTrimBoundary),
    ProceduralGeometry(&'a ProceduralGeometry),
    StereoCamera(&'a StereoCamera),
    CameraSwitcher(&'a CameraSwitcher),
    Marker(&'a Marker),
    LodGroup(&'a LodGroup),
    SkinDeformer(&'a SkinDeformer),
    SkinCluster(&'a SkinCluster),
    BlendDeformer(&'a BlendDeformer),
    BlendChannel(&'a BlendChannel),
    BlendShape(&'a BlendShape),
    CacheDeformer(&'a CacheDeformer),
    CacheFile(&'a CacheFile),
    Material(&'a Material),
    Texture(&'a Texture),
    Video(&'a Video),
    Shader(&'a Shader),
    ShaderBinding(&'a ShaderBinding),
    AnimStack(&'a AnimStack),
    AnimLayer(&'a AnimLayer),
    AnimValue(&'a AnimValue),
    AnimCurve(&'a AnimCurve),
    DisplayLayer(&'a DisplayLayer),
    SelectionSet(&'a SelectionSet),
    SelectionNode(&'a SelectionNode),
    Character(&'a Character),
    Constraint(&'a Constraint),
    Pose(&'a Pose),
    MetadataObject(&'a MetadataObject),
}

impl Element {
    pub fn as_data(&self) -> ElementData {
        unsafe {
            match self.type_ {
                ElementType::Unknown => ElementData::Unknown(&*(self as *const _ as *const Unknown)),
                ElementType::Node => ElementData::Node(&*(self as *const _ as *const Node)),
                ElementType::Mesh => ElementData::Mesh(&*(self as *const _ as *const Mesh)),
                ElementType::Light => ElementData::Light(&*(self as *const _ as *const Light)),
                ElementType::Camera => ElementData::Camera(&*(self as *const _ as *const Camera)),
                ElementType::Bone => ElementData::Bone(&*(self as *const _ as *const Bone)),
                ElementType::Empty => ElementData::Empty(&*(self as *const _ as *const Empty)),
                ElementType::LineCurve => ElementData::LineCurve(&*(self as *const _ as *const LineCurve)),
                ElementType::NurbsCurve => ElementData::NurbsCurve(&*(self as *const _ as *const NurbsCurve)),
                ElementType::NurbsSurface => ElementData::NurbsSurface(&*(self as *const _ as *const NurbsSurface)),
                ElementType::NurbsTrimSurface => ElementData::NurbsTrimSurface(&*(self as *const _ as *const NurbsTrimSurface)),
                ElementType::NurbsTrimBoundary => ElementData::NurbsTrimBoundary(&*(self as *const _ as *const NurbsTrimBoundary)),
                ElementType::ProceduralGeometry => ElementData::ProceduralGeometry(&*(self as *const _ as *const ProceduralGeometry)),
                ElementType::StereoCamera => ElementData::StereoCamera(&*(self as *const _ as *const StereoCamera)),
                ElementType::CameraSwitcher => ElementData::CameraSwitcher(&*(self as *const _ as *const CameraSwitcher)),
                ElementType::Marker => ElementData::Marker(&*(self as *const _ as *const Marker)),
                ElementType::LodGroup => ElementData::LodGroup(&*(self as *const _ as *const LodGroup)),
                ElementType::SkinDeformer => ElementData::SkinDeformer(&*(self as *const _ as *const SkinDeformer)),
                ElementType::SkinCluster => ElementData::SkinCluster(&*(self as *const _ as *const SkinCluster)),
                ElementType::BlendDeformer => ElementData::BlendDeformer(&*(self as *const _ as *const BlendDeformer)),
                ElementType::BlendChannel => ElementData::BlendChannel(&*(self as *const _ as *const BlendChannel)),
                ElementType::BlendShape => ElementData::BlendShape(&*(self as *const _ as *const BlendShape)),
                ElementType::CacheDeformer => ElementData::CacheDeformer(&*(self as *const _ as *const CacheDeformer)),
                ElementType::CacheFile => ElementData::CacheFile(&*(self as *const _ as *const CacheFile)),
                ElementType::Material => ElementData::Material(&*(self as *const _ as *const Material)),
                ElementType::Texture => ElementData::Texture(&*(self as *const _ as *const Texture)),
                ElementType::Video => ElementData::Video(&*(self as *const _ as *const Video)),
                ElementType::Shader => ElementData::Shader(&*(self as *const _ as *const Shader)),
                ElementType::ShaderBinding => ElementData::ShaderBinding(&*(self as *const _ as *const ShaderBinding)),
                ElementType::AnimStack => ElementData::AnimStack(&*(self as *const _ as *const AnimStack)),
                ElementType::AnimLayer => ElementData::AnimLayer(&*(self as *const _ as *const AnimLayer)),
                ElementType::AnimValue => ElementData::AnimValue(&*(self as *const _ as *const AnimValue)),
                ElementType::AnimCurve => ElementData::AnimCurve(&*(self as *const _ as *const AnimCurve)),
                ElementType::DisplayLayer => ElementData::DisplayLayer(&*(self as *const _ as *const DisplayLayer)),
                ElementType::SelectionSet => ElementData::SelectionSet(&*(self as *const _ as *const SelectionSet)),
                ElementType::SelectionNode => ElementData::SelectionNode(&*(self as *const _ as *const SelectionNode)),
                ElementType::Character => ElementData::Character(&*(self as *const _ as *const Character)),
                ElementType::Constraint => ElementData::Constraint(&*(self as *const _ as *const Constraint)),
                ElementType::Pose => ElementData::Pose(&*(self as *const _ as *const Pose)),
                ElementType::MetadataObject => ElementData::MetadataObject(&*(self as *const _ as *const MetadataObject)),
            }
        }
    }
}

