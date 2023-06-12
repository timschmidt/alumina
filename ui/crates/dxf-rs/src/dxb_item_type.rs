enum_from_primitive! {
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DxbItemType {
    EOF = 0,
    Line = 1,
    Point = 2,
    Circle = 3,
    Arc = 8,
    Trace = 9,
    Solid = 11,
    Seqend = 17,
    Polyline = 19,
    Vertex = 20,
    Line3D = 21,
    Face = 22,
    ScaleFactor = 128,
    NewLayer = 129,
    LineExtension = 130,
    TraceExtension = 131,
    BlockBase = 132,
    Bulge = 133,
    Width = 134,
    NumberMode = 135,
    NewColor = 136,
    LineExtension3D = 137,
}
}
