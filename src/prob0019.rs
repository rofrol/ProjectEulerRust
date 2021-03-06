#![crate_id = "prob0019"]
#![crate_id = "prob0019"]
#![crate_type = "rlib"]
#![crate_type = "rlib"]

pub static EXPECTED_ANSWER: &'static str = "171";

fn is_leap_year(y: uint) -> bool {
    if y % 400 == 0 { return true; }
    if y % 100 == 0 { return false; }
    if y % 4   == 0 { return true; }
    return false;
}

fn day_of_year(y: uint) -> uint {
    if is_leap_year(y) { 366 } else { 365 }
}

fn day_of_month(y: uint) -> [uint, ..12] {
    [ 31, // Jan
     if is_leap_year(y) { 29 } else { 28 }, // Feb
     31, // Mar
     30, // Apr
     31, // May
     30, // Jun
     31, // Jul
     31, // Aug
     30, // Sep
     31, // Oct
     30, // Nov
     31  // Dec
    ]
}

fn append_day(y: uint, offset: uint, result: &mut [uint, ..7]) -> uint {
    let mut day = offset;
    let dom = day_of_month(y);
    for n in dom.iter() {
        result[day] += 1;
        day = (day + *n) % 7;
    }
    return day;
}

pub fn solve() -> ~str {
    let mut result = [0, ..7];
    let mut day = 1; // Monday
    day = (day + day_of_year(1900)) % 7;
    for y in range(1901u, 2000 + 1) {
        day = append_day(y, day, &mut result);
    }
    return result[0].to_str();
}
