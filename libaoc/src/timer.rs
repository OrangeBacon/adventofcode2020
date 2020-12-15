use crate::FloatTime;
use std::cmp::max;
use std::fmt;
use std::time::Instant;

pub struct Timer {
    laps: Vec<(&'static str, f64)>,
    latest: Instant,
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        let inst = Instant::now();
        Timer {
            laps: vec![],
            latest: inst,
            start: inst,
        }
    }

    pub fn lap(&mut self, reason: &'static str) {
        let duration = self.latest.elapsed().as_secs_f64();
        self.laps.push((reason, duration));
        self.latest = Instant::now();
    }

    pub fn stop(&mut self) {
        let duration = self.start.elapsed().as_secs_f64();
        self.laps.push(("Total", duration));
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "Timing:")?;
        if self.laps.is_empty() {
            return writeln!(f, "  No timing infomation recieved");
        }

        let width = self
            .laps
            .iter()
            .fold(0, |acc, (reason, _)| max(acc, reason.len()))
            + 1;

        for (reason, duration) in &self.laps {
            writeln!(
                f,
                "  {:<width$} {}",
                format!("{}:", reason),
                FloatTime::from(*duration),
                width = width
            )?;
        }
        Ok(())
    }
}
