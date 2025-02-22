//#![allow(unused, clippy::never_loop)]
mod parsing_info;
mod run_commands;

use std::{thread::sleep, time::Duration};

use parsing_info::RyzenAdjInfo;

fn main() {
    let write_data_to_csv = false;
    let target_fast = 15_000; // mW
    let target_slow = target_fast;
    let mut controller = Controller::new(target_fast, target_slow);
    env_logger::init();
    sleep(Duration::from_secs(1));
    controller.reset_limit();
    sleep(Duration::from_secs(1));
    loop {
        controller.update(write_data_to_csv);
        //dbg!(&controller.changes);
        //dbg!(format!("fast_limit:{}\nfast_target{}", &controller.fast_limit, target_fast));
        if controller
            .changes
            .iter()
            .any(|x| matches!(x, Changes::FastLimit(_) | Changes::SlowLimit(_)))
        {
            controller.reset_limit();
            controller.changes.clear();
            //dbg!(&controller);
            //dbg!(&controller);
            //dbg!(&controller);
            //break;
        }

        // SLEEP TIME
        sleep(Duration::from_secs(1));
    }
}

#[derive(Debug)]
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
    /// push changes onto changes field
    fn update(&mut self, write_data: bool) {
        self.action = State::CheckingChanges;

        let ryzen_adj_info = match run_commands::get_info() {
            Ok(info_output) => parsing_info::parse_ryzenadj_info(info_output),
            Err(_err) => panic!("PAnicking when getting ryzendadj info"),
        };
        

        if write_data {
            ryzen_adj_info.write_csv("datas/power_data.csv");
        }

        let RyzenAdjInfo {
            time: _,
            stapm_value: _,
            ppt_limit_fast,
            ppt_value_fast,
            ppt_limit_slow,
            ppt_value_slow: _,
            ppt_value_apu: _,
        } = ryzen_adj_info;

        // VALUES
        if self.value_fast != ppt_value_fast {
            //dbg!(
            //    "Fast limit changed from {} to {}",
            //    self.fast_limit, ppt_limit_fast
            //);
            self.changes.push(Changes::FastValue(ppt_value_fast));
            
        }

        // LIMITS
        if self.fast_limit != ppt_limit_fast as u32 {
            //println!(
            //    "Fast limit changed from {} to {}",
            //    self.fast_limit, ppt_limit_fast
            //);
            self.changes.push(Changes::FastLimit(ppt_limit_fast as u32));
            self.fast_limit = ppt_limit_fast as u32;
        }
        if self.slow_limit != ppt_limit_slow as u32 {
            //println!(
            //    "Slow limit changed from {} to {}",
            //    self.slow_limit, ppt_limit_slow
            //);
            self.changes.push(Changes::SlowLimit(ppt_limit_slow as u32));
            self.slow_limit = ppt_limit_slow as u32;
        }
    }
    fn new(fast_target: u32, slow_target: u32) -> Self {
        let ryzen_adj_info = match run_commands::get_info() {
            Ok(info_output) => parsing_info::parse_ryzenadj_info(info_output),
            Err(_err) => panic!("PAnicking when getting ryzendadj info"),
        };

        let RyzenAdjInfo {
            time: _,
            stapm_value: _,
            ppt_limit_fast,
            ppt_value_fast,
            ppt_limit_slow,
            ppt_value_slow: _,
            ppt_value_apu: _,
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

    fn reset_limit(&mut self) {
        self.action = State::ResettingRyzenAdjParams;
        run_commands::reset_fast_limit(self.fast_target);
        run_commands::reset_slow_limit(self.slow_target);
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
enum Changes {
    FastLimit(u32), // they carry the new value of the changed param
    FastValue(f32),
    SlowLimit(u32),
}
#[derive(Debug)]
enum State {
    CheckingChanges,
    ResettingRyzenAdjParams,
    Nothing,
}
