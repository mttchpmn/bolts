use std::mem::zeroed;
#[cfg(windows)]
use winapi;
#[cfg(not(windows))]
use libc;

use chrono::{Datelike, DateTime, Local, Timelike, TimeZone};
use clap::{App, Arg};

fn main() {
    let app = App::new("clock")
        .version("0.1")
        .about("Gets and sets the time")
        .arg(Arg::new("action")
            .takes_value(true)
            .possible_values(&["get", "set"])
            .default_value("get")
        )
        .arg(Arg::new("std")
            .short('s')
            .long("standard")
            .takes_value(true)
            .possible_values(&["rfc2822", "rfc3339", "timestamp"])
            .default_value("rfc3339")
        )
        .arg(Arg::new("datetime")
            .help("When <action> is 'set', apply <datetime>. Otherwise, ignore")
        );

    let matches = app.get_matches();

    let action = matches.value_of("action").unwrap();
    let std = matches.value_of("std").unwrap();

    if action == "set" {
        println!("SET");
        let t_ = matches.value_of("datetime").unwrap();

        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            _ => unimplemented!(),
        };

        let err_msg = format!(
            "Unable to parse {} according to {}",
            t_, std
        );
        let t = parser(t_).expect(&err_msg);

        Clock::set(t)
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!()
    }
}

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::Weekday;
        use winapi::shared::minwindef::WORD;
        use winapi::um::minwinbase::SYSTEMTIME;
        use winapi::um::sysinfoapi::SetSystemTime;

        let t = t.with_timezone(&Local);

        let mut systime: SYSTEMTIME = unsafe { zeroed() };

        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };

        let mut ns = t.nanosecond();
        let is_leap_second = ns > 1_000_000_000;

        if is_leap_second {
            ns -= 1_000_000_000;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = t.second() as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            SetSystemTime(systime_ptr);
        }
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::{timeval, time_t, suseconds_t};
        use libc::{settimeofday, timezone};

        let t = t.with_timezone(&Local);
        let mut u: timeval = unsafe { zeroed() };

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofdat(&u as *const timeval, mock_tz);
        }
    }
}
