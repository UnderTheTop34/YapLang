use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{Debug, Display};

pub struct Variable {
    pub value: String,
    pub kind: VariableKind,
    pub is_mutable: bool,
    pub scope: u16,
}

impl Variable {
    pub fn new(kind: VariableKind, scope: u16, is_mutable: bool, value: String) -> Self {
        Variable { kind, scope, is_mutable, value }
    }
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        Variable {kind: self.kind.clone(), scope: self.scope, is_mutable: self.is_mutable, value: self.value.clone()}
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.scope == other.scope && self.value == other.value && self.is_mutable == other.is_mutable && self.kind == other.kind
    }
}


impl Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.value)
            .field(&self.kind.to_string())
            .field(&self.scope.to_string())
            .field(&self.is_mutable)
            .finish()
    }
}

pub enum VariableKind {
    String,
    Integer,
    Float,
    Boolean,
}

impl Clone for VariableKind {
    fn clone(&self) -> Self {
        match self {
            VariableKind::String => {VariableKind::String}
            VariableKind::Integer => {VariableKind::Integer}
            VariableKind::Float => {VariableKind::Float}
            VariableKind::Boolean => {VariableKind::Boolean}
        }
    }
}

impl Display for VariableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableKind::String => {write!(f, "String")}
            VariableKind::Integer => {write!(f, "Integer")}
            VariableKind::Float => {write!(f, "Float")}
            VariableKind::Boolean => {write!(f, "Boolean")}
        }
    }
}

impl PartialEq for VariableKind {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

