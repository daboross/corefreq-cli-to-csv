use std::io::prelude::*;

use itertools::Itertools;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TopLevel {
    cpu: Vec<CpuCore>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CpuCore {
    clock: CpuCoreClock,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CpuCoreClock {
    hz: u32,
}

fn main() -> Result<(), failure::Error> {
    let input = std::io::stdin();
    let mut input = input.lock();
    let output = std::io::stdout();
    let mut output = output.lock();
    let mut line = String::new();
    while input.read_line(&mut line)? > 0 {
        let parsed: TopLevel = serde_json::from_str(&line)?;
        write!(
            output,
            "{}\n",
            parsed.cpu.iter().map(|core| core.clock.hz).format(",")
        )?;
        line.clear();
    }
    Ok(())
}
