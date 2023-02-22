use crate::models::AppState;

use chrono::{NaiveDateTime, Utc};

pub fn format(state: &AppState, received_at: &NaiveDateTime) -> [u8; 48] {
    let li = 0b00;
    let vn = 4;
    let mode = 4;

    let stratum = 16;
    let poll = 10;
    let precision = 0;

    let root_delay = 0u32.to_be_bytes();
    let root_dispersion = 0u32.to_be_bytes();
    let reference_id = "USER".as_bytes();

    let reference_timestamp = state.updated_at.timestamp().to_be_bytes();
    let origin_timestamp = Utc::now().naive_utc().timestamp().to_be_bytes();
    let receive_timestamp = received_at.timestamp().to_be_bytes();
    let transmit_timestamp = state.time.timestamp().to_be_bytes();

    return [
        li & vn & mode,
        stratum,
        poll,
        precision,
        root_delay[0],
        root_delay[1],
        root_delay[2],
        root_delay[3],
        root_dispersion[0],
        root_dispersion[1],
        root_dispersion[2],
        root_dispersion[3],
        reference_id[0],
        reference_id[1],
        reference_id[2],
        reference_id[3],
        reference_timestamp[0],
        reference_timestamp[1],
        reference_timestamp[2],
        reference_timestamp[3],
        reference_timestamp[4],
        reference_timestamp[5],
        reference_timestamp[6],
        reference_timestamp[7],
        origin_timestamp[0],
        origin_timestamp[1],
        origin_timestamp[2],
        origin_timestamp[3],
        origin_timestamp[4],
        origin_timestamp[5],
        origin_timestamp[6],
        origin_timestamp[7],
        receive_timestamp[0],
        receive_timestamp[1],
        receive_timestamp[2],
        receive_timestamp[3],
        receive_timestamp[4],
        receive_timestamp[5],
        receive_timestamp[6],
        receive_timestamp[7],
        transmit_timestamp[0],
        transmit_timestamp[1],
        transmit_timestamp[2],
        transmit_timestamp[3],
        transmit_timestamp[4],
        transmit_timestamp[5],
        transmit_timestamp[6],
        transmit_timestamp[7],
    ];
}
