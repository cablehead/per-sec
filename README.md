# per-sec

A Unix-style tool that buffers stdin and executes a command with the buffered input every second, following the philosophy of djb's tool chain design.

## Installation

```bash
# Clone and install from GitHub
git clone https://github.com/cablehead/per-sec.git
cd per-sec
cargo install --path .
```

## Usage

```
per-sec <command> [args...]
```

The program reads from stdin continuously, buffers the input, and every second:
1. Spawns the specified command with any provided arguments
2. Writes the buffered input to the command's stdin
3. Clears the buffer for the next second

If no input is received during a one-second window, the command is still executed with an empty stdin.

## Examples

Count Bluesky firehose events per second:
```bash
websocat "wss://jetstream1.us-east.bsky.network/subscribe" | per-sec wc -l
```

Process batches of data every second:
```bash
cat stream.jsonl | per-sec jq -c 'group_by(.type)'
```

Monitor HTTP requests per second:
```bash
tail -f access.log | per-sec grep 'GET' | per-sec wc -l
```

## Dependencies

- Built with Rust
- Uses [timeout-readwrite](https://github.com/jcreekmore/timeout-readwrite-rs) for IO timeouts without an async runtime

## License

MIT
