#![allow(unused, clippy::never_loop)]
mod parsing_info;
mod run_commands;

use std::{thread::sleep, time::Duration};

use log::{log, Level};
use parsing_info::RyzenAdjInfo;

fn main() {
    let target_fast = 20_000;
    let target_slow = 20_000;
    let mut controller = Controller::new(target_fast, target_slow);

    loop {
        controller.update();
        dbg!(&controller.changes);
        if controller
            .changes
            .iter()
            .any(|x| matches!(x, Changes::Fast(_) | Changes::Slow(_)))
        {
            controller.reset_limit();
        }
        //if controller.changes.iter().any(|x| matches!(x, Changes::FastValue(_))) {
        //    controller.reset_limit();
        //}

        // DEBUG LINE
        //break;

        // SLEEP TIME
        sleep(Duration::from_secs(1));
    }
}

struct Controller {
    action: State,
    fast_target: u32,
    fast_limit: u32,
    value_fast: f32,
    slow_target: u32,
    slow_limit: u32,
    changes: Vec<Changes>,
}
impl Controller {
    fn update(&mut self) {
        /// push changes onto changes field
        let ryzen_adj_info = match run_commands::get_info() {
            Ok(info_output) => parsing_info::parse_ryzenadj_info(info_output),
            Err(err) => panic!("PAnicking when getting ryzendadj info"),
        };
        self.changes.clear();
        let RyzenAdjInfo {
            stapm_value,
            ppt_limit_fast,
            ppt_value_fast,
            ppt_limit_slow,
            ppt_value_slow,
            ppt_value_apu,
        } = ryzen_adj_info;

        // VALUES
        if self.value_fast != ppt_value_fast {
            //log!(Level::Warn,"Fast limit changed from {} to {}", self.fast_limit, ppt_limit_fast);
            self.changes.push(Changes::FastValue(ppt_value_fast));
        }

        // LIMITS
        if self.fast_limit != ppt_limit_fast as u32 {
            log!(
                Level::Warn,
                "Fast limit changed from {} to {}",
                self.fast_limit,
                ppt_limit_fast
            );
            self.changes.push(Changes::Fast(ppt_limit_fast as u32));
        }
        if self.slow_limit != ppt_limit_slow as u32 {
            log!(
                Level::Warn,
                "Slow limit changed from {} to {}",
                self.slow_limit,
                ppt_limit_slow
            );
            self.changes.push(Changes::Slow(ppt_limit_slow as u32));
        }
    }
    fn new(fast_target: u32, slow_target: u32) -> Self {
        let ryzen_adj_info = match run_commands::get_info() {
            Ok(info_output) => parsing_info::parse_ryzenadj_info(info_output),
            Err(err) => panic!("PAnicking when getting ryzendadj info"),
        };

        let RyzenAdjInfo {
            stapm_value,
            ppt_limit_fast,
            ppt_value_fast,
            ppt_limit_slow,
            ppt_value_slow,
            ppt_value_apu,
        } = ryzen_adj_info;
        Self {
            action: State::Nothing,
            fast_target,
            fast_limit: ppt_limit_fast as u32,
            value_fast: ppt_value_fast,
            slow_target,
            slow_limit: ppt_limit_slow as u32,
            changes: vec![],
        }
    }

    fn reset_limit(&self) {
        run_commands::reset_fast_limit(self.fast_target);
        run_commands::reset_slow_limit(self.slow_target);
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
enum Changes {
    Fast(u32),
    FastValue(f32),
    Slow(u32),
}
enum State {
    Reading,
    Writing,
    Nothing,
}
