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

### Execute a list of commands

This is more fun to build, as it's "packed".

The allowed symbols between the commands are:

| Symbol | Description                                    |
|--------|:-----------------------------------------------|
| &&     | Execute the second command after the first one |


So each command execution group is represented as:

| Name   | Type              | Description                                             |
|--------|:------------------|---------------------------------------------------------|
| first  | `Command`         | The command which will executed in first                |
| symbol | `Symbol \| null`  | The symbol between the first and last commands, if any. |
| second | `Command \| null` | The second command which will be executed, if any.      |



