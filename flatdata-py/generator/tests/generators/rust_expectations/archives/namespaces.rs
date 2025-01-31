// Do not edit: This code was generated by flatdata's generator.
pub mod n {

pub mod schema {
pub mod structs {
pub const S: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

"#;}pub mod x {
pub const X: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

"#;
pub mod resources {pub const PAYLOAD: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

"#;}
}
}

define_struct!(
    S,
    RefS,
    RefMutS,
    schema::structs::S,
    8,
    (x, set_x, u64, u64, 0, 64));




define_archive!(X, XBuilder,
    schema::x::X;
    // struct resources
;
    // vector resources
;
    // multivector resources
;
    // raw data resources
    (payload, set_payload,
        schema::x::resources::PAYLOAD, false);
    // subarchives
);

}

pub mod m {

pub mod schema {
pub mod structs {
pub const S: &str = r#"namespace m {
struct S
{
    x : u64 : 64;
}
}

"#;}pub mod x {
pub const X: &str = r#"namespace m {
archive X
{
    payload : raw_data;
}
}

"#;
pub mod resources {pub const PAYLOAD: &str = r#"namespace m {
archive X
{
    payload : raw_data;
}
}

"#;}
}
}

define_struct!(
    S,
    RefS,
    RefMutS,
    schema::structs::S,
    8,
    (x, set_x, u64, u64, 0, 64));




define_archive!(X, XBuilder,
    schema::x::X;
    // struct resources
;
    // vector resources
;
    // multivector resources
;
    // raw data resources
    (payload, set_payload,
        schema::x::resources::PAYLOAD, false);
    // subarchives
);

}

pub mod a {

pub mod schema {
pub mod structs {}pub mod a {
pub const A: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace m {
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive X
{
    payload : raw_data;
}
}

namespace a {
archive A
{
    single : .n.S;
    list : vector< .m.S >;
    multi : multivector< 32, .n.S >;
    inner : archive .n.X;
}
}

"#;
pub mod resources {pub const SINGLE: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    single : .n.S;
}
}

"#;pub const LIST: &str = r#"namespace m {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    list : vector< .m.S >;
}
}

"#;pub const MULTI: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    multi : multivector< 32, .n.S >;
}
}

"#;pub const INNER: &str = r#"namespace n {
archive X
{
    payload : raw_data;
}
}

namespace a {
archive A
{
    inner : archive .n.X;
}
}

"#;}
}
}


/// Builtin union type of .n.S.
define_variadic_struct!(Multi, RefMulti, BuilderMulti,
    super::_builtin::multivector::IndexType32,
    0 => ( S, super::n::S, add_s));

define_archive!(A, ABuilder,
    schema::a::A;
    // struct resources
    (single, set_single,
        super::n::S,
        schema::a::resources::SINGLE, false);
    // vector resources
    (list, set_list, start_list,
        super::m::S,
        schema::a::resources::LIST, false);
    // multivector resources
    (multi, start_multi,
        Multi,
        schema::a::resources::MULTI,
        multi_index, super::_builtin::multivector::IndexType32, false);
    // raw data resources
;
    // subarchives
    (inner,
        super::n::X, super::n::XBuilder,
        schema::a::resources::INNER, false));

}

pub mod _builtin {

pub mod multivector {

pub mod schema {
pub mod structs {
pub const INDEX_TYPE32: &str = r#""#;}}
/// Builtin type to for MultiVector index
define_index!(
    IndexType32,
    RefIndexType32,
    RefMutIndexType32,
    schema::structs::INDEX_TYPE32,
    4,
    32
);

}

pub mod schema {
pub mod structs {}}
}
