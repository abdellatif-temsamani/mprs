# mprs

A **lightweight MPD client written in Rust**, designed for simplicity,
performance, and open-source collaboration. `mprs` provides a command-line
interface to control your MPD (Music Player Daemon) server efficiently.

## Features

- **High Performance:** Built with Rust for speed and reliability.
- **Simple Command Line Interface:** Intuitive commands to manage your music
  playback.
- **Customizable Connection:** Supports connecting to MPD servers on any host
  and port.

## Installation

### Prerequisites

1. **Rust (nightly):** Install Rust using [rustup](https://rustup.rs/). Ensure
   you are using the stable version.
2. **MPD:** Install and configure MPD on your system.

### Build and Install

To build and install `mprs`, follow these steps:

```bash
# Build the project
rustup run cargo build --release

# Install the binary
rustup run cargo install --path .
```

## Usage

The `mprs` command-line tool provides subcommands for interacting with MPD.

### General Syntax

```bash
mprs [OPTIONS] <COMMAND>
```

### Options

- `--host <HOST>`: Specify the MPD server host (default: `127.0.0.1`).
- `--port <PORT>`: Specify the MPD server port (default: `6600`).
- `-h, --help`: Display help information.

### Commands

| Command  | Description                                     |
| -------- | ----------------------------------------------- |
| `status` | Display MPD's current status and playback info. |
| `play`   | Play the currently queued song.                 |
| `pause`  | Pause playback.                                 |
| `next`   | Skip to the next song in the queue.             |
| `prev`   | Go back to the previous song in the queue.      |
| `stop`   | Stop playback.                                  |
| `kill`   | Kill the MPD process.                           |
| `list`   | List files in the MPD `music_directory`.        |
| `add`    | Add a song to the queue using its path.         |
| `queued` | Show the current queue.                         |
| `clear`  | Clear all songs from the queue.                 |
| `help`   | Show help for a specific subcommand.            |

### Example Commands

1. **Play a song:**
   ```bash
   mprs play
   ```

2. **Pause playback:**
   ```bash
   mprs pause
   ```

3. **List files in the music directory:**
   ```bash
   mprs list
   ```

4. **Add a song to the queue:**
   ```bash
   mprs add <path-to-song>
   ```

5. **Clear the queue:**
   ```bash
   mprs clear
   ```

## Contributing

Contributions are welcome under the terms of the **GPL-3.0 License**. Here's how
you can contribute:

1. **Fork this repository.**
2. **Create a branch** for your feature or bugfix.
3. **Write clear and concise commit messages.**
4. **Submit a pull request** with a description of your changes.

Please ensure your code adheres to
[Rust's style guidelines](https://doc.rust-lang.org/book/).

## License

`mprs` is distributed under the terms of the **GNU General Public License
v3.0**. See the [LICENSE](./LICENSE) file for details.

### Get Started with `mprs`

Take control of your MPD music server with `mprs`—an open-source, Rust-powered
client designed for simplicity and speed. 🎶
