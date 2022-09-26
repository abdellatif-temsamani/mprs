# Mprs

_MPD client written in rust_

Usage: **mprs**.

## commands

|    argument     |        description         |
| :-------------: | :------------------------: |
|     no args     | show stats of current song |
|      play       |   play the current song    |
|      pause      |   pause the current song   |
|      stop       |   stop the current song    |
|      next       |     play the next song     |
|      prev       |  pause the previous song   |
|      help       |    shows this help menu    |
|      kill       |      kill mpd server       |
|  --host='HOST'  |     host of mpd server     |
|  --port='PORT'  |     port of mpd server     |
| --silent or -q  |       silent output        |
| --version or -v |     print the version      |
|  --help or -h   |       print the help       |

# build

- compile

```bash
rustup run nightly cargo build
```

- install

```bash
rustup run nightly cargo install
```
