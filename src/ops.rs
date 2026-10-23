use crate::types::JeffType;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JeffOpKind {
    QubitAlloc,
    QubitFree,
    QubitFreeZero,
    QubitMeasure,
    QubitMeasureNd,
    QubitReset,
    GPhase,
    I,
    X,
    Y,
    Z,
    S,
    T,
    R1,
    Rx,
    Ry,
    Rz,
    H,
    U,
    Swap,
    Ppr,
    Custom,
    QuregAlloc,
    QuregFree,
    QuregFreeZero,
    QuregExtractIndex,
    QuregInsertIndex,
    QuregExtractSlice,
    QuregInsertSlice,
    QuregLength,
    QuregSplit,
    QuregJoin,
    QuregCreate,
    IntConst,
    IntAdd,
    IntSub,
    IntMul,
    IntDiv,
    IntCmp,
    IntBitAnd,
    IntBitOr,
    IntBitXor,
    IntSelect,
    FloatConst,
    FloatAdd,
    FloatMul,
    FloatCmp,
    FuncCall,
    ScfSwitch,
    ScfFor,
    ScfWhile,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JeffValue {
    pub ty: JeffType,
    pub name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JeffOp {
    pub kind: JeffOpKind,
    pub inputs: Vec<JeffValue>,
    pub outputs: Vec<JeffValue>,
    pub metadata: Vec<crate::attributes::JeffMeta>,
}

impl JeffOp {
    pub fn new(kind: JeffOpKind) -> Self {
        Self {
            kind,
            inputs: Vec::new(),
            outputs: Vec::new(),
            metadata: Vec::new(),
        }
    }
}
