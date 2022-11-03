# Goal-Oriented Action Programming

This is a simple GOAP system that I'll try out with my NPCs.

The idea is that I'll choose some other mechanism -- behavior tree, state machine, whatever -- to determine the NPC's concerns at any given time, to pick a goal.  Then I'll use this system to get the NPC to perform actions to achieve that goal.

I'm hoping that I can use this to orchestrate some fairly complex behaviors in interesting ways.

GOAP world state should be calculated directly from the actor's chalkboard (commonly referred to as world state, but I find that term misleading).

GOAP Actions should not be confused with [Actions](../actions/README.md).  Normally there's a correlation, perhaps a 1:1 relationship, but GOAP Actions are more lightweight and contain less context about the world.