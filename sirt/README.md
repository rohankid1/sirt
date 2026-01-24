# Sirt
An analyzer for libsirt.
Can be used as a TUI (still being developed) or a GUI (mostly finished) app.

# CLI Usage
## From File
* GUI:
```
sirt file /path/to/file gui
```

* TUI:
```
sirt file /path/to/file tui
```

## From Input
* GUI:
```
sirt from "Example { a: int(10), b: text(\"this is an example\") }" --using gui
```

* TUI:
```
sirt from "Example { a: int(10), b: text(\"this is an example\") }" --using tui
```
