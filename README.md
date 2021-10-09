trillium-send-file
==================

Send file connection extension for [trillium.rs](https://trillium.rs/).

# Getting Started

```toml
[dependencies]
trillium = "0.2.0"
trillium-smol = "0.2.0"
trillium-send-file = { version = "0.1.0", features = ["smol"] }
```

# Example

## src/main.rs

```rust
use trillium::Conn;
use trillium_send_file::SendFileConnExt;

fn main() {
    trillium_smol::run(|conn: Conn| async move {
        conn.send_file("/tmp/file.txt")
    });
}
```
