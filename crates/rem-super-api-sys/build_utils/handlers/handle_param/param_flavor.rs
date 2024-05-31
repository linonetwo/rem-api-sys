#[derive(Clone, Debug)]
pub enum ParameterFlavor {
    /// return method call style parameter code
    MethodCallParam,
    /// return rust style parameter code
    Rust,
    /// null check for unsafe c ffi code
    UnsafeCheck,
    /// each param as field in rust struct
    RustStruct,
    ///
    SpiFn,
    /// only add debug log
    None,
}
