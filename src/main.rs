#![allow(unused, clippy::never_loop)]
mod parsing_info;
mod run_commands;

use std::{thread::sleep, time::Duration};

use log::{log, Level};
use parsing_info::RyzenAdjInfo;

fn main() {
    let target_fast = 20_000;
    let target_slow = 20_000;
    let mut controller = Controller::new();

    loop {
        controller.action = State::Reading;
        controller.populate_info();
        let mut changes_iter = controller.changes.iter();
        for change in changes_iter {
            match change {
                Changes::FastChanged(fast_limit) => {
                    log!(
                        Level::Warn,
                        "FastLimit has changed from {} to {}",
                        controller.fast.get_current_fast(),
                        fast_limit
                    );
                    controller.action = State::Writing;
                    controller.write_value(ToWriteValues::FastLimit(*fast_limit));
                }
                Changes::SlowChanged(slow_limit) => {
                    log!(
                        Level::Warn,
                        "SlowLimit has changed from {} to {}",
                        controller.fast.get_current_slow(),
                        slow_limit
                    );
                    controller.write_value(ToWriteValues::SlowLimit(*slow_limit))
                }
            }
        }
        // DEBUG LINE
        break;
        sleep(Duration::from_secs(1));
    }
}

enum RyzenAdjParam {
    Fast { target: u32, current: u32 },
    Slow { target: u32, current: u32 },
}
impl RyzenAdjParam {
    fn get_current_fast(&self) -> u32 {
        match self {
            RyzenAdjParam::Fast { current, .. } => *current,
            _ => panic!(),
        }
    }
    fn get_current_slow(&self) -> u32 {
        match self {
            RyzenAdjParam::Slow { current, .. } => *current,
            _ => panic!(),
        }
    }
}
struct Controller {
    action: State,
    fast_target: u32,
    fast_limit: u32,
    slow_target: u32,
    slow_limit: u32,
    changes: Vec<Changes>,
}
impl Controller {
    fn new(fast_target: u32, slow_target: u32) -> Self {
        let ryzen_adj_info = match run_commands::get_info() {
            Ok(info_output) => parsing_info::parse_ryzenadj_info(info_output),
            Err(err) => panic!("PAnicking when getting ryzendadj info"),
        };

        match ryzen_adj_info {
            RyzenAdjInfo {
                stapm_value,
                ppt_limit_fast,
                ppt_value_fast,
                ppt_limit_slow,
                ppt_value_slow,
                ppt_value_apu,
            } => Self {
                action: State::Nothing,
                fast_target,
                fast_limit: ppt_limit_fast as u32,
                slow_target,
                slow_limit: ppt_limit_slow as u32,
                changes: vec![],
            },
        }
    }
    fn write_value(&self, to_write: ToWriteValues) {}
}
enum Changes {
    FastChanged(u32),
    SlowChanged(u32),
}
enum State {
    Reading,
    Writing,
    Nothing,
}

enum ToWriteValues {
    FastLimit(u32),
    SlowLimit(u32),
}
