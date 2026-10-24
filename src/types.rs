use std::fmt;

use pliron::derive::pliron_type;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FloatPrecision {
    Float32,
    Float64,
}

impl FloatPrecision {
    pub fn bits(self) -> u8 {
        match self {
            Self::Float32 => 32,
            Self::Float64 => 64,
        }
    }
}

impl fmt::Display for FloatPrecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.bits())
    }
}

#[pliron_type(name = "jeff.qubit", format, generate_get = true, verifier = "succ")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QubitType;

#[pliron_type(name = "jeff.qureg", format, generate_get = true, verifier = "succ")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QuregType {
    pub length: u32,
    pub is_dynamic: bool,
}

#[pliron_type(name = "jeff.int", format, generate_get = true, verifier = "succ")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntType {
    pub bits: u8,
}

#[pliron_type(name = "jeff.intArray", format, generate_get = true, verifier = "succ")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntArrayType {
    pub bits: u8,
    pub length: u32,
    pub is_dynamic: bool,
}

#[pliron_type(name = "jeff.float", format, generate_get = true, verifier = "succ")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FloatType {
    pub bits: u8,
}

#[pliron_type(
    name = "jeff.floatArray",
    format,
    generate_get = true,
    verifier = "succ"
)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FloatArrayType {
    pub bits: u8,
    pub length: u32,
    pub is_dynamic: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum JeffType {
    Qubit,
    Qureg {
        length: Option<u32>,
    },
    Int {
        bits: u8,
    },
    IntArray {
        bits: u8,
        length: Option<u32>,
    },
    Float {
        precision: FloatPrecision,
    },
    FloatArray {
        precision: FloatPrecision,
        length: Option<u32>,
    },
}

impl fmt::Display for JeffType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Qubit => write!(f, "!jeff.qubit"),
            Self::Qureg { length: None } => write!(f, "!jeff.qureg"),
            Self::Qureg { length: Some(len) } => write!(f, "!jeff.qureg<{len}>"),
            Self::Int { bits } => write!(f, "!jeff.int<{bits}>"),
            Self::IntArray { bits, length: None } => write!(f, "!jeff.intArray<{bits}, dynamic>"),
            Self::IntArray {
                bits,
                length: Some(len),
            } => {
                write!(f, "!jeff.intArray<{bits}, static({len})>")
            }
            Self::Float { precision } => write!(f, "!jeff.float<{precision}>"),
            Self::FloatArray {
                precision,
                length: None,
            } => {
                write!(f, "!jeff.floatArray<{precision}, dynamic>")
            }
            Self::FloatArray {
                precision,
                length: Some(len),
            } => {
                write!(f, "!jeff.floatArray<{precision}, static({len})>")
            }
        }
    }
}

impl JeffType {
    pub fn qubit() -> Self {
        Self::Qubit
    }

    pub fn qureg(length: Option<u32>) -> Self {
        Self::Qureg { length }
    }

    pub fn int(bits: u8) -> Self {
        Self::Int { bits }
    }

    pub fn int_array(bits: u8, length: Option<u32>) -> Self {
        Self::IntArray { bits, length }
    }

    pub fn float(precision: FloatPrecision) -> Self {
        Self::Float { precision }
    }

    pub fn float_array(precision: FloatPrecision, length: Option<u32>) -> Self {
        Self::FloatArray { precision, length }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_display_is_semantic() {
        assert_eq!(JeffType::Qubit.to_string(), "!jeff.qubit");
        assert_eq!(JeffType::Int { bits: 8 }.to_string(), "!jeff.int<8>");
        assert_eq!(
            JeffType::IntArray {
                bits: 8,
                length: None
            }
            .to_string(),
            "!jeff.intArray<8, dynamic>"
        );
        assert_eq!(
            JeffType::FloatArray {
                precision: FloatPrecision::Float64,
                length: Some(16)
            }
            .to_string(),
            "!jeff.floatArray<64, static(16)>"
        );
    }
}
