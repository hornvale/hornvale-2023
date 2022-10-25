# Blackpool

Blackpool is an experimental programming language, VM, and domain-specific library.

As part of my work on **Hornvale**, I want to write a scripting language that is:

- powerful
- performant
- tailored to the subject at hand, which is creating highly extensible virtual worlds

I can afford to compromise the first and second points for the third somewhat, given the nature of Hornvale.

This work is based on **Lox**, from Robert Nystrom's amazing [Crafting Interpreters](https://craftinginterpreters.com), and specifically his bytecode-based CLox, but I'm customizing it heavily based on my personal preferences and intended use case, and adding bindings for functionality in other projects.

It should be noted, though, that I'm less interested in matching the reference implementation of Lox than I am in incorporating a full-featured and flexible scripting language.  I have slightly different syntax in mind than Bob has for Lox, and I'm going to add some domain-specific extensions, etc.   So **TL;DR**: if something's weird or ugly or broken, it's me and not Bob.  All hail Bob.  All... hail... Bob.
