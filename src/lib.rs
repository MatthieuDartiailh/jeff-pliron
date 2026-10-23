pub mod attributes;
pub mod dialect;
pub mod error;
pub mod lowering;
pub mod ops;
pub mod raising;
pub mod types;

pub use attributes::{GateMetadata, JeffMeta, StringTable};
pub use dialect::{jeff_dialect_name, register_jeff_dialect, JEFF_DIALECT_NAME};
pub use error::{JeffPlironError, JeffResult};
pub use lowering::{lower_float_precision, lower_module, lower_op, lower_op_with_context, lower_type, lower_value, lower_value_index, LoweringContext};
pub use ops::{JeffOp, JeffOpKind, JeffValue};
pub use raising::{raise_float_precision, raise_module, raise_op, raise_type, raise_value, RaisingContext};
pub use types::{FloatPrecision, JeffType};

#[cfg(test)]
mod tests;
