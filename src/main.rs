//! # timecard
//!
//! Script to calculate total hours on a timecard.

use std::io;
use std::io::Read;
use std::process;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

// define our Regex patterns as statics to optimize peformance

lazy_static! {
    pub static ref DATE_RE: Regex = Regex::new(r"^(\d{1,2})\s*/\s*(\d{1,2})$").unwrap();
}

lazy_static! {
    pub static ref TIME_RANGE_RE: Regex =
        Regex::new(r"^(\d{1,2}):(\d{1,2})\s*-\s*(\d{1,2}):(\d{1,2})$").unwrap();
}

#[derive(Debug)]
struct Date {
    month: u32,
    day: u32,
}
impl Date {
    fn new(month: u32, day: u32) -> Result<Date, String> {
        match (month, day) {
            (1..=12, 1..=31) => Ok(Date {month: month, day: day}),
            (_, _) => Err(format!("invalid date \"{}/{}\"", month, day))
        }
    }
    fn to_s(&self) -> String {
        format!("{}/{}", self.month, self.day)
    }
}

#[derive(Debug)]
struct Time {
    hours: u32, // use enum, can be 12 or 24
    minutes: u32,
}
impl Time {
    fn new(hours: u32, minutes: u32) -> Result<Time, String> {
        match (hours, minutes) {
            (1..=12, 0..=59) => Ok(Time {hours: hours, minutes: minutes}),
            (_, _) => Err(format!("invalid time \"{}:{}\"", hours, minutes))
        }
    }
    /// Time of day in minutes, 12 hour clock,
    /// 0 at noon & midnight
    /// 12:00 ->  0 * 60 + 0
    /// 12:01 ->  0 * 60 + 1
    ///  1:05 ->  1 * 60 + 5
    fn in_minutes(&self) -> u32 {
        if self.hours == 12 {
            self.minutes
        } else {
            self.hours * 60 + self.minutes
        }
    }
    // fn to_s(&self) -> String {
    //     format!("{}:{}", self.hours, self.minutes)
    // }
}

// TimeRange e.g. 1:15-2:45
#[derive(Debug)]
struct TimeRange {
    start: Time,
    end: Time,
}
impl TimeRange {
    fn new(start_hr: u32, start_min: u32, end_hr: u32, end_min: u32) -> Result<TimeRange, String> {
        let start = Time::new(start_hr, start_min);
        let end = Time::new(end_hr, end_min);
        match (start, end) {
            (Ok(start), Ok(end)) => Ok(TimeRange {
                start: start,
                end: end,
            }),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
    fn minutes(&self) -> u32 {
        // if end minutes greater than start minutes,
        // simply take the difference,
        // e.g. 1:15-2:45 -> 90 min
        let start_min = &self.start.in_minutes();
        let end_min = &self.end.in_minutes();
        if start_min < end_min {
            end_min - start_min
        } else if start_min > end_min {
            // if end minutes less that start minutes, we've
            // rolled over the 12 hour clock,
            // e.g. 12:45-1:15 -> 30 min
            let minutes_before_rollover = 12 * 60 - start_min;
            end_min + minutes_before_rollover
        } else {
            // consider cases where start and end equal to
            // be zero minutes, not 24 hours. =)
            0
        }
    }
    // fn to_s(&self) -> String {
    //     format!("{}-{}", self.start.to_s(), self.end.to_s())
    // }
}

enum Entry {
    DateEntry(Date),
    TimeRangeEntry(TimeRange),
}

fn main() {
    let input: String = get_input_from_user();

    // mock input string
    // let input = "1/3\n\n1:23-1:27\n12/4\n3:45-4:45\n12/15\n12:45-1:15";

    let entries = parse_user_input(input);
    if entries.is_empty() {
        println!("Input empty! ¯\\_(ツ)_/¯");
        print_div();
        return;
    }
    process_entries(entries);
}

fn get_input_from_user() -> String {
    let mut input = String::new();
    print_div();
    println!("Paste in timecard data, then <Enter>, then <CTRL-D>");
    print_div();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input string");
    input
}

fn parse_user_input(input: String) -> Vec<Entry> {
    let mut entries = Vec::new();
    let lines = input.lines();
    for (i, line) in lines.enumerate() {
        let line = line.trim();

        // try to match date w/ regex
        match DATE_RE.captures(line) {
            Some(m) => {
                // regex date match
                let date = Date::new(m[1].parse().unwrap(), m[2].parse().unwrap());
                match date {
                    Ok(date) => {
                        entries.push(Entry::DateEntry(date));
                        continue
                    }
                    Err(e) => handle_parse_error(i + 1, line, e),
                }
            }
            None => (), // not date, carry on
        }

        // try to match time range w/ regex
        match TIME_RANGE_RE.captures(line) {
            Some(m) => {
                // regex time range match
                let time_range = TimeRange::new(
                    m[1].parse().unwrap(),
                    m[2].parse().unwrap(),
                    m[3].parse().unwrap(),
                    m[4].parse().unwrap(),
                );
                match time_range {
                    Ok(time_range) => {
                        entries.push(Entry::TimeRangeEntry(time_range));
                        continue
                    }
                    Err(e) => handle_parse_error(i + 1, line, e),
                };
            }
            None => (), // not time range, carry on
        }

        match line {
            "" => (), // blank line, ignore
            _ => handle_parse_error(i +1, line, "Doesn't look like date or time range".to_string())
        }
    }

    entries
}

fn process_entries(entries: Vec<Entry>) {
    let mut day_minutes = 0;
    let mut tot_minutes = 0;

    print_div();

    for entry in entries {
        match entry {
            Entry::DateEntry(date) => {
                if day_minutes > 0 {
                    println!("{}", format_minutes(day_minutes));
                    tot_minutes += day_minutes;
                    day_minutes = 0;
                }
                print!("{: >5}: ", date.to_s());
            }
            Entry::TimeRangeEntry(time_range) => {
                // println!("{}", time_range.to_s())
                day_minutes += time_range.minutes();
            }
        }
    }

    // Handle final day's hours
    if day_minutes > 0 {
        println!("{}", format_minutes(day_minutes));
        tot_minutes += day_minutes;
    }

    // Total
    print_div();
    println!("Total: {}", format_minutes(tot_minutes));
    print_div();
}

fn format_minutes(minutes: u32) -> String {
    format!("{:.2} hrs", (minutes as f32) / 60.0)
}

fn print_div() {
    println!("{}", "-------------------------");
}

fn handle_parse_error(line_no: usize, line: &str, msg: String) {
    print_div();
    println!("Line {}: \"{}\": {}", line_no, line, msg);
    process::exit(1);
}
