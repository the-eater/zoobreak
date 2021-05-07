#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::os::raw::c_int;
use std::ops::Deref;

// Entry point from Lua
#[no_mangle]
unsafe extern "C" fn zoobreak(l: *mut lua_State) -> i32 {
    let lua = Lua(l);
    // Create Lua table
    let table = Table::new(lua);
    // Set the key "gamer" to "gamers"
    table.set("gamer", LString::new(lua, "gamers"));
    // Push table to top, so it's seen as return value
    table.to_lref().push();
    1
}


// Dumb Lua helpers

#[derive(Copy, Clone)]
struct Lua(*mut lua_State);

impl Deref for Lua {
    type Target = *mut lua_State;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone)]
struct LRef {
    lua: Lua,
    index: c_int,
}

trait ToLRef {
    fn to_lref(&self) -> LRef;
}

impl LRef {
    pub fn top(lua: Lua) -> LRef {
        LRef {
            lua,
            index: unsafe { lua_gettop(*lua) },
        }
    }

    pub fn push(&self) {
        unsafe {
            lua_pushvalue(*self.lua, self.index)
        }
    }
}

struct LString(LRef);

impl ToLRef for LString {
    fn to_lref(&self) -> LRef {
        self.0
    }
}

impl LString {
    pub fn new<S: ToString>(lua: Lua, val: S) -> LString {
        let str = val.to_string();
        unsafe { lua_pushlstring(*lua, str.as_ptr() as *const _, str.bytes().len() as u32) }
        LString(LRef::top(lua))
    }
}

struct Table(LRef);

impl Table {
    pub fn new(lua: Lua) -> Table {
        unsafe { lua_newtable(*lua) }
        Table(LRef::top(lua))
    }

    pub fn set<S: ToString, V: ToLRef>(&self, name: S, value: V) {
        // Push key to stack
        LString::new(self.0.lua, name);
        // Push value to stack
        value.to_lref().push();
        // Set field on table
        unsafe { lua_settable(*self.0.lua, self.0.index) }
    }
}

impl ToLRef for Table {
    fn to_lref(&self) -> LRef {
        self.0
    }
}