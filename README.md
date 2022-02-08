# crate

示例1 — 引入内部包  
|——`src`  
|————`src_a`  
|——————`a.rs`  
|——————`c.rs`  
|————`src_b`  
|——————`b.rs`  
|————`src_a.rs`  
|————`src_b.rs`  
|————`main.rs`  

`a.rs`
```
pub fn a_echo(){
    println!("a_echo!");
}
```
`c.rs` crate不能用self替换crate
```
use crate::src_a::a::*;
pub fn c_echo(){
    println!("c_echo!");
    a_echo();
}
```
`b.rs`
```
use crate::src_a::a_echo;
//如何引入a.rs或c.rs中的函数
pub fn b_echo(){
    println!("b_echo! => call a()!");
    a_echo();

}
```
`src_a.rs`
```
pub mod a;
pub mod c;
pub use a::*;
pub use c::*;
```
`src_b.rs`
```
pub mod b;
pub use b::*;
```
`main.rs` crate和self可以互相替代
```
pub mod src_a;
pub mod src_b;
pub use self::src_a::*;
pub use crate::src_b::*;
fn main() {
    println!("Hello, world!");
    src_a::a_echo();
    src_b::b_echo();
}
```
示例2 — 引入外部包  
|——`d`  
|————`src`  
|——————`lib.rs`  
|——`src`  
|————`src_a`  
|——————`a.rs`  
|——————`c.rs`  
|————`src_a.rs`  
|————`main.rs`  

`lib.rs`
```
pub fn d_echo(){
    println!("d_echo!");
}
```
`Cargo.toml` 要在rust_test的toml文件中进行修改，增加对d的依赖
```
[dependencies]
d = { path = "../rust_test/d" }
```
`main.rs`
```
pub mod src_a;
pub mod src_b;
pub use self::src_a::*;
pub use crate::src_b::*;
// main调用src_out里面文件的函数
use d::{d_echo}
fn main() {
    println!("Hello, world!");
    src_a::a_echo();
    src_b::b_echo();
    d_echo();
}
```
示例3 — `mod.rs`   
|——`src`  
|————`src_a`  
|——————`a.rs`  
|——————`c.rs`  
|————`src_e`  
|——————`e.rs`  
|————`src_a.rs`  
|————`src_b.rs`  
|————`main.rs`  

`a.rs`
```
pub fn a_echo(){
    println!("a_echo!");
}
```
`c.rs` crate不能用self替换crate
```
use crate::src_a::a::*;
pub fn c_echo(){
    println!("c_echo!");
    a_echo();
}
```
`e.rs`
```
use crate::src_a::a_echo;
//如何引入a.rs或c.rs中的函数
pub fn e_echo(){
    println!("b_echo! => call a()!");
    a_echo();

}
```
`src_a.rs`
```
pub mod a;
pub mod c;
pub use a::*;
pub use c::*;
```
`src_e.rs`
```
pub mod e;
pub use e::*;
```
`main.rs` crate和self可以互相替代
```
pub mod src_a;
pub mod src_e;
pub use self::src_a::*;
pub use crate::src_e::*;
fn main() {
    println!("Hello, world!");
    src_a::a_echo();
    e::e_echo();
}
```
