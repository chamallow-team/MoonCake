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

TODO

| Name       | Description                                       |
|------------|:--------------------------------------------------|
| file       | A simple file, no need to give more informations. |
| directory  | A directory (new node) which contain files        |
| link       | A link to a file or directory                     |
| executable | An executable file                                |

### Executable

As it's not meant to be a real shell that we can found on your favorite OS, executable are only part of an enums (above).

But the main changes is that, when called, the executable will call a function :)

> If there is no functions, well, fuck it: Error.
