
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
- [🧬 Biology](./src/biology/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Biological, taxonomy and related subjects.
- [💻 Scripting Language](./src/scripting_language/README.md)&nbsp;<sup><sub><sub>🟡</sub></sub></sup>: Embedded programming language (based on [Lox](https://www.craftinginterpreters.com/)) and domain-specific library.
- [🏦 Economics](./src/economics/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Economic system, merchants, resources, scarcity.
- [⛏️ Downdelving](./src/downdelving/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Dungeons, mining, and the Underdark.
- [🧍‍♂️ Anatomy](./src/anatomy/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Anatomy and physiology, health, damage, poison, and regeneration.
- [📜 Mythopoetics](./src/mythopoetics/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Narrative/mythopoetic procedural content generation and tools.
- [📖 The Book](https://ndouglas.github.io/hornvale/)&nbsp;<sup><sub><sub>🟠</sub></sub></sup>: The book that I'm writing alongside this and using to guide my thoughts and ideas, etc.
- [🌿 User Interface](./src/ui/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Frontend and connective logic.  As little as possible.
- [🪦 Combat](./src/combat/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Combat system, some closely related topics.
- [💬 Linguistics](./src/linguistics/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Linguistics system permitting communication, bidirectional translation, etc.
- [🧠 Goal-Oriented Action Planning](./src/goap/README.md)&nbsp;<sup><sub><sub>🟠</sub></sub></sup>: Part of psychology and Artificial Intelligence.
- [👻 Supernatural](./src/supernatural/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Supernatural and metaphysical concepts, the thermodynamics of spirit.
- [💫 Astronomy](./src/astronomy/README.md)&nbsp;<sup><sub><sub>🟠</sub></sub></sup>: Astronomical sciences, from the galaxy to the moon.
- [🌋 Geology](./src/geology/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Geology and physical geography, for terrain generation at a macro and local scale.
- [🧑‍🤝‍🧑 Sociology](./src/sociology/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Social psychology and sociology, individual and group behavior.
- [📍 Room](./src/room/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: The Room concept and related tools.
- [🔗 Input/Output](./src/io/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Various forms of Input/Output.

**Status**: These indicators' meanings are subject to change as I progress.
 - <sup><sub><sub>🔴</sub></sub></sup>: I haven't even started.
 - <sup><sub><sub>🟠</sub></sub></sup>: I've laid the groundwork, or at least taken some initial steps.
 - <sup><sub><sub>🟡</sub></sub></sup>: It's serving some purpose, though far from complete.
 - <sup><sub><sub>🟢</sub></sub></sup>: Working, although I'll never really consider it "feature complete".
 - <sup><sub><sub>🔵</sub></sub></sup>: A vast radiant beach and cool jeweled moon, etc.  Some evenings I just watch the test suites as they run.

## FAQ

#### Why are you generating stellar neighborhoods for an interactive fiction project?
Because I'm an idiot, most likely.

#### Is this singleplayer or multiplayer?
Singleplayer.  A lot of this is informed by MUDs, but their nature (being multiplayer and easy to join) forces some design decisions on MUDs that I don't think I want to follow.  For instance, I want to largely avoid grinding.  Incredible levels of grinding can be required in MUDs because of Massively Online players, and I don't think that farming XP or skills is the experience I want to create.

#### Are you really calling this _Hornvale_?  What does that even mean?
I use castle names from George R. R. Martin's _A Song of Ice and Fire_ for project names.  Gives me something interesting and at least semi-evocative, but also narrows my choices so I don't spend all damned day bikeshedding the project name.

So "Hornvale" might be a codename, it might be the name of the game for all eternity, I might rename this at some point to _Seymour Butts in the Festival of Massacres II: Revenge of the Soulslurpers_.  No idea.  I just don't care.  I already spend too much time naming things.

#### Do you have any code coverage eyecandy?
Uh, yeah, sure.  Oddly specific.

![Code Coverage Graph](https://codecov.io/gh/ndouglas/hornvale/branch/main/graphs/icicle.svg?token=YP8GDSHG73)

