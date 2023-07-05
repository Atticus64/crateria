use std::{collections::HashMap, any::TypeId};

// - - -
#[derive(Clone, Debug)]
enum Any {
    USize(usize),
    ISize(isize),
    F32(f32),
    F64(f64),
    Bool(bool),
    String(String),
    Str(&'static str),
}

impl Any {
    pub fn get<T>(&self) -> &T
    where
        T: 'static,
    {
        match self {
            Any::USize(value) if TypeId::of::<usize>() == TypeId::of::<T>() => {
                unsafe { &*(value as *const usize as *const T) }
            }
            Any::ISize(value) if TypeId::of::<isize>() == TypeId::of::<T>() => {
                unsafe { &*(value as *const isize as *const T) }
            }
            Any::F32(value) if TypeId::of::<f32>() == TypeId::of::<T>() => {
                unsafe { &*(value as *const f32 as *const T) }
            }
            Any::F64(value) if TypeId::of::<f64>() == TypeId::of::<T>() => {
                unsafe { &*(value as *const f64 as *const T) }
            }
            Any::Bool(value) if TypeId::of::<bool>() == TypeId::of::<T>() => {
                unsafe { &*(value as *const bool as *const T) }
            }
            Any::String(value) if TypeId::of::<String>() == TypeId::of::<T>() => {
                unsafe { &*(value as *const String as *const T) }
            }
            Any::Str(value) if TypeId::of::<&'static str>() == TypeId::of::<T>() => {
                unsafe { &*(value as *const &'static str as *const T) }
            }
            _ => panic!("{:?} is not of type {:?}", self, std::any::type_name::<T>()),
        }
    }
}

trait Update {
    fn initialize(entity: &mut Inventory) {}
    fn update(entity: &mut Inventory) -> bool {
        false
    }
}

trait State {
    fn initialize(entity: &mut Inventory) {}
    fn update(entity: &mut Inventory) -> bool {
        false
    }
}

// - - -

trait Shootable {
    fn initialize(entity: &mut Inventory) {}
    fn update(entity: &mut Inventory) -> bool {
        false
    }
}

// - - -

trait Collidable {
    fn initialize(entity: &mut Inventory) {}
    fn update(entity: &mut Inventory) -> bool {
        false
    }
}

trait Moveable {
    fn initialize(entity: &mut Inventory) {
        entity.set("acceleration", Any::F64(0.0));
        entity.set("text", Any::Str("Hello world!"));
    }

    fn update(entity: &mut Inventory) -> bool {
        let acceleration = entity.get::<f32>("acceleration");
        let text = entity.get::<&'static str>("text");

        println!("Acceleration: {}", acceleration);
        println!("Text: {}", text);

        false
    }
}
// - - -
struct Inventory {
    content: HashMap<&'static str, Any>,
}

impl Inventory {
    pub fn get<T>(&self, key: &'static str) -> &T
    where
        T: 'static,
    {
        self.content.get(key).unwrap().get::<T>()
    }

    pub fn set(&mut self, key: &'static str, value: Any)
    {
        self.content.insert(key, value);
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self { content: HashMap::new(), }
    }
}

//

struct Object {
    pub _id: usize,
    pub inventory: Inventory,
}
