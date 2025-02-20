use serde::{self, Deserialize, Serialize};

pub fn parse_ryzenadj_info(cmd_output: String)-> RyzenAdjInfo {
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

            
            RyzenAdjInfo {
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
    pub stapm_value: f32,
    pub ppt_limit_fast: f32,
    pub ppt_value_fast: f32,
    pub ppt_limit_slow: f32,
    pub ppt_value_slow: f32,
    pub ppt_value_apu: f32,
}

impl RyzenAdjInfo {
    fn write_csv(self) {
        let mut writer = csv::Writer::from_path("data.csv").expect("Writing data");
        writer
            .serialize(self)
            .expect("writing serialized RyzenAdjInfo");
        writer.flush().expect("flushing");
    }
}
