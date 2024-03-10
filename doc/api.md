# Api

The api of the module is as followed.

## Command definition

Each command contain the following information:

| Name      | Type             | Description                                |
|-----------|------------------|--------------------------------------------|
| name      | `string`         | The name of the command                    |
| arguments | `string \| null` | The list of arguments given to the command |

The arguments can be like this:
```
--verbose --features benchmark run
```
And will be parsed to:
```js
["--verbose", "--features", "benchmark", "run"]
```

Or you can give the array, and the pre-processor will not be run.

## Command entered

A command or list of commands can be executed.

### Execute a single command

```js
mooncake.call_command({ name: "cd", arguments: "mooncake" })
```
Or
```js
mooncake.call_command({ name: "cd", arguments: ["mooncake"] })
```

The arguments can be undefined.

### Execute a list of commands

This is more fun to build, as it's "packed".

The allowed symbols between the commands are:

| Symbol | Description                                                                                                |
|--------|:-----------------------------------------------------------------------------------------------------------|
| `&&`   | Execute the second command after the first one if the first command ended without an error.                |
| `\|\|` | Execute the second command after the first one if the first command ended with an error.                   |                                                                                      |                                                                                            |
| `;`    | Execute the second command after the first one, whatever the first command ended with or without an error. |


So each command execution group is represented as:

| Name   | Type                             | Description                                             |
|--------|:---------------------------------|---------------------------------------------------------|
| first  | `Command`                        | The command which will executed in first                |
| symbol | `Symbol \| null`                 | The symbol between the first and last commands, if any. |
| second | `CommandPack \| Command \| null` | The second command which will be executed, if any.      |

Therefore, the api will be called like:
```js
mooncake.call_commands({
    first: { name: "javac", arguments: "Main.java"},
    symbol: "&&",
    second: {
        first: { name: "java", argumnets: "Main" } 
    }
})
```
```js
mooncake.call_commands({
    first: { name: "javac", arguments: "Main.java"},
    symbol: "&&",
    second: { name: "java", argumnets: "Main" }
})
```
```js
mooncake.call_commands({
    first: { name: "cd", arguments: "/home/mooncake/java/helloWorld/"},
    symbol: "&&",
    second: {
        first: { name: "javac", arguments: "Main.java"},
        symbol: "&&",
        second: { name: "java", argumnets: "Main" }
    }
})
```

but it can also be called like:
```js
mooncake.call_raw_commands("javac Main.java && java Main");
```
```js
mooncake.call_raw_commands("javac Main.java || echo 'error'");
```
And a pre-processor will parse the string to the object.

## Events

### Registering an event

```js
mooncake.events.add_global_listener("completion", function(app, { name, arguments, last_argument, tab }){
    // A dummy example
    if ("name" == "cd") {
        return ["mooncake", "mooncake2"]
    }
})
```
or like
```js
let id = "918efc9b-9249-4d39-bb20-8e88ecc46652";

mooncake.events.add_global_listener(id, "completion", function(app, { name, arguments, last_argument, tab }){
    // A dummy example
    if ("name" == "cd") {
        return ["mooncake", "mooncake2"]
    }
})
```

But for local events
```js
// The ID is a uuid
let id = "918efc9b-9249-4d39-bb20-8e88ecc46652";

// And the event is registered like this
mooncake.events.add_local_listener(id, "keydown", function(key_event){
    if (key.ctrl && key.code == "c") {
        mooncake.call_command({ name: "exit" })
    }
})
```

### Remove an event

```js
mooncake.events.remove_global_listener("completion", id)
```

Removing a local event is the same, but with `remove_local_listener` instead of `remove_global_listener`.

> Note that the `id` is optional, but if you want to remove a specific event, you will need to give an `id`.

### Fire an event

```js
mooncake.events.fire("completion", { name: "cd", arguments: "mooncake", last_argument: "moon", tab: false })
```

The arguments can be undefined or an empty object.

The `tab` is a boolean which is `true` if the tab key is pressed, and `false` otherwise.

