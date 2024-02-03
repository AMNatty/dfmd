# dfmd

A small utility daemon to handle "open containing folder" activities in various
software.

🦊

## Dependencies

- DBus
- xdg-open as the default handler

## Building

```
cargo build --release
```

## Configuration

Use the DFMD_*_PROGRAM environment variables. `%ARGS%` is substituted with a
list of input files.

### Default handler programs

```
DFMD_FOLDER_PROGRAM: echo %ARGS% | xargs -n1 xdg-open
DFMD_ITEMS_PROGRAM: echo %ARGS% | xargs -d " " -I {} sh -c 'p="{}"; echo "${p%/*}"' | xargs -n1 xdg-open
DFMD_PROPERTIES_PROGRAM: echo %ARGS% | xargs -n1 xdg-open
```

## Autostart

Create a DBus service in
`$XDG_DATA_HOME/dbus-1/services/org.freedesktop.FileManager1.service`.

(The default directory for `XDG_DATA_HOME` is `$HOME/.local/share/`)

```
[D-BUS Service]
Name=org.freedesktop.FileManager1
Exec=path/to/dfmd
```
