use crate::models::Time;

pub fn format(time: Time) -> [u8; 1] {
    let li = 0b00;
    let vn = 4;
    let mode = 4;
    return [
        li & vn & mode,
    ];
}
