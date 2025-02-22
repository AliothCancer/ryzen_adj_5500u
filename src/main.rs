#![allow(unused, clippy::never_loop)]
mod parsing_info;
mod run_commands;

use std::{thread::sleep, time::Duration};

use log::{log, warn, Level};
use parsing_info::RyzenAdjInfo;

fn main() {
    let write_data_to_csv = true;
    let target_fast = 20_000;
    let target_slow = 20_000;
    let mut controller = Controller::new(target_fast, target_slow);
    env_logger::init();
    loop {
        controller.update(write_data_to_csv);
        dbg!(&controller.changes);
        if controller
            .changes
            .iter()
            .any(|x| matches!(x, Changes::FastLimit(_) | Changes::SlowLimit(_)))
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
    fn update(&mut self, write_data: bool) {
        /// push changes onto changes field
        let ryzen_adj_info = match run_commands::get_info() {
            Ok(info_output) => parsing_info::parse_ryzenadj_info(info_output),
            Err(err) => panic!("PAnicking when getting ryzendadj info"),
        };
        self.changes.clear();

        if write_data{
            ryzen_adj_info.write_csv("datas/power_data.csv");
        }

        let RyzenAdjInfo {
            time,
            stapm_value,
            ppt_limit_fast,
            ppt_value_fast,
            ppt_limit_slow,
            ppt_value_slow,
            ppt_value_apu,
        } = ryzen_adj_info;

        // VALUES
        if self.value_fast != ppt_value_fast {
            warn!("Fast limit changed from {} to {}", self.fast_limit, ppt_limit_fast);
            self.changes.push(Changes::FastValue(ppt_value_fast));
        }

        // LIMITS
        if self.fast_limit != ppt_limit_fast as u32 {
            warn!(
                "Fast limit changed from {} to {}",
                self.fast_limit,
                ppt_limit_fast
            );
            self.changes.push(Changes::FastLimit(ppt_limit_fast as u32));
        }
        if self.slow_limit != ppt_limit_slow as u32 {
            warn!(
                "Slow limit changed from {} to {}",
                self.slow_limit,
                ppt_limit_slow
            );
            self.changes.push(Changes::SlowLimit(ppt_limit_slow as u32));
        }
    }
    fn new(fast_target: u32, slow_target: u32) -> Self {
        let ryzen_adj_info = match run_commands::get_info() {
            Ok(info_output) => parsing_info::parse_ryzenadj_info(info_output),
            Err(err) => panic!("PAnicking when getting ryzendadj info"),
        };

        let RyzenAdjInfo {
            time,
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
    FastLimit(u32),
    FastValue(f32),
    SlowLimit(u32),
}
enum State {
    Reading,
    Writing,
    Nothing,
}
