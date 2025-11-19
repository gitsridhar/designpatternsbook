pub struct Prototype {
    pub name: String,
}

impl Prototype {
    fn new(name: String) -> Prototype { // not used
        println!("Prototype: new");
        Prototype {
            name,
        }
    }
    fn newclone(&self) -> Prototype {
        println!("Prototype: newclone");
        Prototype {name: self.name.clone()}
    }
}

pub struct FruitPrototype {
    prototype: Prototype,
}

impl FruitPrototype {
    fn new (name: String) -> FruitPrototype {
        println!("FruitPrototype: new");
        FruitPrototype {
            prototype: Prototype { name }
        }
    }
    fn newclone(&self) -> Option<FruitPrototype> {
        println!("FruitPrototype: newclone");
        Some(FruitPrototype {
            prototype: self.prototype.newclone()
        })
    }
}

pub struct VegetablePrototype {
    prototype: Prototype,
}

impl VegetablePrototype {
    fn new (name: String) -> VegetablePrototype {
        println!("VegetablePrototype: new");
        VegetablePrototype {
            prototype: Prototype { name }
        }
    }
    fn newclone(&self) -> Option<VegetablePrototype> {
        println!("VegetablePrototype: newclone");
        Some(VegetablePrototype {
            prototype: self.prototype.newclone()
        })
    }
}

pub struct PrototypeFactory {
}

impl PrototypeFactory {
    pub fn create_fruit_prototype(&self, name: String) -> Option<FruitPrototype> {
        println!("PrototypeFactory: create fruit prototype");
        if name == "fruit" {
            let f = Some(FruitPrototype{
                prototype: Prototype{name: name}
            });
            return Some(f?.newclone()?);
        } else {
            None
        }
    }
    pub fn create_vegetable_prototype(&self, name: String) -> Option<VegetablePrototype> {
        println!("PrototypeFactory: create vegetable prototype");
        if name == "vegetable" {
            let v = Some(VegetablePrototype{
                prototype: Prototype{name: name}
            });
            return Some(v?.newclone()?);
        } else {
            None
        }
    }
}