mcrs (tentative name)
----

A Minecraft: Java Edition server written in Rust.

## Goals
- Use this to actually learn to program Rust "in the large", instead of "in the small" things like Advent of Code
- Explore alternative implementation strategies for things versus vanilla, such as perhaps using Bedrock Edition's ECS definitions for entities.

## Acknowledgements
- [wiki.vg](https://wiki.vg/Protocol) for mostly black-box documentation of the Java Edition protocol
- [burger](https://pokechu22.github.io/Burger/) for automated data extraction from the client jar
- See `cheats.md` for more

## FAQ
### What's the license?
I am an experienced Java Edition modder who has significant exposure to the decompiled and reverse-engineered Java Edition codebase.
As such, I don't think it would be accurate to say that anything I produce is "clean-room" and free from the influence of Java Edition's design decisions.
This project exists in the same legal gray area as Minecraft mods do.
As such, while the copyright of the code belongs to me, there is no defined license. (You probably don't want to be using this anyways..)

### Why no async network?
The maximum a fully-featured MC server can support is only around 100-200 people, which thread-per-connection can handle definitely.
Async is not free and comes with lots of roughness and additional cognitive load, especially since the ecosystem is so young.
*If* I am ever forced to switch to async for performance, this project must have made it big :P.

### What about (some other Rust implementation)?
I've spent way too long working on other people's projects instead of starting and guiding myself through my own. It's something
professional developers should be able to do, and this is a test of my own skills. So yes, there may be other better implementations
out there, but I'm going to stick to my own, because writing it myself would teach me much more than reading what someone else wrote.
Additionally, I don't think there's many Rust impls out there besides `hematite_server`, which is quite dead.
