# Events

Here is the list of the available events that can be listened:

| Name            | Arguments                           | Description                             |
|-----------------|-------------------------------------|-----------------------------------------|
| command_entered | `Command \| CommandExecutionGroup`  | Called each time a command is fired     |
| completion      | `(Command, string[], string, tab?)` | Called when the completion is required. |
| keydown         | `KeyboardEvent`                     | Called when a key is pressed.           |
| keyup           | `KeyboardEvent`                     | Called when a key is released.          |

## Completion

The completion is fired each time a key is pressed.

For example, if I press `cd /home/D`, the completion event will be fired and, if no plugins respond with the next characters proposed, the default completion handler will respond.

## Keyboard events

These events are allowed only for executables.
