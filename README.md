# Zoobreak

A PoC Lua interpreter break out for Zoo Tycoon 2

It's Just Normal Lua Things, but like, 15 years later, so uh. I guess it's interesting.

## Explanation

Zoo Tycoon contains a Lua 5.0.2 interpreter which executes scripts located in zip and z2f files in Zoo Tycoon 2 program
folder.

Some entries to run custom code has already been found, and can be found on other places.

One of the functions this Lua interpreter exposes is `loadlib` with which one can load C symbols from libraries,

Zoobreak defines a C symbol called `zoobreak`, which can then be loaded via `loadlib`. the result of this `loadlib` call
can then be executed and `zoobreak` gets access to the `lua_State` variable.

`zoobreak` loads it's own Lua 5.0.3 library since Zoo Tycoon 2 doesn't expose the symbols, and can execute actions on
the stack, and return a valid table.

The Lua would looks something like the following

```lua
--- Returns the C symbol to the rust code
local zoobreak_entry = loadlib([[zoobreak.dll]], "zoobreak")
--- Creates the "module" table
local zoobreak = zoobreak_entry()
--- Returns "gamers"
zoobreak.gamer
```

### Building

Make sure you have MSVC (rustup will point you), rustup (with the i686-pc-windows-msvc toolchain) and clang installed,

```bash
cargo build
```

After building copy the resulting `zoobreak.dll` from `target/i686-pc-windows-msvc/debug/build` and `lua50.dll`
from `lua50` and place them into the Zoo Tycoon 2 progam folder next to `zt2.exe`.

Choose a lua entry point, and the code shown above should work