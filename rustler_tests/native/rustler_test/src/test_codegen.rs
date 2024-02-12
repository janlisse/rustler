use rustler::types::truthy::Truthy;
use rustler::{
    NifException, NifMap, NifRecord, NifStruct, NifTaggedEnum, NifTuple, NifUnitEnum,
    NifUntaggedEnum,
};

/// A trait for testing the ambiguity of `encode` and `decode`.
pub trait EmptyTranscoder {
    fn encode(&self);
    fn decode();
}

impl<T> EmptyTranscoder for T {
    fn encode(&self) {}
    fn decode() {}
}

#[derive(NifTuple)]
pub struct AddTuple {
    lhs: i32,
    rhs: i32,
}

#[rustler::nif]
pub fn tuple_echo(tuple: AddTuple) -> AddTuple {
    tuple
}

#[derive(NifRecord)]
#[rustler(encode, decode)] // Added to check encode/decode attribute, #180
#[must_use] // Added to check attribute order (see similar issue #152)
#[tag = "record"]
pub struct AddRecord {
    lhs: i32,
    rhs: i32,
}

#[rustler::nif]
pub fn record_echo(record: AddRecord) -> AddRecord {
    record
}

#[derive(NifMap)]
pub struct AddMap {
    lhs: i32,
    rhs: i32,
    loc: (u32, u32),
}

#[rustler::nif]
pub fn map_echo(map: AddMap) -> AddMap {
    map
}

#[derive(Debug, NifStruct)]
#[must_use] // Added to test Issue #152
#[module = "AddStruct"]
pub struct AddStruct {
    lhs: i32,
    rhs: i32,
    loc: (u32, u32),
}

#[derive(Debug, NifException)]
#[module = "AddException"]
pub struct AddException {
    message: String,
    loc: (u32, u32),
}

#[rustler::nif]
pub fn struct_echo(add_struct: AddStruct) -> AddStruct {
    add_struct
}

#[rustler::nif]
pub fn exception_echo(add_exception: AddException) -> AddException {
    add_exception
}

#[derive(NifUnitEnum)]
pub enum UnitEnum {
    FooBar,
    Baz,
}

#[rustler::nif]
pub fn unit_enum_echo(unit_enum: UnitEnum) -> UnitEnum {
    unit_enum
}

#[derive(NifTaggedEnum)]
pub enum TaggedEnum1 {
    Named { x: i32, y: i32 },
    String1(String),
    String2(String),
    Untagged,
    Samefields { x: i32, y: i32 },
}

#[rustler::nif]
pub fn tagged_enum_1_echo(tagged_enum: TaggedEnum1) -> TaggedEnum1 {
    tagged_enum
}

#[derive(NifTaggedEnum)]
pub enum TaggedEnum2 {
    Untagged,
    HashMap(std::collections::HashMap<i32, i32>),
    Tuple(i64, i64),
    Named { s: String },
    Enum(TaggedEnum1),
}

#[rustler::nif]
pub fn tagged_enum_2_echo(tagged_enum: TaggedEnum2) -> TaggedEnum2 {
    tagged_enum
}

#[derive(NifTaggedEnum)]
pub enum TaggedEnum3 {
    Struct(AddStruct),
    Named { lhs: i32, rhs: i32 },
}

#[rustler::nif]
pub fn tagged_enum_3_echo(tagged_enum: TaggedEnum3) -> TaggedEnum3 {
    tagged_enum
}

#[derive(NifTaggedEnum)]
pub enum TaggedEnum4 {
    Unit,
    Unnamed(u64, bool),
    Named {
        size: u64,
        filename: String,
    },
    Long {
        f0: bool,
        f1: u8,
        f2: u8,
        f3: u8,
        f4: u8,
        f5: Option<i32>,
        f6: Option<i32>,
        f7: Option<i32>,
        f8: Option<i32>,
    },
}

#[rustler::nif]
pub fn tagged_enum_4_echo(tagged_enum: TaggedEnum4) -> TaggedEnum4 {
    tagged_enum
}

#[derive(NifUntaggedEnum)]
pub enum UntaggedEnum {
    Foo(u32),
    Bar(String),
    Baz(AddStruct),
    Bool(bool),
}

#[rustler::nif]
pub fn untagged_enum_echo(untagged_enum: UntaggedEnum) -> UntaggedEnum {
    untagged_enum
}

#[derive(NifUntaggedEnum)]
pub enum UntaggedEnumWithTruthy {
    Baz(AddStruct),
    Truthy(Truthy),
}

#[rustler::nif]
pub fn untagged_enum_with_truthy(untagged_enum: UntaggedEnumWithTruthy) -> UntaggedEnumWithTruthy {
    untagged_enum
}

#[derive(NifUntaggedEnum)]
pub enum UntaggedEnumForIssue370 {
    Vec(Vec<i64>),
}

#[rustler::nif]
pub fn untagged_enum_for_issue_370(
    untagged_enum: UntaggedEnumForIssue370,
) -> UntaggedEnumForIssue370 {
    untagged_enum
}

#[derive(NifTuple)]
pub struct Newtype(i64);

#[rustler::nif]
pub fn newtype_echo(newtype: Newtype) -> Newtype {
    newtype
}

#[derive(NifTuple)]
pub struct TupleStruct(i64, i64, i64);

#[rustler::nif]
pub fn tuplestruct_echo(tuplestruct: TupleStruct) -> TupleStruct {
    tuplestruct
}

