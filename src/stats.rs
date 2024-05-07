use std::ops::Add;

pub struct Stats {
    pub comps: usize,
    pub swaps: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self { comps: 0, swaps: 0 }
    }

    pub(crate) fn comp(&mut self) {
        self.comps += 1;
    }

    pub(crate) fn swap(&mut self) {
        self.swaps += 1;
    }
}

impl Add for Stats {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            comps: self.comps + other.comps,
            swaps: self.swaps + other.swaps,
        }
    }
}

impl std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "comps: {}, swaps: {}", self.comps, self.swaps)
    }
}

use std::io::Write;
pub fn setup_logger(size: usize) {
    let mut log = env_logger::builder();
    log.format(|buf, record| writeln!(buf, "{}", record.args()));
    log.filter_level(if size < 40 { log::LevelFilter::Trace } else { log::LevelFilter::Debug });
    log.init();
}