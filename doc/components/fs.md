# File System

The file system is similar as UNIX.

The file system can be built as a Rooted tree like:

*for simplicity, executables are preceded with `(-)` and links starts with with `*`*
```
/ (root)
├── home
│   ├── *hello.py // will point to `/home/mooncake/Code/hello.py`
│   ├── .local
│   │   └── bin
│   │       ├── fuck     (-)
│   │       └── *thefuck
│   ├── Documents
│   │   └── important_document.odt
│   └── Code
│       └── hello.py
├── usr
│   └── local
│       ├── python3 (-)
│       ├── pip3    (-)
│       ├── lua     (-)
│       └── luvit   (-)
└── bin
    ├── cd  (-)
    ├── pwd (-)
    ├── rm  (-)
    └── ls  (-)
```

## Permissions

Like UNIX, the permissions given to a file can be either read, write or execute.

Certain files, such as the files located in `/bin/` are not meant to be deleted and therefor will only have the read and execute permissions.

## Type of files

Here is the types of files allowed:

| Name       | Description                                       |
|------------|:--------------------------------------------------|
| File       | A simple file, no need to give more informations. |
| Directory  | A directory (new node) which contain files        |
| Symlink    | A link to a file or directory                     |
| Executable | An executable file                                |

### Executable

As it's not meant to be a real shell that we can found on your favorite OS, executable are only part of an enums (above).

But the main changes is that, when called, the executable will call a function :)

> If there is no functions, well, fuck it: Error.

## File Metadata

The permissions of the file, but also the creation/edition timestamp and other information are stored in the metadata of each file (also directory and links).

| Name        | Type           | Description                                                                      |
|-------------|----------------|----------------------------------------------------------------------------------|
| created     | `Date`         | The creation date                                                                |
| edited      | `Date \| null` | The edited date, if any                                                          |
| permissions | `u8`           | The permissions are a unsigned int on 8 bits. Please refer to `File Permissions` |


## File permissions

| Permission type | Alias | Operation | Description            |
|-----------------|-------|-----------|------------------------|
| Read            | r     | 0 \| 1    | The read permission    |
| Write           | w     | 0 \| 10   | The write permission   |
| Execute         | x     | 0 \| 100  | The execute permission |

Example:
```js
assert(0 | 10 | 1       == 0b011); // Read and write permissions
assert(0 | 100 | 1      == 0b101); // Read and execute permissions
assert(0 | 10 | 100 | 1 == 0b111); // Read, write and execute permissions
assert(0 | 100          == 0b100); // Execute permission
```


## API

### IO Error

Here is a non-exclusive list of error:

| Name        | Code | Description                                                           |
|-------------|------|-----------------------------------------------------------------------|
| CannotWrite | 101  | If the file cannot be written, often because of permissions required. |
| CannotRead  | 102  | If the file cannot be read, often because of permissions required.    |
| NotFound    | 103  | If the file doesn't exist.                                            |
| NotUtf8     | 104  | If the file content cannot be transformed into a utf8 string.         |


### Getting a file

```js
mooncake.fs.read_file_string("/home/Code/hello.py");
```

Can fire the `CannotRead` or `NotUtf8` error.

If you want to read a file and get his buffer:
```js
mooncake.fs.read("/home/Code/hello.py");
```

### Reading a file's metadata

```js
mooncake.fs.metadata("/home/Code/hello.py");
```
```js
mooncake.fs.metadata("/home/Code")
```

### Writing a file

If you want to write a file and truncate any content left after:
```js
mooncake.fs.write_all("/home/Code/hello.py", "print('Hello from python')");
```

You can pass these optional arguments:

| Name     | Type    | Default | Description                                                                                   |
|----------|---------|---------|-----------------------------------------------------------------------------------------------|
| truncate | boolean | false   | Whether or not the file content should be truncated. Always true with the method `write_all`. |
| create   | boolean | true    | Whether or not the file should be created if it doesn't exist                                 |

The `write_all` method can fire the `CannotWrite` error.

### Creating a directory

```js
mooncake.fs.mkdir("/home/Code");
```

If you want to create a directory and its parents:
```js
mooncake.fs.mkdir_all("/home/Code/Rust/MoonCake"); // will create 'Code', 'Rust' and 'MoonCake' if they don't exist
```

The `mkdir` and `mkdir_all` methods can fire the `CannotWrite` error.

### Removing a file

```js
mooncake.fs.rm("/home/Code/hello.py");
```

The `rm` method can fire the `CannotWrite` or `NotFound` error.

### Removing a directory

```js
mooncake.fs.rmdir("/home/Code");
```

The `rmdir` method can fire the `CannotWrite` or `NotFound` error.

