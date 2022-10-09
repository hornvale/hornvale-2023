# Hornvale
Hornvale is an experimental, idiosyncratic game written in Rust.  I was referring to it as a "prose-based roguelike", but now I'm thinking it might be more like "open world interactive fiction".  

I don't know if this is stupid or not.  It's a more personal project than I typically throw on GitHub, and I'm making it up as I go.

## Major Ideas
The main thing I want to play with, that ties this to the roguelike tradition, is procedural content generation.  But I'm also deeply embedded in interactive (and non-interactive) fiction, and MUDs, both of which tend to be intentionally authored experiences.  The former is deeply invested in a notion of efficient storytelling, the latter in a more open-world concept with multiple narratives.  I'd like to explore this area and figure out what sort of messes I can cause.

This is likely to be an incredible amount of work, and frankly, I don't really have a good history with regard to actually completing personal projects, so feel free to just cruise right by this one.  Also, the fact that I've Unlicensed this is probably a good indicator of how much commercial appeal this project has.

But maybe this will be something that grows over time into something worth looking at.

## Prior Incarnation
A previous iteration, purely exploratory, can be found [here](https://github.com/ndouglas/hornvale-rust/).  It was/is mostly just investigatory, figuring out what was possible, and whether I could bear to do it in Rust.

## Hornvale Project
- [Hornvale](https://github.com/ndouglas/hornvale/): Frontend and connective logic
  - [Goldengrove](https://github.com/ndouglas/goldengrove/): Narrative/mythopoetic procedural content generation and tools
  - [Lasthearth](https://github.com/ndouglas/lasthearth/): Embedded programming language and domain-specific library
  - [Breakwater](https://github.com/ndouglas/breakwater/): Environmental procedural content generation and tools
  - [Brownhollow](https://github.com/ndouglas/brownhollow/): Artificial life/social/factional/economic procedural content generation and tools
  - [Honeyholt](https://github.com/ndouglas/honeyholt/): Tool for humanizing numbers, concepts, etc.
  - [Lemonwood](https://github.com/ndouglas/lemonwood/): Flexible and powerful test harness for roguelike projects.
  - [Volmark](https://github.com/ndouglas/volmark/): Debugging and other macros.

## FAQ

#### Why is this project structured this way?
Because I'm an idiot, most likely.  Don't look for a method here.  I like over-structuring things.  

If at some point I think this is worth having other people contribute, I'll fix it so that the dependency graph is sane.  

Right now it's optimized for my current workflow, which is tweaking package A and having the build fail, then tweaking package B to fix the problem and breaking the build in a different way, then tweaking package C to fix that and breaking it in a still different way, and then me collapsing and sobbing.  

When this stabilizes a bit, and becomes more about adding features and less about figuring out really basic concepts, then the dependency graph should become a little more normal.

Or I might just merge it all into a single repo.  I don't know.

If you're reading this, I just haven't gotten far enough to where anyone but me should care about any of this.

