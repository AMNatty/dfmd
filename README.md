# dfmd

A small utility daemon to handle "open containing folder" activities in various
software.

🦊

## Quick installation

AUR package: `dfmd-git`

If some other handler takes precedence over this one, try copying the DBus
service file into your user directory. Also make sure no other service is
running while executing `dfmd`.

Bash et al.:

```sh
mkdir -p "${XDG_DATA_HOME:-"$HOME/.local/share"}/dbus-1/services"
cp /usr/share/dbus-1/services/sh.natty.FileManager1.service "${XDG_DATA_HOME:-"$HOME/.local/share"}/dbus-1/services/org.freedesktop.FileManager1.service"
```

Fish:

```sh
# Fallback universal variable in case XDG_DATA_HOME is not set
set -U XDG_DATA_HOME ~/.local/share
mkdir -p "$XDG_DATA_HOME/dbus-1/services"
cp /usr/share/dbus-1/services/sh.natty.FileManager1.service "$XDG_DATA_HOME/dbus-1/services/org.freedesktop.FileManager1.service"
```

## Dependencies

- DBus
- xdg-open as the default handler

## Building

```sh
cargo build --release
```

## Configuration

Use the DFMD_*_PROGRAM environment variables. `%ARGS%` is substituted with a
list of input files.

### Default handler programs

```sh
DFMD_FOLDER_PROGRAM='echo %ARGS% | xargs -n1 xdg-open'
DFMD_ITEMS_PROGRAM='echo %ARGS% | xargs -d " " -I {} sh -c '"'"'p="{}"; echo "${p%/*}"'"'"' | xargs -n1 xdg-open'
DFMD_PROPERTIES_PROGRAM='echo %ARGS% | xargs -n1 xdg-open'
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
