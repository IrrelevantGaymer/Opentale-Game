#[derive(Clone, Copy)]
pub enum MeshType {
    /// Used for opaque blocks like stone and dirt,
    /// and can also be used for opaque blocks
    /// with custom models or are billboards
    Normal,
    /// Used for transparent blocks like glass
    Transparent,
    /// Used for objects with complex models that
    /// could be instanced to reduce draw calls and
    /// meshing complexity/times,
    Instance
}