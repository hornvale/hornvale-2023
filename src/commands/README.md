# Commands

**Commands** here are specifically used for transforming user input into IC and OOC [Actions](../actions/README.md).

From the user's perspective, Commands are mapped to [Actions](../actions/README.md), which might then have [Effects](../effects/README.md) on the world and the objects in it.

Commands also convey context about the player to the action.

Command execution returns a `Result<Option<String>, Error>`; this is so that errors and immediate responses ("You see nothing unusual about the amulet.") can be returned immediately, outside of the Command -> Action -> Effect -> Event chain.

Things I want from Commands:
- easily extensible.  Add a command in one place, two places max.
- easily testable.  Should be able to create a test with minimal boilerplate.
- centralized.  Parser/interpreter shouldn't struggle to find it.

Thinking about structuring it this way:

```rust
pub enum Command<E: Engine> {
  LookAround(LookAround<E: Engine>)
}

pub struct LookAround<E: Engine> {

}
```
