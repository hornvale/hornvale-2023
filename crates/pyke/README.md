# Pyke

I think I can start out with a sort of Maslow's hierarchy of needs, an abstract calculus for figuring out roughly what a creature's concerns are at any given point in time and weighing them accordingly.

I don't have a good approach in mind for how to do this yet.  I suppose that each creature has an awareness of their context, and this awareness informs their motivations.  They have some varying ability to delay gratification, so they don't abandon something that's 99% done to go eat a snack at the first pang of hunger, then start over.

This gets very complicated, though, when we think of all of the possible circumstances that might occur in a creature's context, and all of the different ways these might be weighted.

One way of handling this generically might be a sort of multilevel priority queue.  Briefly, we could loop through the hierarchical levels (Physiological, Safety, Social, Esteem, etc) and see which messages exist and their priority levels.  These priority levels might be similar to e.g. RFC 5424 levels (debug, info, notice, warning, error, critical, alert, emergency).

These priority levels should then be weighted relative to one another across the hierarchical levels, creating a table.  This can vary by species, so that self-actualization is far less important than social needs for certain species, and possibly might vary per individual as well.  That can allow re-sorting the "pyramid" model.

As an actor moves through the world, they receive updates from their context.  They incorporate this, through the subjective filter of their weighting, into an internal reflection of the state of the world, with the most significant weighted messages having precedence.  If we say they're at a certain level, that means that they've had no messages more significant than that level any time recently.  (Everyone is always at _at least_ INFO; let's just think of DEBUG and TRACE as unconscious.)

For instance, let's say that a goblin is walking through the woods.  

- Physiologically, he is at NOTICE levels of hunger (he could go for something to eat, but not yet at the tummy-rumbling stage), breathing fine, warm, dressed sufficiently.  Life is good.  

- In Safety terms (and in a game like this, we must be careful to note that Safety does not relate to combat, which is physiological, but rather day-to-day safety.  I should rename these, really), the goblin is not entirely at ease -- a neighboring tribe of hobgoblins has been encroaching on their territory, and while no actual acts of aggression have taken place, there's a general feeling of unease among the goblins.  So let's say he's at NOTICE there, too.

- In Social terms, the goblin's doing fine.  He's an integrated member of his community, and his hovel is relatively near the chief's hut and upwind of the latrine.  He'll be attuned to any slights (INFO) but is fairly comfortable.

- In Esteem, the goblin's also doing fine.  He has three goats and a baby on the way.  Let's say he's at INFO.

- Let's say that goblins, as a species, just aren't terribly concerned with cognitive needs, self-actualization, etc.  They might try to improve their hunting and goat-herding skills 🤔  We might envision this as giving self-actualization very little additional weighting at the higher priority levels.  Basically, a goblin never sees his fundamental lack of development as anything much more serious than a NOTICE level.  Our goblin has noticed recently that he's not managing his goats as well as he could, so let's say he's at a NOTICE level.

So, to calculate, we might have a table like the following for goblins:

| Priority | Physiological | Safety | Social | Personal | Higher |
|--|--|--|--|--|--|
| Debug | 0.01 | 0.01 | 0.01 | 0.01 | 0.01 |
| Info | 0.08 | 0.05 | 0.05 | 0.03 | 0.02 |
| Notice | 0.2 | 0.18 | 0.17 | 0.15 | 0.1 |
| Warning | 0.4 | 0.3 | 0.2 | 0.18 | 0.12 |
| Error | 0.8 | 0.7 | 0.55 | 0.4 | 0.14 |
| Critical | 0.9 | 0.75 | 0.6 | 0.5 | 0.16 |
| Alert | 0.95 | 0.8 | 0.7 | 0.6 | 0.18 |
| Emergency | 1.0 | 0.9 | 0.8 | 0.7 | 0.2 |

(This is just a rough idea, it would obviously need to be tuned considerably.)

So our goblin is at NOTICE/NOTICE/INFO/INFO/NOTICE overall, which are weighted 0.2, 0.18, 0.05, 0.03, 0.1.

This means he'd act to bring his Physiological state to the desired state.  So he grabs an apple and eats it.  Then he's at 0.08/0.18/0.05/0.03/0.1.

So he might then do something to bring his Safety state to the desired state.  He sniffs around for signs of bugbears.

If he smells some, this is likely to arouse his feelings of personal immediate danger, raising it to WARNING, and his feelings of safety, raising it to WARNING as well.  He would then behave differently: move more stealthily.  He might head back to his village to confer.  This might change the behavior of other goblins; they might travel only in groups of 2-3 or even 6-8, to increase their feelings of safety.

If not, then Safety should get improved as well, to Info.  At this point, his Higher motivations kick in, and perhaps he inspects his goats for signs of poor health.

So, implementing this practically, we can view the psychological state of the goblin as a 64-bit bitfield, with the most significant bit being PHYSIOLOGICAL EMERGENCY and the least significant bit being TRANSCENDENT TRACE (I might need to work on these naems).  Each of these has a float assigned, stored within the individual and derived from a species-level prototype, modified slightly by chance, and possibly modified by past experience.

Each turn, the bitfield is iterated and the floats corresponding to the set bits are compared.

Some more thinking:

- Each sentient entity begins as a tabula rasa.
- As events occur in the world, these events are sensed by entities according to their sensation rate (which varies i.e. inversely with distance from player).
- These sensations are recorded as information in the sentient entity's consciousness along two axes: which level of the creature's hierarchy of needs is affected, and the severity of the information (e.g. it is possible to have mild hunger events and severe self-actualization events).
- The eight needs are as follows:
  - Programmatic (so to enable direct control of the entity)
  - Physiological
  - Security
  - Social
  - Esteem
  - Cognitive 
  - Reserved I
  - Reserved II
- The eight levels of severity are as follows:
  - Trace/Debug: "*yawn*" (Unused, most likely)
  - Info: "Good to know..." (Background information [There are strawberries ripening in the fields.  There is grass here.])
  - Notice: "Hmm, interesting." (A discrete fact with no ominous implications. [It has started raining. You smell strawberries on the breeze.  Your stomach growls.])
  - Warning: "Uh-oh." (Some ominous implications. [You found some fresh Bugbear spoor.  You are hungry.])
  - Danger: "Oh, crap!" (Clear and present danger. [A bugbear enters from the north, sword drawn!  You are very hungry.])
  - Critical: "Oh, shit!" (Shit is flying toward the fan. [The bugbear attacks!  You are starving!])
  - Alert: "OH SHIT!" (Shit is hitting the fan. [The bugbear hits you for 23 damage!  You are bleeding!])
  - Emergency: "I'm terrified beyond the capacity for rational thought." (The fan is submerged in shit. [The bugbear hits you again! You are bleeding AWFULLY!])
- A bit is set in a 64-bit bitfield when information matching the corresponding need/severity is introduced.
- Every tick of the entity's planning rate, the entity determines its most significant concern and develops a plan to address the concern.
- Determining the most significant concern consists of the following:
  - Iterate through the information bitfield to find the most significant bits that are set.
- Developing a plan consists of the following:
  - Identifying the goal state that relieves the concern.

More thoughts:

1. Each entity springs into existence as a _tabula rasa_.
2. Sensory events are emitted by the world and things in it.
3. Each entity has a perceptual filter that passes, blocks, or transforms sensory events.
4. Each entity has a perceptual receiver that calculates an _actual_ representation of the world state based on the received sensory events. [Note: this sounds a lot like functional reactive programming.  I might need to dedicate some time to working on that specifically.]
6. Each entity has a _desired_ representation of the world state as well.
7. The entity has a dissonance system that compares the actual and desired representations of the world state and creates events corresponding to these differences. [More FRP.]
8. These discrepancies form the Goal Set.
9. The entity will formulate an Action Set to resolve the discrepancy.  This is normally derived directly from the entity's species or something like that, but the entity can modify this.  If he has the "cowardly" trait, or if both of his arms have been severed, being aggressive might not make any sense.
10. The planner will review the Action Set and Goal Set and formulate a course of Actions to take to achieve the Goal state.
