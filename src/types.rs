use std::collections::HashMap;

#[derive(PartialEq,Eq,Debug,Hash,Clone)]
pub enum BasicValue {
    Byte(u8),
    Boolean(bool),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    String(String),
    ObjectPath(Path),
    Signature(Signature),
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct Path(pub String);

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub struct Signature(pub String);

impl BasicValue {
    fn get_signature(&self) -> &str {
        match self {
            &BasicValue::Byte(_) => "y",
            &BasicValue::Boolean(_) => "b",
            &BasicValue::Int16(_) => "n",
            &BasicValue::Uint16(_) => "q",
            &BasicValue::Int32(_) => "i",
            &BasicValue::Uint32(_) => "u",
            &BasicValue::Int64(_) => "x",
            &BasicValue::Uint64(_) => "t",
            &BasicValue::String(_) => "s",
            &BasicValue::ObjectPath(_) => "o",
            &BasicValue::Signature(_) => "g",
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct Struct {
    pub objects: Vec<Value>,
    pub signature: Signature
}

#[derive(PartialEq,Debug,Clone)]
pub struct Variant {
    pub object: Box<Value>,
    pub signature: Signature
}

impl Variant {
    pub fn new (v: Value, s: &str) -> Variant {
        Variant {
            object: Box::new(v),
            signature: Signature(s.to_string())
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct Array {
    pub objects: Vec<Value>,
    signature: Signature
}

impl Array {
    // This function can only be used if it is never possible for the
    // input vector to be empty.  If it is empty, this function will
    // panic.  The reason is that it is impossible to determine the type
    // signature for an empty vector.  Use new_with_sig instead.
    pub fn new(objects: Vec<Value>) -> Array {
        let sig = objects.iter().next().unwrap().get_signature().to_string();
        Array {
            objects: objects,
            signature: Signature(sig)
        }
    }

    pub fn new_with_sig(objects: Vec<Value>, sig: String) -> Array {
        Array {
            objects: objects,
            signature: Signature(sig)
        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub struct Dictionary {
    pub map: HashMap<BasicValue,Value>,
    signature: Signature
}

impl Dictionary {
    // This function can only be used if it is never possible for the
    // input map to be empty.  If it is empty, this function will
    // panic.  The reason is that it is impossible to determine the type
    // signature for an empty map.  Use new_with_sig instead.
    pub fn new(map: HashMap<BasicValue,Value>) -> Dictionary {
        let key_type = map.keys().next().unwrap().get_signature().to_string();
        let val_type = map.values().next().unwrap().get_signature().to_string();
        let sig = "a{".to_string() + &key_type + &val_type + "}";
        Dictionary {
            map: map,
            signature: Signature(sig)
        }
    }

    pub fn new_with_sig(map: HashMap<BasicValue,Value>, sig: String) -> Dictionary {
        Dictionary {
            map: map,
            signature: Signature(sig)
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
pub enum Value {
    BasicValue(BasicValue),
    Double(f64),
    Array(Array),
    Variant(Variant),
    Struct(Struct),
    Dictionary(Dictionary)
}

impl Value {
    fn get_signature(&self) -> &str {
        match self {
            &Value::BasicValue(ref x) => x.get_signature(),
            &Value::Double(_) => "d",
            &Value::Array(ref x) => &x.signature.0,
            &Value::Variant(_) => "v",
            &Value::Struct(ref x) => &x.signature.0,
            &Value::Dictionary(ref x) => &x.signature.0
        }
    }
}

#[test]
fn test_from () {
    let x = Value::from(12);
    assert_eq!(x, Value::BasicValue(BasicValue::Int32(12)));
    let y = Value::from("foobar");
    assert_eq!(y, Value::BasicValue(BasicValue::String("foobar".to_string())));
}
