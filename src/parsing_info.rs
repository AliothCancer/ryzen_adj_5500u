use std::{
    fs::{File, OpenOptions},
    io::{stdin, stdout, Write},
    os::unix::process,
    path::Path,
};

use chrono::Local;
use log::{log, warn};
use serde::{self, Deserialize, Serialize};

pub fn parse_ryzenadj_info(cmd_output: String) -> RyzenAdjInfo {
    /// Parse the string output of "sudo $HOME/.local/bin/ryzenadj -i"
    let params = [
        "STAPM VALUE",
        "PPT LIMIT FAST",
        "PPT VALUE FAST",
        "PPT LIMIT SLOW",
        "PPT VALUE SLOW",
        "PPT VALUE APU",
    ];
    let k = cmd_output
        .lines()
        .filter(|x| params.iter().any(|param| x.contains(param)))
        .collect::<Vec<_>>();
    //dbg!(&k);
    match k.as_slice() {
        &[stapm_value_raw, ppt_limit_fast_raw, ppt_value_fast_raw, ppt_limit_slow_raw, ppt_value_slow_raw, ppt_value_apu_raw] =>
        {
            let extract_and_parse_info = |info_raw: &str| {
                info_raw
                    .split('|')
                    .nth(2)
                    .take()
                    .unwrap()
                    .trim()
                    .parse::<f32>()
                    .unwrap()
            };

            let stapm_value = extract_and_parse_info(stapm_value_raw);
            let ppt_limit_fast = extract_and_parse_info(ppt_limit_fast_raw);
            let ppt_value_fast = extract_and_parse_info(ppt_value_fast_raw);

            let ppt_limit_slow = extract_and_parse_info(ppt_limit_slow_raw);
            let ppt_value_slow = extract_and_parse_info(ppt_value_slow_raw);

            let ppt_value_apu = extract_and_parse_info(ppt_value_apu_raw);

            // Get the time and format it
            let time_now = Local::now();
            let time = time_now.format("%d/%m/%Y %H:%M:%S").to_string();

            RyzenAdjInfo {
                time,
                stapm_value,
                ppt_limit_fast,
                ppt_value_fast,
                ppt_limit_slow,
                ppt_value_slow,
                ppt_value_apu,
            }
            //dbg!(&infos);
        }
        _ => panic!(),
    }
}

#[derive(Serialize, Debug)]
pub struct RyzenAdjInfo {
    pub time: String,
    pub stapm_value: f32,
    pub ppt_limit_fast: f32,
    pub ppt_value_fast: f32,
    pub ppt_limit_slow: f32,
    pub ppt_value_slow: f32,
    pub ppt_value_apu: f32,
}

impl RyzenAdjInfo {
    pub fn write_csv(&self, file_path: impl AsRef<Path>) {
        match std::fs::File::open(&file_path) {
            Ok(_) => {
                let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
                let mut wtr = csv::WriterBuilder::new()
                    .has_headers(false)
                    .from_writer(file);

                wtr.serialize(self)
                    .expect("writing serialized RyzenAdjInfo");
            }
            Err(err) => {
                warn!("{}", err);
                print!("File has not been found, Do you want to create it?(Y/n) ");
                stdout().flush().expect("Err in flushing..");
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Err in reading input");
                match input.to_lowercase().trim() {
                    "n" => (),
                    _ => {
                        create_file(&file_path);
                        let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
                        let mut wtr = csv::WriterBuilder::new()
                            .has_headers(true)
                            .from_writer(file);
                        wtr.serialize(self)
                            .expect("writing serialized RyzenAdjInfo");
                    }
                }
            }
        }
    }
}

fn create_file(file_path: impl AsRef<Path>) {
    File::create(file_path).expect("Err trying to create the file");
}
