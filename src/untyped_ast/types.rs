#![allow(dead_code)]

use crate::lex::types::NumberType;

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub base: BaseType,
    pub volatile: bool,
    pub register: bool,
}

impl Type {
    pub fn new(base: BaseType) -> Self {
        Type {
            base,
            volatile: false,
            register: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseType {
    // Numbers
    Float,
    Double,
    SignedChar,
    UnsignedChar,
    SignedShort,
    UnsignedShort,
    SignedInt,
    UnsignedInt,
    SignedLong,
    UnsignedLong,
    SignedLongLong,
    UnsignedLongLong,

    Pointer(Option<Box<Type>>),
    Array(Box<Type>, usize),
    Struct(Option<String>, Vec<(String, Type)>),
    FunctionPtr(Vec<Type>, Option<Box<Type>>),
    Union(Option<String>, Vec<(String, Type)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopStatement {
    Declaration(Type, String, Option<Box<RValue>>),
    ForwardDeclaration(Type, String, Vec<Type>),
    FunctionDeclaration(Type, String, Vec<(Type, String)>, Block),
    StructOrUnionDeclaration(Type),
    TypedefDeclaration(String, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug, Clone, PartialEq)]
pub struct Root(pub Vec<TopStatement>);

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LValue(Box<LValue>),
    Declaration(Type, String, Option<Box<RValue>>),

    If(RValue, Box<Statement>, Box<Statement>),
    While(RValue, Box<Statement>),
    DoWhile(RValue, Box<Statement>),
    For(Box<Statement>, RValue, RValue, Box<Statement>),

    Block(Block),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LValue {
    PointerDeref(Box<RValue>),
    StructDeref(Box<RValue>, String),
    StructRef(Box<LValue>, String),
    Variable(String),
    Subscript(Box<RValue>, Box<RValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RValue {
    NumberLiteral(NumberType),
    StringLiteral(Vec<u8>),

    PreIncrement(LValue),
    PostIncrement(LValue),
    PreDecrement(LValue),
    PostDecrement(LValue),
    Add(Box<RValue>, Box<RValue>),
    Mul(Box<RValue>, Box<RValue>),
    Sub(Box<RValue>, Box<RValue>),
    Div(Box<RValue>, Box<RValue>),
    Mod(Box<RValue>, Box<RValue>),
    Positive(Box<RValue>),
    Negative(Box<RValue>),

    Equals(Box<RValue>, Box<RValue>),
    NotEquals(Box<RValue>, Box<RValue>),
    Greater(Box<RValue>, Box<RValue>),
    GreaterOrEqual(Box<RValue>, Box<RValue>),
    Less(Box<RValue>, Box<RValue>),
    LessOrEqual(Box<RValue>, Box<RValue>),

    LogicalNot(Box<RValue>),
    LogicalAnd(Box<RValue>, Box<RValue>),
    LogicalOr(Box<RValue>, Box<RValue>),

    BitwiseNot(Box<RValue>),
    BitwiseAnd(Box<RValue>, Box<RValue>),
    BitwiseOr(Box<RValue>, Box<RValue>),
    BitwiseXor(Box<RValue>, Box<RValue>),
    BitwiseLeftShift(Box<RValue>, Box<RValue>),
    BitwiseRightShift(Box<RValue>, Box<RValue>),

    AddressOf(Box<LValue>),
    StructRef(Box<RValue>, String),

    SizeofType(Type),
    SizeofValue(Box<RValue>),

    Cast(Type, Box<RValue>),

    FunctionCall(Box<RValue>, Vec<RValue>),
    Comma(Box<RValue>, Box<RValue>),
    Ternary(Box<RValue>, Box<RValue>, Box<RValue>),

    FunctionName(String),

    Assign(Box<LValue>, Box<RValue>),
    AddAssign(Box<LValue>, Box<RValue>),
    SubAssign(Box<LValue>, Box<RValue>),
    MulAssign(Box<LValue>, Box<RValue>),
    DivAssign(Box<LValue>, Box<RValue>),
    ModAssign(Box<LValue>, Box<RValue>),
    BitwiseAndAssign(Box<LValue>, Box<RValue>),
    BitwiseOrAssign(Box<LValue>, Box<RValue>),
    BitwiseXorAssign(Box<LValue>, Box<RValue>),
    BitwiseLeftShiftAssign(Box<LValue>, Box<RValue>),
    BitwiseRightShiftAssign(Box<LValue>, Box<RValue>),

    LValue(LValue),
}
