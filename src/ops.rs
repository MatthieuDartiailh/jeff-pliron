use crate::types::JeffType;
use pliron::derive::pliron_op;

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

#[pliron_op(name = "jeff.qubit_alloc", format, verifier = "succ")]
pub struct QubitAllocOp;

#[pliron_op(name = "jeff.qubit_free", format, verifier = "succ")]
pub struct QubitFreeOp;

#[pliron_op(name = "jeff.qubit_measure", format, verifier = "succ")]
pub struct QubitMeasureOp;

#[pliron_op(name = "jeff.qureg_alloc", format, verifier = "succ")]
pub struct QuregAllocOp;

#[pliron_op(name = "jeff.func_call", format, verifier = "succ")]
pub struct FuncCallOp;

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
