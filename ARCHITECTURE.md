# Architecture

This document describes the high-level architecture of ***Hornvale***.

See also the [Book](https://ndouglas.github.io/hornvale/), which is a more aspirational document about what I want to accomplish and how I want to get there, and the [README](./README.md), which is a _very_ basic introduction to the project.

## Bird's Eye View

_Hornvale_ will become a game that accepts semi-natural-language input from a player and supplies output in the form of descriptive text about a fictional world – in short, a type of game variously referred to as a "text adventure" or "interactive fiction".

Text adventures and interactive fiction are normally concise, tightly-crafted works, although they vary substantially in scope and scale.  _Hornvale_ differs from these in that it tries to be a an "open world," "procedurally generated" text adventure.

I'm not under any illusions about how complex a task this is.  I expect that if I'm able to bring it to full fruition, it will only be after a period of several years, possibly decades.  It's that concern that guides the various architectural and design decisions; I'm aiming for maintainability and testability and debuggability pretty much _über alles_.

## Code Map

The following should be a 1:1 listing of the code in the `src/` subdirectory.

- [🪲 Macros](./src/_macros/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Macros developed for various uses.  Given the peculiarities of how macros are made available in Rust, I've segregated them to a specific folder.

- [🎭 Traits](./src/_traits/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Macros developed for various uses.  Given the peculiarities of how macros are made available in Rust, I've segregated them to a specific folder.

- [🎬 Actions](./src/actions/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Actions, in-game actions undertaken by entities with the intent to alter something about the game world.

- [🧍‍♂️ Anatomy](./src/anatomy/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Anatomy and physiology, health, damage, poison, and regeneration.

- [💫 Astronomy](./src/astronomy/README.md)&nbsp;<sup><sub><sub>🟠</sub></sub></sup>: Astronomical sciences, from the galaxy to the moon.

- [🧬 Biology](./src/biology/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Biological, taxonomy and related subjects.

- [🪦 Combat](./src/combat/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Combat system, some closely related topics.

- [✅ Commands](./src/commands/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: The input from the player is parsed and transformed into commands, which should map to various in-character actions or out-of-character queries of or modifications to the game state.

- [🧩 Components](./src/components/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Attributes and behaviors are shifted out of the Entity class and into small, focused types to promote composition and modularity.

- [🧭 Direction](./src/direction/README.md)&nbsp;<sup><sub><sub>🟡</sub></sub></sup>: Direction is a fundamental concept to navigation and description.

- [⛏️ Downdelving](./src/downdelving/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Dungeons, mining, and the Underdark.  Underground portions of the game pose different challenges than above-ground portions, and we deal with some of those issues here.

- [🏦 Economics](./src/economics/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Economic system, merchants, resources, scarcity.  I want shopkeepers to actually consider economic ideas, rather than act as a sink for currency.

- [💥 Effects](./src/effects/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Alterations to the world state should take certain pre-defined forms that can be tested for accuracy and correctness.

- [👽 Entity](./src/entity/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Entities are any objects that appear in the game.  This section comprises not so much entity _behavior_ (which should be handled primarily in the Components), but the creation and management of entities.

- [🎮 Game](./src/game/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Game state management.

- [🌋 Geology](./src/geology/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Geology and physical geography, for terrain generation at a macro and local scale.

- [🧠 Goal-Oriented Action Planning](./src/goap/README.md)&nbsp;<sup><sub><sub>🟠</sub></sub></sup>: When an entity selects a goal, this system can be used to select the action that they should take to move toward accomplishing it.

- [🔗 Input/Output](./src/io/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Input/Output interfaces are centralized so that we can easily adapt to new systems, e.g. playing through a telnet connection rather than running the application directly.

- [💬 Linguistics](./src/linguistics/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Linguistics system permitting communication, bidirectional translation, etc.

- [🗺️ Map](./src/map/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: The representation of the game world with regard to its most fundamental navigational components: rooms.

- [📜 Mythopoetics](./src/mythopoetics/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Narrative/mythopoetic procedural content generation and tools.  The end goal is being able to generate _interesting_ stories, plotlines, etc.

- [📝 Parsing](./src/parsing/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Parsing user input and transforming it into commands of arbitrary complexity.

- [🚪 Passage](./src/passage/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Passages can be somewhat complex; either hallways, doorways, lockable doors, hidden exits, not-very-visible exits, slow exits, exits that present a message when the user attempts to go in that direction, etc.

- [👁️ Perception](./src/perception/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Stimuli are processed by senses and perceived by individual entities.

- [⛹️ Player](./src/player/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: I try to universalize as much as possible of behavior between player-characters and non-player-characters, but there is _some_ stuff that is particular to the player.

- [📍 Room](./src/room/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: The Room concept and related tools.

- [💻 Scripting Language](./src/scripting_language/README.md)&nbsp;<sup><sub><sub>🟡</sub></sub></sup>: Embedded programming language (based on [Lox](https://www.craftinginterpreters.com/)) and domain-specific library.

- [🧑‍🤝‍🧑 Sociology](./src/sociology/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Social psychology and sociology, individual and group behavior.

- [👻 Supernatural](./src/supernatural/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Supernatural and metaphysical concepts, the thermodynamics of spirit.

- [🌿 User Interface](./src/ui/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Frontend and connective logic.  As little as possible.

- [🌎 World](./src/world/README.md)&nbsp;<sup><sub><sub>🔴</sub></sub></sup>: Data structure containing everything that happens within the in-game world.

**Status**: These indicators' meanings are subject to change as I progress.
 - <sup><sub><sub>🔴</sub></sub></sup>: I haven't even started.
 - <sup><sub><sub>🟠</sub></sub></sup>: I've laid the groundwork, or at least taken some initial steps.
 - <sup><sub><sub>🟡</sub></sub></sup>: It's serving some purpose, though far from complete.
 - <sup><sub><sub>🟢</sub></sub></sup>: Working, although I'll never really consider it "feature complete".
 - <sup><sub><sub>🔵</sub></sub></sup>: A vast radiant beach and cool jeweled moon, etc.  Some evenings I just watch the test suites as they run.
