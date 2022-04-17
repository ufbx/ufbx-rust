use std::ffi::{c_void};
use std::{marker, result, ptr, mem, str, slice};
use std::ops::{Deref, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, FnMut, Index};
use crate::prelude::{Real, List, Ref, RefList, String, Blob, RawString, Unsafe};
use crate::prelude::{Allocator, Stream, call_open_file_cb, call_progress_cb};

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct Vec2 {
    pub x: Real,
    pub y: Real,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct Vec3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct Vec4 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub w: Real,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
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
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
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
pub enum PropType {
    Unknown = 0,
    Boolean = 1,
    Integer = 2,
    Number = 3,
    Vector = 4,
    Color = 5,
    String = 6,
    DateTime = 7,
    Translation = 8,
    Rotation = 9,
    Scaling = 10,
    Distance = 11,
    Compound = 12,
    NumPropTypes = 13,
}

impl Default for PropType {
    fn default() -> Self { Self::Unknown }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PropFlags(u32);
impl PropFlags {
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
}

impl PropFlags {
    pub fn any(self) -> bool { self.0 != 0 }
    pub fn has_any(self, bits: Self) -> bool { (self.0 & bits.0) != 0 }
    pub fn has_all(self, bits: Self) -> bool { (self.0 & bits.0) == bits.0 }
}
impl Default for PropFlags {
    fn default() -> Self { Self(0) }
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
    pub value_int: i64,
    pub value_vec3: Vec3,
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
    LodGroup = 15,
    SkinDeformer = 16,
    SkinCluster = 17,
    BlendDeformer = 18,
    BlendChannel = 19,
    BlendShape = 20,
    CacheDeformer = 21,
    CacheFile = 22,
    Material = 23,
    Texture = 24,
    Video = 25,
    Shader = 26,
    ShaderBinding = 27,
    AnimStack = 28,
    AnimLayer = 29,
    AnimValue = 30,
    AnimCurve = 31,
    DisplayLayer = 32,
    SelectionSet = 33,
    SelectionNode = 34,
    Character = 35,
    Constraint = 36,
    Pose = 37,
    MetadataObject = 38,
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
    pub bone: Option<Ref<Bone>>,
    pub attrib: Option<Ref<Element>>,
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
    pub visible: bool,
    pub is_root: bool,
    pub node_depth: u32,
}

#[repr(C)]
pub struct VertexAttrib {
    pub exists: bool,
    pub values: VoidList,
    pub indices: List<i32>,
    pub value_reals: usize,
    pub unique_per_vertex: bool,
}

#[repr(C)]
pub struct VertexReal {
    pub exists: bool,
    pub values: List<Real>,
    pub indices: List<i32>,
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
    pub indices: List<i32>,
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
    pub indices: List<i32>,
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
    pub indices: List<i32>,
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
    pub index: i32,
    pub vertex_uv: VertexVec2,
    pub vertex_tangent: VertexVec3,
    pub vertex_bitangent: VertexVec3,
}

#[repr(C)]
pub struct ColorSet {
    pub name: String,
    pub index: i32,
    pub vertex_color: VertexVec4,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct Edge {
    pub indices: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct Face {
    pub index_begin: u32,
    pub num_indices: u32,
}

#[repr(C)]
pub struct MeshMaterial {
    pub material: Option<Ref<Material>>,
    pub num_faces: usize,
    pub num_triangles: usize,
    pub face_indices: List<i32>,
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
    SharpNone = 2,
    SharpCorners = 3,
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
    pub faces: List<Face>,
    pub face_smoothing: List<bool>,
    pub face_material: List<i32>,
    pub max_face_triangles: usize,
    pub num_bad_faces: usize,
    pub edges: List<Edge>,
    pub edge_smoothing: List<bool>,
    pub edge_crease: List<Real>,
    pub vertex_indices: List<i32>,
    pub vertices: List<Vec3>,
    pub vertex_first_index: List<i32>,
    pub vertex_position: VertexVec3,
    pub vertex_normal: VertexVec3,
    pub vertex_uv: VertexVec2,
    pub vertex_tangent: VertexVec3,
    pub vertex_bitangent: VertexVec3,
    pub vertex_color: VertexVec4,
    pub vertex_crease: VertexReal,
    pub uv_sets: List<UvSet>,
    pub color_sets: List<ColorSet>,
    pub materials: List<MeshMaterial>,
    pub skinned_is_local: bool,
    pub skinned_position: VertexVec3,
    pub skinned_normal: VertexVec3,
    pub skin_deformers: RefList<SkinDeformer>,
    pub blend_deformers: RefList<BlendDeformer>,
    pub cache_deformers: RefList<CacheDeformer>,
    pub all_deformers: RefList<Element>,
    pub subdivision_preview_levels: i32,
    pub subdivision_render_levels: i32,
    pub subdivision_display_mode: SubdivisionDisplayMode,
    pub subdivision_boundary: SubdivisionBoundary,
    pub subdivision_uv_boundary: SubdivisionBoundary,
    pub subdivision_evaluated: bool,
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

#[repr(C)]
pub struct Camera {
    pub element: Element,
    pub resolution_is_pixels: bool,
    pub resolution: Vec2,
    pub field_of_view_deg: Vec2,
    pub field_of_view_tan: Vec2,
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
pub struct LineSegment {
    pub index_begin: u32,
    pub num_indices: u32,
}

#[repr(C)]
pub struct LineCurve {
    pub element: Element,
    pub color: Vec3,
    pub control_points: List<Vec3>,
    pub point_indices: List<i32>,
    pub segments: List<LineSegment>,
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
    pub span_subdivision_u: i32,
    pub span_subdivision_v: i32,
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
pub struct SkinVertex {
    pub weight_begin: u32,
    pub num_weights: u32,
    pub dq_weight: Real,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
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
    pub dq_vertices: List<i32>,
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
    pub vertices: List<i32>,
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
}

#[repr(C)]
pub struct BlendShape {
    pub element: Element,
    pub num_offsets: usize,
    pub offset_vertices: List<i32>,
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
    VertexPosition = 1,
    VertexNormal = 2,
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
    pub format: CacheFileFormat,
    pub external_cache: Option<Ref<GeometryCache>>,
}

#[repr(C)]
pub struct MaterialMap {
    pub has_value: bool,
    pub value: Vec3,
    pub value_int: i64,
    pub texture: Option<Ref<Texture>>,
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
    OslStandard = 3,
    Arnold = 4,
    BlenderPhong = 5,
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
    Metallic = 3,
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
    TransmissionPriority = 17,
    TransmissionEnableInAov = 18,
    SubsurfaceFactor = 19,
    SubsurfaceColor = 20,
    SubsurfaceRadius = 21,
    SubsurfaceScale = 22,
    SubsurfaceAnisotropy = 23,
    SubsurfaceType = 24,
    SheenFactor = 25,
    SheenColor = 26,
    SheenRoughness = 27,
    CoatFactor = 28,
    CoatColor = 29,
    CoatRoughness = 30,
    CoatIor = 31,
    CoatAnisotropy = 32,
    CoatRotation = 33,
    CoatNormal = 34,
    ThinFilmThickness = 35,
    ThinFilmIor = 36,
    EmissionFactor = 37,
    EmissionColor = 38,
    Opacity = 39,
    IndirectDiffuse = 40,
    IndirectSpecular = 41,
    NormalMap = 42,
    TangentMap = 43,
    MatteEnabled = 44,
    MatteFactor = 45,
    MatteColor = 46,
    ThinWalled = 47,
    Caustics = 48,
    ExitToBackground = 49,
    InternalReflections = 50,
}

impl Default for MaterialPbrMap {
    fn default() -> Self { Self::BaseFactor }
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
    pub metallic: MaterialMap,
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
    pub transmission_priority: MaterialMap,
    pub transmission_enable_in_aov: MaterialMap,
    pub subsurface_factor: MaterialMap,
    pub subsurface_color: MaterialMap,
    pub subsurface_radius: MaterialMap,
    pub subsurface_scale: MaterialMap,
    pub subsurface_anisotropy: MaterialMap,
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
    pub thin_film_thickness: MaterialMap,
    pub thin_film_ior: MaterialMap,
    pub emission_factor: MaterialMap,
    pub emission_color: MaterialMap,
    pub opacity: MaterialMap,
    pub indirect_diffuse: MaterialMap,
    pub indirect_specular: MaterialMap,
    pub normal_map: MaterialMap,
    pub tangent_map: MaterialMap,
    pub matte_enabled: MaterialMap,
    pub matte_factor: MaterialMap,
    pub matte_color: MaterialMap,
    pub thin_walled: MaterialMap,
    pub caustics: MaterialMap,
    pub exit_to_background: MaterialMap,
    pub internal_reflections: MaterialMap,
}

#[repr(C)]
pub struct Material {
    pub element: Element,
    pub fbx: MaterialFbxMaps,
    pub pbr: MaterialPbrMaps,
    pub shader_type: ShaderType,
    pub shader: Option<Ref<Shader>>,
    pub shading_model_name: String,
    pub textures: List<MaterialTexture>,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextureType {
    File = 0,
    Layered = 1,
    Procedural = 2,
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

#[repr(C)]
pub struct Texture {
    pub element: Element,
    pub type_: TextureType,
    pub filename: String,
    pub absolute_filename: String,
    pub relative_filename: String,
    pub content: Blob,
    pub video: Option<Ref<Video>>,
    pub layers: List<TextureLayer>,
    pub uv_set: String,
    pub wrap_u: WrapMode,
    pub wrap_v: WrapMode,
    pub transform: Transform,
    pub texture_to_uv: Matrix,
    pub uv_to_texture: Matrix,
}

#[repr(C)]
pub struct Video {
    pub element: Element,
    pub filename: String,
    pub absolute_filename: String,
    pub relative_filename: String,
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
pub struct AnimLayerDesc {
    pub layer: Ref<AnimLayer>,
    pub weight: Real,
}

#[repr(C)]
pub struct PropOverride {
    pub element_id: u32,
    pub prop_name: Ref<u8>,
    pub value: Vec3,
    pub value_str: Ref<u8>,
    pub value_int: i64,
    _internal_key: u32,
}

#[repr(C)]
pub struct Anim {
    pub layers: List<AnimLayerDesc>,
    pub prop_overrides: List<PropOverride>,
    pub ignore_connections: bool,
}

#[repr(C)]
pub struct AnimStack {
    pub element: Element,
    pub time_begin: f64,
    pub time_end: f64,
    pub layers: RefList<AnimLayer>,
    pub anim: Anim,
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
    pub anim: Anim,
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
pub struct Tangent {
    pub dx: f32,
    pub dy: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
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
    pub vertices: List<i32>,
    pub edges: List<i32>,
    pub faces: List<i32>,
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
    pub aim_up_node: Ref<Node>,
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

#[repr(C)]
pub struct Metadata {
    pub ascii: bool,
    pub version: u32,
    pub creator: String,
    pub big_endian: bool,
    pub filename: String,
    pub relative_root: String,
    pub exporter: Exporter,
    pub exporter_version: u32,
    pub scene_props: Props,
    pub original_application: Application,
    pub latest_application: Application,
    pub geometry_ignored: bool,
    pub animation_ignored: bool,
    pub embedded_ignored: bool,
    pub max_face_triangles: usize,
    pub result_memory_used: usize,
    pub temp_memory_used: usize,
    pub result_allocs: usize,
    pub temp_allocs: usize,
    pub element_buffer_size: usize,
    pub bone_prop_size_unit: Real,
    pub bone_prop_limb_length_relative: bool,
    pub ktime_to_sec: f64,
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
pub struct CoordinateAxes {
    pub right: CoordinateAxis,
    pub up: CoordinateAxis,
    pub front: CoordinateAxis,
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
    pub anim: Anim,
    pub combined_anim: Anim,
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
    pub elements: RefList<Element>,
    pub connections_src: List<Connection>,
    pub connections_dst: List<Connection>,
    pub elements_by_name: List<NameElement>,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct CurvePoint {
    pub valid: bool,
    pub position: Vec3,
    pub derivative: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
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
    pub const NON_MANIFOLD: TopoFlags = TopoFlags(0x1);
}

impl TopoFlags {
    pub fn any(self) -> bool { self.0 != 0 }
    pub fn has_any(self, bits: Self) -> bool { (self.0 & bits.0) != 0 }
    pub fn has_all(self, bits: Self) -> bool { (self.0 & bits.0) == bits.0 }
}
impl Default for TopoFlags {
    fn default() -> Self { Self(0) }
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
pub struct TopoEdge {
    pub index: i32,
    pub next: i32,
    pub prev: i32,
    pub twin: i32,
    pub face: i32,
    pub edge: i32,
    pub flags: TopoFlags,
}

#[repr(C)]
pub struct VertexStream {
    pub data: *const c_void,
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawOpenFileCb {
    pub fn_: Option<unsafe extern "C" fn (*mut c_void, *mut RawStream, *const u8, usize) -> bool>,
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
    OutOfMemory = 3,
    MemoryLimit = 4,
    AllocationLimit = 5,
    TruncatedFile = 6,
    Io = 7,
    Cancelled = 8,
    UnsupportedVersion = 9,
    NotFbx = 10,
    UninitializedOptions = 11,
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
}

#[repr(C)]
pub struct InflateRetain {
    pub initialized: bool,
    pub data: [u64; 512],
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
    pub evaluate_skinning: bool,
    pub evaluate_caches: bool,
    pub load_external_files: bool,
    pub skip_skin_vertices: bool,
    pub disable_quirks: bool,
    pub strict: bool,
    pub allow_out_of_bounds_vertex_indices: bool,
    pub connect_broken_elements: bool,
    pub allow_nodes_out_of_root: bool,
    pub allow_null_material: bool,
    pub generate_missing_normals: bool,
    pub file_size_estimate: u64,
    pub read_buffer_size: usize,
    pub filename: RawString,
    pub progress_cb: RawProgressCb,
    pub progress_interval_hint: u64,
    pub open_file_cb: RawOpenFileCb,
    pub target_axes: CoordinateAxes,
    pub target_unit_meters: Real,
    pub no_prop_unit_scaling: bool,
    pub no_anim_curve_unit_scaling: bool,
    pub use_root_transform: bool,
    pub root_transform: Transform,
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
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct RawTessellateOpts {
    pub _begin_zero: u32,
    pub temp_allocator: RawAllocatorOpts,
    pub result_allocator: RawAllocatorOpts,
    pub span_subdivision_u: i32,
    pub span_subdivision_v: i32,
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
pub struct Panic {
    pub did_panic: bool,
    pub message_length: usize,
    pub message: [u8; 128],
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
    fn from_rust(arg: &mut AllocatorOpts) -> Self {
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
    Mut(&'a mut dyn FnMut(&str) -> Option<Stream>),
    Ref(&'a dyn Fn(&str) -> Option<Stream>),
    Raw(Unsafe<RawOpenFileCb>),
}

impl<'a> Default for OpenFileCb<'a> {
    fn default() -> Self { Self::None }
}

impl RawOpenFileCb {
    fn from_func<F: FnMut(&str) -> Option<Stream>>(arg: &mut F) -> Self {
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
    pub evaluate_skinning: bool,
    pub evaluate_caches: bool,
    pub load_external_files: bool,
    pub skip_skin_vertices: bool,
    pub disable_quirks: bool,
    pub strict: bool,
    pub allow_out_of_bounds_vertex_indices: bool,
    pub connect_broken_elements: bool,
    pub allow_nodes_out_of_root: bool,
    pub allow_null_material: bool,
    pub generate_missing_normals: bool,
    pub file_size_estimate: u64,
    pub read_buffer_size: usize,
    pub filename: Option<&'a str>,
    pub progress_cb: ProgressCb<'a>,
    pub progress_interval_hint: u64,
    pub open_file_cb: OpenFileCb<'a>,
    pub target_axes: CoordinateAxes,
    pub target_unit_meters: Real,
    pub no_prop_unit_scaling: bool,
    pub no_anim_curve_unit_scaling: bool,
    pub use_root_transform: bool,
    pub root_transform: Transform,
}

impl RawLoadOpts {
    fn from_rust(arg: &mut LoadOpts) -> Self {
        RawLoadOpts {
            _begin_zero: 0,
            temp_allocator: RawAllocatorOpts::from_rust(&mut arg.temp_allocator),
            result_allocator: RawAllocatorOpts::from_rust(&mut arg.result_allocator),
            ignore_geometry: arg.ignore_geometry,
            ignore_animation: arg.ignore_animation,
            ignore_embedded: arg.ignore_embedded,
            evaluate_skinning: arg.evaluate_skinning,
            evaluate_caches: arg.evaluate_caches,
            load_external_files: arg.load_external_files,
            skip_skin_vertices: arg.skip_skin_vertices,
            disable_quirks: arg.disable_quirks,
            strict: arg.strict,
            allow_out_of_bounds_vertex_indices: arg.allow_out_of_bounds_vertex_indices,
            connect_broken_elements: arg.connect_broken_elements,
            allow_nodes_out_of_root: arg.allow_nodes_out_of_root,
            allow_null_material: arg.allow_null_material,
            generate_missing_normals: arg.generate_missing_normals,
            file_size_estimate: arg.file_size_estimate,
            read_buffer_size: arg.read_buffer_size,
            filename: RawString::from_rust(&mut arg.filename),
            progress_cb: RawProgressCb::from_rust(&mut arg.progress_cb),
            progress_interval_hint: arg.progress_interval_hint,
            open_file_cb: RawOpenFileCb::from_rust(&mut arg.open_file_cb),
            target_axes: arg.target_axes,
            target_unit_meters: arg.target_unit_meters,
            no_prop_unit_scaling: arg.no_prop_unit_scaling,
            no_anim_curve_unit_scaling: arg.no_anim_curve_unit_scaling,
            use_root_transform: arg.use_root_transform,
            root_transform: arg.root_transform,
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
    fn from_rust(arg: &mut EvaluateOpts) -> Self {
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
pub struct TessellateOpts {
    pub temp_allocator: AllocatorOpts,
    pub result_allocator: AllocatorOpts,
    pub span_subdivision_u: i32,
    pub span_subdivision_v: i32,
}

impl RawTessellateOpts {
    fn from_rust(arg: &mut TessellateOpts) -> Self {
        RawTessellateOpts {
            _begin_zero: 0,
            temp_allocator: RawAllocatorOpts::from_rust(&mut arg.temp_allocator),
            result_allocator: RawAllocatorOpts::from_rust(&mut arg.result_allocator),
            span_subdivision_u: arg.span_subdivision_u,
            span_subdivision_v: arg.span_subdivision_v,
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
}

impl RawSubdivideOpts {
    fn from_rust(arg: &mut SubdivideOpts) -> Self {
        RawSubdivideOpts {
            _begin_zero: 0,
            temp_allocator: RawAllocatorOpts::from_rust(&mut arg.temp_allocator),
            result_allocator: RawAllocatorOpts::from_rust(&mut arg.result_allocator),
            boundary: arg.boundary,
            uv_boundary: arg.uv_boundary,
            ignore_normals: arg.ignore_normals,
            interpolate_normals: arg.interpolate_normals,
            interpolate_tangents: arg.interpolate_tangents,
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
    fn from_rust(arg: &mut GeometryCacheOpts) -> Self {
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
    fn from_rust(arg: &mut GeometryCacheDataOpts) -> Self {
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
    pub fn ufbx_find_element_len(scene: *const Scene, type_: ElementType, name: *const u8, name_len: usize) -> *mut Element;
    pub fn ufbx_find_node_len(scene: *const Scene, name: *const u8, name_len: usize) -> *mut Node;
    pub fn ufbx_find_anim_stack_len(scene: *const Scene, name: *const u8, name_len: usize) -> *mut AnimStack;
    pub fn ufbx_find_anim_prop_len(layer: *const AnimLayer, element: *const Element, prop: *const u8, prop_len: usize) -> *mut AnimProp;
    pub fn ufbx_find_anim_props(layer: *const AnimLayer, element: *const Element) -> List<AnimProp>;
    pub fn ufbx_get_compatible_matrix_for_normals(node: *const Node) -> Matrix;
    pub fn ufbx_inflate(dst: *mut c_void, dst_size: usize, input: *const InflateInput, retain: *mut InflateRetain) -> isize;
    pub fn ufbx_open_file(user: *mut c_void, stream: *mut RawStream, path: *const u8, path_len: usize) -> bool;
    pub fn ufbx_evaluate_curve(curve: *const AnimCurve, time: f64, default_value: Real) -> Real;
    pub fn ufbx_evaluate_anim_value_real(anim_value: *const AnimValue, time: f64) -> Real;
    pub fn ufbx_evaluate_anim_value_vec2(anim_value: *const AnimValue, time: f64) -> Vec2;
    pub fn ufbx_evaluate_anim_value_vec3(anim_value: *const AnimValue, time: f64) -> Vec3;
    pub fn ufbx_evaluate_prop_len(anim: *const Anim, element: *const Element, name: *const u8, name_len: usize, time: f64) -> Prop;
    pub fn ufbx_evaluate_props(anim: *const Anim, element: *const Element, time: f64, buffer: *mut Prop, buffer_size: usize) -> Props;
    pub fn ufbx_evaluate_transform(anim: *const Anim, node: *const Node, time: f64) -> Transform;
    pub fn ufbx_evaluate_blend_weight(anim: *const Anim, channel: *const BlendChannel, time: f64) -> Real;
    pub fn ufbx_prepare_prop_overrides(overrides: *mut PropOverride, num_overrides: usize) -> List<PropOverride>;
    pub fn ufbx_evaluate_scene(scene: *const Scene, anim: *const Anim, time: f64, opts: *const RawEvaluateOpts, error: *mut Error) -> *mut Scene;
    pub fn ufbx_find_prop_texture_len(material: *const Material, name: *const u8, name_len: usize) -> *mut Texture;
    pub fn ufbx_find_shader_prop_len(shader: *const Shader, name: *const u8, name_len: usize) -> String;
    pub fn ufbx_coordinate_axes_valid(axes: CoordinateAxes) -> bool;
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
    pub fn ufbx_tessellate_nurbs_surface(surface: *const NurbsSurface, opts: *const RawTessellateOpts, error: *mut Error) -> *mut Mesh;
    pub fn ufbx_catch_triangulate_face(panic: *mut Panic, indices: *mut u32, num_indices: usize, mesh: *const Mesh, face: Face) -> u32;
    pub fn ufbx_catch_compute_topology(panic: *mut Panic, mesh: *const Mesh, topo: *mut TopoEdge, num_topo: usize);
    pub fn ufbx_catch_topo_next_vertex_edge(panic: *mut Panic, topo: *const TopoEdge, num_topo: usize, index: i32) -> i32;
    pub fn ufbx_catch_topo_prev_vertex_edge(panic: *mut Panic, topo: *const TopoEdge, num_topo: usize, index: i32) -> i32;
    pub fn ufbx_catch_get_weighted_face_normal(panic: *mut Panic, positions: *const VertexVec3, face: Face) -> Vec3;
    pub fn ufbx_catch_generate_normal_mapping(panic: *mut Panic, mesh: *const Mesh, topo: *const TopoEdge, num_topo: usize, normal_indices: *mut i32, num_normal_indices: usize, assume_smooth: bool) -> usize;
    pub fn ufbx_generate_normal_mapping(mesh: *const Mesh, topo: *const TopoEdge, num_topo: usize, normal_indices: *mut i32, num_normal_indices: usize, assume_smooth: bool) -> usize;
    pub fn ufbx_catch_compute_normals(panic: *mut Panic, mesh: *const Mesh, positions: *const VertexVec3, normal_indices: *const i32, num_normal_indices: usize, normals: *mut Vec3, num_normals: usize);
    pub fn ufbx_compute_normals(mesh: *const Mesh, positions: *const VertexVec3, normal_indices: *const i32, num_normal_indices: usize, normals: *mut Vec3, num_normals: usize);
    pub fn ufbx_subdivide_mesh(mesh: *const Mesh, level: usize, opts: *const RawSubdivideOpts, error: *mut Error) -> *mut Mesh;
    pub fn ufbx_free_mesh(mesh: *mut Mesh);
    pub fn ufbx_retain_mesh(mesh: *mut Mesh);
    pub fn ufbx_load_geometry_cache(filename: *const u8, opts: *const RawGeometryCacheOpts, error: *mut Error) -> *mut GeometryCache;
    pub fn ufbx_load_geometry_cache_len(filename: *const u8, filename_len: usize, opts: *const RawGeometryCacheOpts, error: *mut Error) -> *mut GeometryCache;
    pub fn ufbx_free_geometry_cache(cache: *mut GeometryCache);
    pub fn ufbx_retain_geometry_cache(cache: *mut GeometryCache);
    pub fn ufbx_get_read_geometry_cache_real_num_data(frame: *const CacheFrame) -> usize;
    pub fn ufbx_get_sample_geometry_cache_real_num_data(channel: *const CacheChannel, time: f64) -> usize;
    pub fn ufbx_get_read_geometry_cache_vec3_num_data(frame: *const CacheFrame) -> usize;
    pub fn ufbx_get_sample_geometry_cache_vec3_num_data(channel: *const CacheChannel, time: f64) -> usize;
    pub fn ufbx_read_geometry_cache_real(frame: *const CacheFrame, data: *mut Real, num_data: usize, opts: *const RawGeometryCacheDataOpts) -> usize;
    pub fn ufbx_sample_geometry_cache_real(channel: *const CacheChannel, time: f64, data: *mut Real, num_data: usize, opts: *const RawGeometryCacheDataOpts) -> usize;
    pub fn ufbx_read_geometry_cache_vec3(frame: *const CacheFrame, data: *mut Vec3, num_data: usize, opts: *const RawGeometryCacheDataOpts) -> usize;
    pub fn ufbx_sample_geometry_cache_vec3(channel: *const CacheChannel, time: f64, data: *mut Vec3, num_data: usize, opts: *const RawGeometryCacheDataOpts) -> usize;
    pub fn ufbx_generate_indices(streams: *const VertexStream, num_streams: usize, indices: *mut u32, num_indices: usize, allocator: *const RawAllocatorOpts, error: *mut Error) -> usize;
    pub fn ufbx_catch_get_vertex_real(panic: *mut Panic, v: *const VertexReal, index: usize) -> Real;
    pub fn ufbx_catch_get_vertex_vec2(panic: *mut Panic, v: *const VertexVec2, index: usize) -> Vec2;
    pub fn ufbx_catch_get_vertex_vec3(panic: *mut Panic, v: *const VertexVec3, index: usize) -> Vec3;
    pub fn ufbx_catch_get_vertex_vec4(panic: *mut Panic, v: *const VertexVec4, index: usize) -> Vec4;
    pub fn ufbx_get_triangulate_face_num_indices(face: Face) -> usize;
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
    pub fn ufbx_ffi_prepare_prop_overrides(retval: *mut List<PropOverride>, overrides: *mut PropOverride, num_overrides: usize);
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

pub unsafe fn load_file_raw(filename: &u8, opts: &RawLoadOpts) -> Result<SceneRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_file(filename as *const u8, opts as *const RawLoadOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(SceneRoot::new(result))
}

pub fn load_file(filename: &u8, opts: LoadOpts) -> Result<SceneRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawLoadOpts::from_rust(&mut opts_mut);
    unsafe { load_file_raw(filename, &opts_raw) }
}

pub unsafe fn load_file_len_raw(filename: &str, opts: &RawLoadOpts) -> Result<SceneRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_file_len(filename.as_ptr(), filename.len(), opts as *const RawLoadOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(SceneRoot::new(result))
}

pub fn load_file_len(filename: &str, opts: LoadOpts) -> Result<SceneRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawLoadOpts::from_rust(&mut opts_mut);
    unsafe { load_file_len_raw(filename, &opts_raw) }
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

pub fn find_prop_len<'a>(props: &Props, name: &str) -> &'a Prop {
    let result = unsafe { ufbx_find_prop_len(props as *const Props, name.as_ptr(), name.len()) };
    unsafe { &*result }
}

pub fn find_real_len(props: &Props, name: &str, def: Real) -> Real {
    let result = unsafe { ufbx_find_real_len(props as *const Props, name.as_ptr(), name.len(), def) };
    result
}

pub fn find_vec3_len(props: &Props, name: &str, def: Vec3) -> Vec3 {
    let result = unsafe { ufbx_find_vec3_len(props as *const Props, name.as_ptr(), name.len(), def) };
    result
}

pub fn find_int_len(props: &Props, name: &str, def: i64) -> i64 {
    let result = unsafe { ufbx_find_int_len(props as *const Props, name.as_ptr(), name.len(), def) };
    result
}

pub fn find_bool_len(props: &Props, name: &str, def: bool) -> bool {
    let result = unsafe { ufbx_find_bool_len(props as *const Props, name.as_ptr(), name.len(), def) };
    result
}

pub fn find_string_len(props: &Props, name: &str, def: String) -> String {
    let result = unsafe { ufbx_find_string_len(props as *const Props, name.as_ptr(), name.len(), def) };
    result
}

pub fn find_element_len<'a>(scene: &'a Scene, type_: ElementType, name: &str) -> &'a Element {
    let result = unsafe { ufbx_find_element_len(scene as *const Scene, type_, name.as_ptr(), name.len()) };
    unsafe { &*result }
}

pub fn find_node_len<'a>(scene: &'a Scene, name: &str) -> &'a Node {
    let result = unsafe { ufbx_find_node_len(scene as *const Scene, name.as_ptr(), name.len()) };
    unsafe { &*result }
}

pub fn find_anim_stack_len<'a>(scene: &'a Scene, name: &str) -> &'a AnimStack {
    let result = unsafe { ufbx_find_anim_stack_len(scene as *const Scene, name.as_ptr(), name.len()) };
    unsafe { &*result }
}

pub fn find_anim_prop_len<'a>(layer: &'a AnimLayer, element: &'a Element, prop: &str) -> &'a AnimProp {
    let result = unsafe { ufbx_find_anim_prop_len(layer as *const AnimLayer, element as *const Element, prop.as_ptr(), prop.len()) };
    unsafe { &*result }
}

pub fn find_anim_props(layer: &AnimLayer, element: &Element) -> List<AnimProp> {
    let result = unsafe { ufbx_find_anim_props(layer as *const AnimLayer, element as *const Element) };
    result
}

pub fn get_compatible_matrix_for_normals(node: &Node) -> Matrix {
    let result = unsafe { ufbx_get_compatible_matrix_for_normals(node as *const Node) };
    result
}

pub fn inflate(dst: &mut [u8], input: &InflateInput, retain: &mut InflateRetain) -> isize {
    let result = unsafe { ufbx_inflate(dst.as_mut_ptr() as *mut c_void, dst.len(), input as *const InflateInput, retain as *mut InflateRetain) };
    result
}

pub unsafe fn open_file_raw(user: *mut c_void, stream: &mut RawStream, path: &str) -> bool {
    let result = { ufbx_open_file(user as *mut c_void, stream as *mut RawStream, path.as_ptr(), path.len()) };
    result
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

pub fn evaluate_prop_len(anim: &Anim, element: &Element, name: &str, time: f64) -> Prop {
    let result = unsafe { ufbx_evaluate_prop_len(anim as *const Anim, element as *const Element, name.as_ptr(), name.len(), time) };
    result
}

pub fn evaluate_props(anim: &Anim, element: &Element, time: f64, buffer: &mut Prop, buffer_size: usize) -> Props {
    let result = unsafe { ufbx_evaluate_props(anim as *const Anim, element as *const Element, time, buffer as *mut Prop, buffer_size) };
    result
}

pub fn evaluate_transform(anim: &Anim, node: &Node, time: f64) -> Transform {
    let result = unsafe { ufbx_evaluate_transform(anim as *const Anim, node as *const Node, time) };
    result
}

pub fn evaluate_blend_weight(anim: &Anim, channel: &BlendChannel, time: f64) -> Real {
    let result = unsafe { ufbx_evaluate_blend_weight(anim as *const Anim, channel as *const BlendChannel, time) };
    result
}

pub fn prepare_prop_overrides(overrides: &mut [PropOverride]) -> List<PropOverride> {
    let result = unsafe { ufbx_prepare_prop_overrides(overrides.as_mut_ptr(), overrides.len()) };
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

pub fn find_prop_texture_len<'a>(material: &'a Material, name: &str) -> &'a Texture {
    let result = unsafe { ufbx_find_prop_texture_len(material as *const Material, name.as_ptr(), name.len()) };
    unsafe { &*result }
}

pub fn find_shader_prop_len(shader: &Shader, name: &str) -> String {
    let result = unsafe { ufbx_find_shader_prop_len(shader as *const Shader, name.as_ptr(), name.len()) };
    result
}

pub fn coordinate_axes_valid(axes: CoordinateAxes) -> bool {
    let result = unsafe { ufbx_coordinate_axes_valid(axes) };
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
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_get_skin_vertex_matrix(&mut panic, skin as *const SkinDeformer, vertex, fallback as *const Matrix) };
    if panic.did_panic {
        panic!("ufbx::get_skin_vertex_matrix() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
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

pub unsafe fn tessellate_nurbs_surface_raw(surface: &NurbsSurface, opts: &RawTessellateOpts) -> Result<MeshRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_tessellate_nurbs_surface(surface as *const NurbsSurface, opts as *const RawTessellateOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(MeshRoot::new(result))
}

pub fn tessellate_nurbs_surface(surface: &NurbsSurface, opts: TessellateOpts) -> Result<MeshRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawTessellateOpts::from_rust(&mut opts_mut);
    unsafe { tessellate_nurbs_surface_raw(surface, &opts_raw) }
}

pub fn triangulate_face(indices: &mut [u32], mesh: &Mesh, face: Face) -> u32 {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_triangulate_face(&mut panic, indices.as_mut_ptr(), indices.len(), mesh as *const Mesh, face) };
    if panic.did_panic {
        panic!("ufbx::triangulate_face() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
    result
}

pub fn compute_topology(mesh: &Mesh, topo: &mut [TopoEdge]) {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    unsafe { ufbx_catch_compute_topology(&mut panic, mesh as *const Mesh, topo.as_mut_ptr(), topo.len()) };
    if panic.did_panic {
        panic!("ufbx::compute_topology() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
}

pub fn topo_next_vertex_edge(topo: &[TopoEdge], index: i32) -> i32 {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_topo_next_vertex_edge(&mut panic, topo.as_ptr(), topo.len(), index) };
    if panic.did_panic {
        panic!("ufbx::topo_next_vertex_edge() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
    result
}

pub fn topo_prev_vertex_edge(topo: &[TopoEdge], index: i32) -> i32 {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_topo_prev_vertex_edge(&mut panic, topo.as_ptr(), topo.len(), index) };
    if panic.did_panic {
        panic!("ufbx::topo_prev_vertex_edge() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
    result
}

pub fn get_weighted_face_normal(positions: &VertexVec3, face: Face) -> Vec3 {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_get_weighted_face_normal(&mut panic, positions as *const VertexVec3, face) };
    if panic.did_panic {
        panic!("ufbx::get_weighted_face_normal() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
    result
}

pub fn generate_normal_mapping(mesh: &Mesh, topo: &[TopoEdge], normal_indices: &mut [i32], assume_smooth: bool) -> usize {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_generate_normal_mapping(&mut panic, mesh as *const Mesh, topo.as_ptr(), topo.len(), normal_indices.as_mut_ptr(), normal_indices.len(), assume_smooth) };
    if panic.did_panic {
        panic!("ufbx::generate_normal_mapping() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
    result
}

pub fn compute_normals(mesh: &Mesh, positions: &VertexVec3, normal_indices: &[i32], normals: &mut [Vec3]) {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    unsafe { ufbx_catch_compute_normals(&mut panic, mesh as *const Mesh, positions as *const VertexVec3, normal_indices.as_ptr(), normal_indices.len(), normals.as_mut_ptr(), normals.len()) };
    if panic.did_panic {
        panic!("ufbx::compute_normals() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
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

pub unsafe fn load_geometry_cache_raw(filename: &u8, opts: &RawGeometryCacheOpts) -> Result<GeometryCacheRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_geometry_cache(filename as *const u8, opts as *const RawGeometryCacheOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(GeometryCacheRoot::new(result))
}

pub fn load_geometry_cache(filename: &u8, opts: GeometryCacheOpts) -> Result<GeometryCacheRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawGeometryCacheOpts::from_rust(&mut opts_mut);
    unsafe { load_geometry_cache_raw(filename, &opts_raw) }
}

pub unsafe fn load_geometry_cache_len_raw(filename: &str, opts: &RawGeometryCacheOpts) -> Result<GeometryCacheRoot> {
    let mut error: Error = Error::default();
    let result = { ufbx_load_geometry_cache_len(filename.as_ptr(), filename.len(), opts as *const RawGeometryCacheOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(GeometryCacheRoot::new(result))
}

pub fn load_geometry_cache_len(filename: &str, opts: GeometryCacheOpts) -> Result<GeometryCacheRoot> {
    let mut opts_mut = opts;
    let opts_raw = RawGeometryCacheOpts::from_rust(&mut opts_mut);
    unsafe { load_geometry_cache_len_raw(filename, &opts_raw) }
}

pub fn get_read_geometry_cache_real_num_data(frame: &CacheFrame) -> usize {
    let result = unsafe { ufbx_get_read_geometry_cache_real_num_data(frame as *const CacheFrame) };
    result
}

pub fn get_sample_geometry_cache_real_num_data(channel: &CacheChannel, time: f64) -> usize {
    let result = unsafe { ufbx_get_sample_geometry_cache_real_num_data(channel as *const CacheChannel, time) };
    result
}

pub fn get_read_geometry_cache_vec3_num_data(frame: &CacheFrame) -> usize {
    let result = unsafe { ufbx_get_read_geometry_cache_vec3_num_data(frame as *const CacheFrame) };
    result
}

pub fn get_sample_geometry_cache_vec3_num_data(channel: &CacheChannel, time: f64) -> usize {
    let result = unsafe { ufbx_get_sample_geometry_cache_vec3_num_data(channel as *const CacheChannel, time) };
    result
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

pub unsafe fn generate_indices_raw(streams: &[VertexStream], indices: &mut [u32], allocator: &RawAllocatorOpts) -> Result<usize> {
    let mut error: Error = Error::default();
    let result = { ufbx_generate_indices(streams.as_ptr(), streams.len(), indices.as_mut_ptr(), indices.len(), allocator as *const RawAllocatorOpts, &mut error) };
    if error.type_ != ErrorType::None {
        return Err(error)
    }
    Ok(result)
}

pub fn generate_indices(streams: &[VertexStream], indices: &mut [u32], allocator: AllocatorOpts) -> Result<usize> {
    let mut allocator_mut = allocator;
    let allocator_raw = RawAllocatorOpts::from_rust(&mut allocator_mut);
    unsafe { generate_indices_raw(streams, indices, &allocator_raw) }
}

pub fn get_vertex_real(v: &VertexReal, index: usize) -> Real {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_get_vertex_real(&mut panic, v as *const VertexReal, index) };
    if panic.did_panic {
        panic!("ufbx::get_vertex_real() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
    result
}

pub fn get_vertex_vec2(v: &VertexVec2, index: usize) -> Vec2 {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_get_vertex_vec2(&mut panic, v as *const VertexVec2, index) };
    if panic.did_panic {
        panic!("ufbx::get_vertex_vec2() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
    result
}

pub fn get_vertex_vec3(v: &VertexVec3, index: usize) -> Vec3 {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_get_vertex_vec3(&mut panic, v as *const VertexVec3, index) };
    if panic.did_panic {
        panic!("ufbx::get_vertex_vec3() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
    result
}

pub fn get_vertex_vec4(v: &VertexVec4, index: usize) -> Vec4 {
    let mut panic: Panic = Panic{
        did_panic: false,
        message_length: 0,
        message: unsafe { mem::MaybeUninit::uninit().assume_init() },
    };
    let result = unsafe { ufbx_catch_get_vertex_vec4(&mut panic, v as *const VertexVec4, index) };
    if panic.did_panic {
        panic!("ufbx::get_vertex_vec4() {}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts(&panic.message as *const _, panic.message_length)) });
    }
    result
}

pub fn get_triangulate_face_num_indices(face: Face) -> usize {
    let result = unsafe { ufbx_get_triangulate_face_num_indices(face) };
    result
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

