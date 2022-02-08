extern crate d;

pub use self::src_a::*;
pub use self::src_b::*;

pub mod src_a;
pub mod src_b;

pub mod src_e;
pub use self::src_e::*;

pub mod f;
pub mod g;

fn main() {
    println!("Hello world!");
    src_a::a_echo();
    src_b::b_echo();
    d::d_echo();
    e::e_echo();
    f::f_echo();
    g::g_echo();
}