#[derive(NifRecord)]
#[tag = "newtype"]
pub struct NewtypeRecord(i64);

#[rustler::nif]
pub fn newtype_record_echo(newtype: NewtypeRecord) -> NewtypeRecord {
    newtype
}

#[derive(NifRecord)]
#[tag = "tuplestruct"]
pub struct TupleStructRecord(i64, i64, i64);

#[rustler::nif]
pub fn tuplestruct_record_echo(tuplestruct: TupleStructRecord) -> TupleStructRecord {
    tuplestruct
}

pub mod reserved_keywords {
    use rustler::{NifMap, NifRecord, NifStruct, NifTuple, NifUntaggedEnum};

    #[derive(NifMap, Debug)]
    pub struct Map {
        r#override: i32,
    }

    #[derive(NifStruct, Debug)]
    #[module = "Struct"]
    pub struct Struct {
        r#override: i32,
    }

    #[derive(NifTuple, Debug)]
    pub struct Tuple {
        r#override: i32,
    }

    #[derive(NifRecord, Debug)]
    #[tag = "record"]
    pub struct Record {
        r#override: i32,
    }

    #[derive(NifUntaggedEnum)]
    pub enum ReservedKeywords {
        Struct(Struct),
        Map(Map),
        Tuple(Tuple),
        Record(Record),
    }

    #[rustler::nif]
    pub fn reserved_keywords_type_echo(reserved: ReservedKeywords) -> ReservedKeywords {
        reserved
    }
}

pub mod enum_struct {
    use rustler::NifEnumStruct;

    #[derive(NifEnumStruct)]
    pub enum EnumStruct {
        #[module = "EnumStruct.Foo"]
        Foo {
            foo1: i32,
            foo2: String,
            some_field: String,
        },
        #[module = "EnumStruct.Bar"]
        Bar {
            bar: bool,
            some_field: f64,
        },
        #[module = "EnumStruct.Baz"]
        Baz {}
    }

    #[derive(NifEnumStruct)]
    pub enum EnumStruct2 {
        #[module = "EnumStruct.Foo"]
        Foo {
            foo1: i32,
            foo2: String,
            some_field: String,
        },
        #[module = "EnumStruct.Bar"]
        Bar {
            bar: bool,
            some_field: f64,
        },
        #[module = "EnumStruct.Baz"]
        Baz {}
    }

    /* mod atoms {
        pub mod foo {
            rustler::atoms! {
                __struct__ = "Elixir.EnumStruct.Foo",
                foo1,
                foo2,
                some_field,
            }
        }
        pub mod bar {
            rustler::atoms! {
                __struct__ = "Elixir.EnumStruct.Bar",
                bar,
                some_field,
            }
        }
        pub mod baz {
            rustler::atoms! {
                __struct__ = "Elixir.EnumStruct.Baz",
            }
        }
    } */

    /* impl<'a> rustler::Decoder<'a> for EnumStruct {
        fn decode(term: rustler::Term<'a>) -> rustler::NifResult<EnumStruct> {
            let struct_module: rustler::types::atom::Atom =
                term.map_get(rustler::types::atom::__struct__())?.decode()?;
            if struct_module == atoms::foo::__struct__() {
                let foo1 = term.map_get(atoms::foo::foo1())?.decode()?;
                let foo2 = term.map_get(atoms::foo::foo2())?.decode()?;
                let some_field = term.map_get(atoms::foo::some_field())?.decode()?;
                return Ok(EnumStruct::Foo {
                    foo1,
                    foo2,
                    some_field,
                });
            }
            if struct_module == atoms::bar::__struct__() {
                let bar = term.map_get(atoms::bar::bar())?.decode()?;
                let some_field = term.map_get(atoms::bar::some_field())?.decode()?;
                return Ok(EnumStruct::Bar { bar, some_field });
            }
            if struct_module == atoms::baz::__struct__() {
                return Ok(EnumStruct::Baz {});
            }
            Err(rustler::Error::RaiseAtom("No known variant for enum"))
        }
    }

    impl rustler::Encoder for EnumStruct {
        fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
            match self {
                EnumStruct::Foo {
                    foo1,
                    foo2,
                    some_field,
                } => ::rustler::Term::map_from_term_arrays(
                    env,
                    &[
                        rustler::types::atom::__struct__().encode(env),
                        atoms::foo::foo1().encode(env),
                        atoms::foo::foo2().encode(env),
                        atoms::foo::some_field().encode(env),
                    ],
                    &[
                        atoms::foo::__struct__().encode(env),
                        foo1.encode(env),
                        foo2.encode(env),
                        some_field.encode(env),
                    ],
                ),
                EnumStruct::Bar { bar, some_field } => ::rustler::Term::map_from_term_arrays(
                    env,
                    &[
                        rustler::types::atom::__struct__().encode(env),
                        atoms::bar::bar().encode(env),
                        atoms::bar::some_field().encode(env),
                    ],
                    &[
                        atoms::bar::__struct__().encode(env),
                        bar.encode(env),
                        some_field.encode(env),
                    ],
                ),
                EnumStruct::Baz {} => ::rustler::Term::map_from_term_arrays(
                    env,
                    &[rustler::types::atom::__struct__().encode(env)],
                    &[atoms::baz::__struct__().encode(env)],
                ),
            }
            .expect("encode an enumstruct")
        }
    } */

    #[rustler::nif]
    pub fn enum_struct_echo(value: EnumStruct) -> EnumStruct {
        value
    }
}
