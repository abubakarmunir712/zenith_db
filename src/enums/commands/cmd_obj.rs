use crate::enums::types::datatypes::DataType;

pub struct ColumnDef {
    pub name: String,
    pub datatype: DataType,
    pub size:Option<u32>,
}


pub struct Condition {
    pub column: String,
    pub op: ComparisonOp,
    pub value: String,
}

pub enum ComparisonOp {
    Eq,
    NotEq,
    Lt,
    Gt,
    Lte,
    Gte,
}