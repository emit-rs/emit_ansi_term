# emit_ansi_term

Colored (ANSI) terminal collector for the [emit](https://crates.io/crates/emit) structured logger.

```rust
let _flush = PipelineBuilder::new()
    .write_to(AnsiTerminalCollector::new())
    .init();

eminfo!("Hello, nice colored output!");
```
