use crate::Identifier;

pub mod structures;

pub enum NyarType {
    I32,
    F32,
}

pub enum NyarValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Function(Identifier),
}

impl NyarValue {
    pub fn as_type(&self) -> NyarType {
        match self {
            NyarValue::I32(_) => NyarType::I32,
            NyarValue::I64(_) => NyarType::I32,
            NyarValue::F32(_) => NyarType::F32,
            NyarValue::F64(_) => NyarType::F32,
            NyarValue::Function(_) => NyarType::I32,
        }
    }
}
