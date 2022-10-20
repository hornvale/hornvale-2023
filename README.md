
![CI](https://github.com/ndouglas/hornvale/actions/workflows/continuous_integration.yml/badge.svg?branch=main) [![codecov](https://codecov.io/gh/ndouglas/hornvale/branch/main/graph/badge.svg?token=YP8GDSHG73)](https://codecov.io/gh/ndouglas/hornvale)

# Hornvale
Hornvale[📖](https://ndouglas.github.io/hornvale/)  is an experimental, idiosyncratic game written in Rust.  I was referring to it as a "prose-based roguelike", but now I'm thinking it might be more like "open-world interactive fiction".  

I don't know if this is stupid or not.  It's a more personal project than I typically throw on GitHub, and I'm making it up as I go.

For more (a lot more) on my approach to the project, check out the [book](https://ndouglas.github.io/hornvale/), which is pretty much what's going to serve as documentation.

## Major Ideas
The main thing I want to play with, that ties this to the roguelike tradition, is procedural content generation.  But I'm also deeply embedded in interactive (and non-interactive) fiction, and MUDs, both of which tend to be intentionally authored experiences.  The former is deeply invested in a notion of efficient storytelling, the latter in a more open-world concept with multiple narratives.  I'd like to explore this area and figure out what sort of messes I can cause.

This is likely to be an incredible amount of work, and frankly, I don't really have a good history with regard to actually completing personal projects, so feel free to just cruise right by this one.  Also, the fact that I've Unlicensed this is probably a good indicator of how much general appeal this project has.

But maybe this will be something that grows over time into something worth looking at.

## Prior Incarnation
A previous iteration, purely exploratory, can be found [here](https://github.com/ndouglas/hornvale-rust/).  It was/is mostly just investigatory, figuring out what was possible, and whether I could bear to do it in Rust.

## Hornvale Subprojects
I know these names are somewhat opaque, but my thinking is that I decouple the name from a concrete name like "astronomy" and gain some flexibility with the borders.

- [🧬 Appleton](./appleton/README.md): Biological/psychological, up to Maslow's hierarchy of needs stuff.
- [💻 Blackpool](./blackpool/README.md): Embedded programming language and domain-specific library.
- [🏦 Casterlyrock](./casterlyrock/README.md): Economic system, merchants, resources, scarcity.
- [🧍‍♂️ Dreadfort](./dreadfort/README.md): Anatomy and physiology, health, damage, poison, and regeneration.
- [📜 Goldengrove](./goldengrove/README.md): Narrative/mythopoetic procedural content generation and tools.
- [📖 Harlaw](./harlaw/README.md): The book that I'm writing alongside this and using to guide my thoughts and ideas, etc.
- [🍯 Honeyholt](./honeyholt/README.md): Tool for humanizing numbers, concepts, etc.  Syntactic sugar.
- [🌿 Ivyhall](./ivyhall/README.md): Frontend and connective logic.  As little as possible.
- [🪦 Kingsgrave](./kingsgrave/README.md): Combat system, some closely related topics.
- [💬 Lemonwood](./lemonwood/README.md): Linguistics system permitting communication, bidirectional translation, etc.
- [👻 Ramsgate](./ramsgate/README.md): Supernatural and metaphysical concepts, the thermodynamics of spirit.
- [💫 Starfall](./starfall/README.md): Astronomical sciences, from the galaxy to the moon.
- [🌋 Stonedance](./stonedance/README.md): Geology and physical geography.
- [🐛 Volmark](./volmark/README.md): Debugging and other macros.
- [🧑‍🤝‍🧑 Weepingtown](./weepingtown/README.md): Social psychology and sociology, individual and group behavior.

## FAQ

#### Why are you generating stellar neighborhoods for an interactive fiction project?
Because I'm an idiot, most likely.

#### Do you have any code coverage eyecandy?
Uh, yeah, sure.  

![Code Coverage Graph](https://codecov.io/gh/ndouglas/hornvale/branch/main/graphs/icicle.svg?token=YP8GDSHG73)
