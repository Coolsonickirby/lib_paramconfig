#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]

#[macro_use]
extern crate lazy_static;


use smash::{
    hash40,
    app::{lua_bind::*, *},
    lib::lua_const::*
};
use std::{
    collections::HashMap, 
    sync::Arc,
    arch::asm,
};
use parking_lot::RwLock;

pub mod data;
mod hook;

#[skyline::main(name = "libparam_config")]
pub fn main() {
    println!("[libparam_config::main] Loading...");
    if data::install() {
        println!("[libparam_config::main] Hooking...");
        hook::install();
    }
    else{
        println!("[libparam_config::main] No commonconfig data found");
    }
}