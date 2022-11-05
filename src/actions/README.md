# Actions

**Actions** are "in-character", world-affecting things, not to be confused with [Commands](../commands/README.md), although both use the [Command](https://gameprogrammingpatterns.com/command.html) design pattern.

Regardless of the actor, **Actions** will generally trigger **Effects**, which are modifications to the world or the things within it.

Actions should be:
- agnostic to the reasons they are being performed
- agnostic to the world state they reference
- atomic, not trying to do more than one thing

Actions should not:
- change the world state directly

```mermaid
flowchart TD
  A[Action] --> B[Effect]
```