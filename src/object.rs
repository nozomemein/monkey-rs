use crate::ast::Expr;
use crate::{ast::Stmt, environment::Environment};
use std::any::Any;
use std::collections::HashMap;

#[allow(dead_code)]
pub trait Object: ObjectClone {
    fn as_any(&self) -> &dyn Any;
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

pub trait ObjectClone {
    fn clone_box(&self) -> ObjectRef;
}

impl<T> ObjectClone for T
where
    T: 'static + Object + Clone,
{
    fn clone_box(&self) -> ObjectRef {
        Box::new(self.clone())
    }
}

impl Clone for ObjectRef {
    fn clone(&self) -> ObjectRef {
        self.clone_box()
    }
}

pub type ObjectRef = Box<dyn Object>;

const INTEGER_OBJ: &str = "INTEGER";
const NULL_OBJ: &str = "NULL";
const BOOLEAN_OBJ: &str = "BOOLEAN";
const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
const ERROR_OBJ: &str = "ERROR";
const FUNCTION_OBJ: &str = "FUNCTION";
const STRING_OBJ: &str = "STRING";
const BUILTIN_OBJ: &str = "BUILTIN";
const ARRAY_OBJ: &str = "ARRAY";
const HASH_OBJ: &str = "HASH";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ObjectType {
    Integer,
    Null,
    Boolean,
    ReturnValue,
    Error,
    Function,
    StringObj,
    Builtin,
    Array,
    Hash,
}

#[allow(dead_code)]
impl ObjectType {
    pub fn as_str(&self) -> &str {
        match self {
            ObjectType::Integer => INTEGER_OBJ,
            ObjectType::Null => NULL_OBJ,
            ObjectType::Boolean => BOOLEAN_OBJ,
            ObjectType::ReturnValue => RETURN_VALUE_OBJ,
            ObjectType::Error => ERROR_OBJ,
            ObjectType::Function => FUNCTION_OBJ,
            ObjectType::StringObj => STRING_OBJ,
            ObjectType::Builtin => BUILTIN_OBJ,
            ObjectType::Array => ARRAY_OBJ,
            ObjectType::Hash => HASH_OBJ,
        }
    }
}

#[derive(Clone)]
pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Integer
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Clone)]
pub struct Null;

impl Object for Null {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Null
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }
}

#[derive(Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Clone)]
pub struct ReturnValue {
    pub value: ObjectRef,
}

impl Object for ReturnValue {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::ReturnValue
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }
}

#[derive(Clone)]
pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Error
    }

    fn inspect(&self) -> String {
        format!("{}", self.message)
    }
}

#[derive(Clone)]
pub struct Function {
    pub parameters: Vec<Box<Expr>>,
    pub body: Box<Stmt>,
    pub env: Environment,
}

impl Object for Function {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Function
    }

    fn inspect(&self) -> String {
        let mut params = Vec::new();
        for p in &self.parameters {
            params.push(format!("{:?}", p));
        }
        format!("fn({}) {:?}", params.join(", "), self.body)
    }
}

#[derive(Clone)]
pub struct StringObj {
    pub value: String,
}

impl Object for StringObj {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::StringObj
    }

    fn inspect(&self) -> String {
        format!("\"{}\"", self.value)
    }
}

#[derive(Clone)]
pub struct Builtin {
    pub func: fn(Vec<ObjectRef>) -> ObjectRef,
}

impl Object for Builtin {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Builtin
    }

    fn inspect(&self) -> String {
        "builtin function".to_string()
    }
}

#[derive(Clone)]
pub struct Array {
    pub elements: Vec<ObjectRef>,
}

impl Object for Array {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Array
    }

    fn inspect(&self) -> String {
        let mut elements = Vec::new();
        for e in &self.elements {
            elements.push(e.inspect());
        }
        format!("[{}]", elements.join(", "))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashKey {
    pub value: u64,
}

#[derive(Clone)]
pub struct HashPair {
    pub key: ObjectRef,
    pub value: ObjectRef,
}

#[allow(dead_code)]
pub trait Hashable {
    fn hash_key(&self) -> HashKey;
}

#[derive(Clone)]
pub struct Hash {
    pub pairs: HashMap<HashKey, HashPair>,
}

impl Object for Hash {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn object_type(&self) -> ObjectType {
        ObjectType::Hash
    }

    fn inspect(&self) -> String {
        let mut pairs = Vec::new();
        for (_, pair) in &self.pairs {
            pairs.push(format!("{}: {}", pair.key.inspect(), pair.value.inspect()));
        }
        format!("{{{}}}", pairs.join(", "))
    }
}

impl Hashable for Integer {
    fn hash_key(&self) -> HashKey {
        HashKey {
            value: self.value as u64,
        }
    }
}

impl Hashable for Boolean {
    fn hash_key(&self) -> HashKey {
        let value = if self.value { 1 } else { 0 };
        HashKey { value }
    }
}

impl Hashable for StringObj {
    fn hash_key(&self) -> HashKey {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.value.hash(&mut hasher);
        HashKey {
            value: hasher.finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_hash_key() {
        let hello1 = StringObj {
            value: "Hello World".to_string(),
        };
        let hello2 = StringObj {
            value: "Hello World".to_string(),
        };
        let diff1 = StringObj {
            value: "My name is johnny".to_string(),
        };
        let diff2 = StringObj {
            value: "My name is johnny".to_string(),
        };

        assert_eq!(hello1.hash_key(), hello2.hash_key());
        assert_eq!(diff1.hash_key(), diff2.hash_key());
        assert_ne!(hello1.hash_key(), diff1.hash_key());
    }

    #[test]
    fn test_boolean_hash_key() {
        let true1 = Boolean { value: true };
        let true2 = Boolean { value: true };
        let false1 = Boolean { value: false };
        let false2 = Boolean { value: false };

        assert_eq!(true1.hash_key(), true2.hash_key());
        assert_eq!(false1.hash_key(), false2.hash_key());
        assert_ne!(true1.hash_key(), false1.hash_key());
    }

    #[test]
    fn test_integer_hash_key() {
        let one1 = Integer { value: 1 };
        let one2 = Integer { value: 1 };
        let two1 = Integer { value: 2 };
        let two2 = Integer { value: 2 };

        assert_eq!(one1.hash_key(), one2.hash_key());
        assert_eq!(two1.hash_key(), two2.hash_key());
        assert_ne!(one1.hash_key(), two1.hash_key());
    }
}
