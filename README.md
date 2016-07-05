# emit_ansi_term [![Join the chat at https://gitter.im/serilog/serilog](https://img.shields.io/gitter/room/emit/emit-rs.svg)](https://gitter.im/emit-rs/emit) [![Crates.io](https://img.shields.io/crates/v/emit_ansi_term.svg)](https://crates.io/crates/emit_ansi_term)

Colored (ANSI) terminal collector for the [emit](https://crates.io/crates/emit) structured logger.

```rust
let _flush = PipelineBuilder::new()
    .write_to(AnsiTerminalCollector::new())
    .init();

info!("Hello, nice colored output!");
```
