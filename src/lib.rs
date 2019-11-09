use chrono::prelude::*;

pub fn beats() -> String {
    let cet = Utc::now().with_timezone(&FixedOffset::east(3600));
    let beats = (cet.num_seconds_from_midnight() as f32 * 0.011_574) as usize;

    let prefix = match beats {
        x if x < 10 => "00",
        x if x < 100 => "0",
        _ => "",
    };
    format!("@{}{}.beats", prefix, beats)
}
