#[macro_use]
extern crate emit;
extern crate log;
extern crate ansi_term;
extern crate chrono;

use std::io;
use std::error::Error;
use ansi_term::{Style,Colour,ANSIString};
use emit::events::Event;
use emit::collectors::AcceptEvents;
use emit::templates::repl::MessageTemplateRepl;
use std::io::Write;
use chrono::Local;

pub struct AnsiTerminalCollector {}

impl AnsiTerminalCollector {
    pub fn new() -> Self {
        AnsiTerminalCollector{}
    }
}

static LEVEL_COLORS: [(Colour,Colour); 6] = [
    (Colour::White,  Colour::Red),
    (Colour::White,  Colour::Red),
    (Colour::Yellow, Colour::Black),
    (Colour::White,  Colour::Black),
    (Colour::Cyan,   Colour::Black),
    (Colour::Cyan,   Colour::Black)
];

static LEVEL_NAMES: [&'static str; 6] = [
    "OFF",
    "ERR",
    "WRN",
    "INF",
    "DBG",
    "TRA"
];

fn to_colored_level(level: log::LogLevel) -> ANSIString<'static> {
    let n = LEVEL_NAMES[level as usize];
    let c = LEVEL_COLORS[level as usize];
    Style::new().fg(c.0).bold().on(c.1).paint(n)
}

impl AcceptEvents for AnsiTerminalCollector {
    fn accept_events(&self, events: &[Event<'static>])-> Result<(), Box<Error>> {
        let bold = Style::new().bold();
        for event in events {
            let out = io::stdout();
            let mut to = &mut out.lock();
            let repl = MessageTemplateRepl::new(event.message_template().text());
            let content = repl.replace(event.properties());
            try!(writeln!(to, "[{} {}] {}",
                event.timestamp().with_timezone(&Local).format("%T%.3f"),
                 to_colored_level(event.level()),
                 bold.paint(content)));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use emit::PipelineBuilder;
    use super::AnsiTerminalCollector;

    #[test]
    fn it_works() {        
        let _flush = PipelineBuilder::new()
            .write_to(AnsiTerminalCollector::new())
            .init();

        eminfo!("Hello, {}!", name: "Alice");
        emerror!("Goodbye, {}!", number: 42);
    }
}
