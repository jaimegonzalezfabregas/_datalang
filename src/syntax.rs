use std::{collections::HashSet, hash};

#[derive(Debug, Clone)]
pub struct RelName(pub String);

#[derive(Debug, Clone)]
pub enum VarName {
    DestructuredArray(Vec<Expresion>),
    Direct(String),
}

#[derive(Clone, Debug)]
pub enum Data {
    Number(f64),
    String(String),
    Array(Vec<Data>),
}

#[derive(Debug, Clone)]
pub enum VarLiteral {
    EmptySet,
    FullSet,
    Set(HashSet<Data>),
    AntiSet(HashSet<Data>),
}

impl VarLiteral {
    pub fn add(self: &mut VarLiteral, d: Data) -> Result<(), String> {
        match self {
            VarLiteral::EmptySet => *self = VarLiteral::Set(HashSet::from([d])),
            VarLiteral::FullSet => (),
            VarLiteral::Set(v) => {
                v.insert(d);
            }
            VarLiteral::AntiSet(v) => {
                v.retain(|e| !d.eq(e));
            }
        }

        Ok(())
    }
    pub fn remove(self: &mut VarLiteral, d: Data) -> Result<(), String> {
        match self {
            VarLiteral::EmptySet => (),
            VarLiteral::FullSet => *self = VarLiteral::Set(HashSet::from([d])),
            VarLiteral::AntiSet(v) => {
                v.insert(d);
            }
            VarLiteral::Set(v) => {
                v.retain(|e| !d.eq(e));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    // resolvable to a bolean
    Hypothetical(Vec<Line>, Box<Statement>), // TODO
    And(Box<Statement>, Box<Statement>),
    Or(Box<Statement>, Box<Statement>),
    Not(Box<Statement>),
    Arithmetic(
        Expresion,
        Expresion,
        fn(Expresion, Expresion) -> Result<bool, String>,
    ),
    Relation(RelName, Vec<Expresion>),
    Empty,
}

#[derive(Debug, Clone)]
pub enum Line {
    CreateRelation(RelName, Vec<VarLiteral>),
    ForgetRelation(RelName, Vec<VarLiteral>),
    TrueWhen(Box<Statement>, Box<Statement>),
    Query(RelName, Vec<Expresion>),
}

#[derive(Debug, Clone)]
pub enum Expresion {
    // resolvable to a value
    Arithmetic(
        Box<Expresion>,
        Box<Expresion>,
        fn(Expresion, Expresion) -> Result<Expresion, String>,
    ),
    Literal(VarLiteral),
    RestOfList(VarName),
    Var(VarName),
    Empty,
}

impl Expresion {
    pub fn literalize(self: &Expresion) -> Result<VarLiteral, String> {
        let ret = match self.clone() {
            Expresion::Arithmetic(a, b, f) => {
                if let Expresion::Literal(VarLiteral::FullSet) = *a {
                    Ok(VarLiteral::FullSet)
                } else if let Expresion::Literal(VarLiteral::FullSet) = *a {
                    Ok(VarLiteral::FullSet)
                } else {
                    f(*a, *b)?.literalize()
                }
            }
            Expresion::Literal(e) => Ok(e),
            _ => Err(format!("no se ha podido literalizar: {:#?}", self)),
        };

        return ret;
    }
}

impl VarLiteral {
    pub fn get_element_if_singleton(&self) -> Result<Data, String> {
        match self {
            VarLiteral::FullSet | VarLiteral::EmptySet | VarLiteral::AntiSet(_) => {
                Err("Not a singleton".into())
            }
            VarLiteral::Set(e) => {
                if e.len() == 1 {
                    return Ok(e.iter().take(1).collect::<Vec<&Data>>()[0].to_owned());
                } else {
                    Err("Not a singleton".into())
                }
            }
        }
    }

    pub fn set_eq(&self, other: &VarLiteral) -> bool {
        match (self, other) {
            (VarLiteral::FullSet, VarLiteral::FullSet) => true,
            (VarLiteral::EmptySet, VarLiteral::EmptySet) => true,
            (VarLiteral::Set(a), VarLiteral::Set(b)) => {
                for (a_it, b_it) in a.iter().zip(b) {
                    if !a_it.eq(b_it) {
                        return false;
                    }
                }
                true
            }
            (VarLiteral::AntiSet(a), VarLiteral::AntiSet(b)) => {
                for (a_it, b_it) in a.iter().zip(b) {
                    if !a_it.eq(b_it) {
                        return false;
                    }
                }
                true
            }

            (_, _) => false,
        }
    }

    pub fn contains_set(&self, contained_set: &VarLiteral) -> bool {
        match (contained_set, self) {
            (_, VarLiteral::FullSet) => true,
            (_, VarLiteral::EmptySet) => false,
            (VarLiteral::EmptySet, _) => true,
            (VarLiteral::FullSet, _) => false,

            (VarLiteral::Set(contained), VarLiteral::Set(container)) => {
                contained.is_subset(container)
            }

            (VarLiteral::AntiSet(_), VarLiteral::Set(_)) => false,
            (VarLiteral::AntiSet(not_in_contained), VarLiteral::AntiSet(not_in_container)) => {
                not_in_contained.is_superset(not_in_container)
            }

            (VarLiteral::Set(contained), VarLiteral::AntiSet(not_in_container)) => {
                not_in_container
                    .symmetric_difference(contained)
                    .map(|_| 0)
                    .collect::<Vec<i32>>()
                    .len()
                    == not_in_container
                        .union(contained)
                        .map(|_| 0)
                        .collect::<Vec<i32>>()
                        .len()
            }
        }
    }

    fn contains_element(&self, data: &Data) -> bool {
        match self {
            VarLiteral::FullSet => true,
            VarLiteral::EmptySet => false,
            VarLiteral::Set(set) => set.contains(data),
            VarLiteral::AntiSet(set) => !set.contains(data),
        }
    }
}

impl Data {
    fn eq(&self, other: &Data) -> bool {
        match (self, other) {
            (Data::Number(a), Data::Number(b)) => a == b,
            (Data::String(a), Data::String(b)) => a == b,
            (Data::Array(a), Data::Array(b)) => {
                if a.len() != b.len() {
                    false
                } else {
                    let mut c = 0;
                    for (it_a, it_b) in a.iter().zip(b) {
                        if it_a.eq(it_b) {
                            return false;
                        } else {
                            return false;
                        }
                    }
                    true
                }
            }
            _ => false,
        }
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl Eq for Data {}

impl hash::Hash for Data {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        match self {
            Data::Number(n) => {
                if (n.is_finite()) {
                    n.to_bits().hash(state)
                } else if n.is_infinite() {
                    f64::INFINITY.to_bits().hash(state)
                } else {
                    f64::NAN.to_bits().hash(state)
                }
            }
            Data::String(str) => str.hash(state),
            Data::Array(array) => array.hash(state),
        }
    }
}
