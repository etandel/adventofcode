extern crate chrono;
extern crate regex;

use std::env;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::num::ParseIntError;

use chrono::{DateTime, ParseError, Timelike, TimeZone, Utc};
use regex::Regex;



#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Log {
    NewShift(u16),
    FallsAsleep,
    WakesUp,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct LogEntry {
    datetime: DateTime<Utc>,
    log: Log,
}


impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.datetime.cmp(&other.datetime))
    }
}


impl Ord for LogEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.datetime.cmp(&other.datetime)
    }
}


#[derive(Debug)]
struct LogParseErr {}

impl From<ParseIntError> for LogParseErr {
    fn from(_: ParseIntError) -> LogParseErr {
        LogParseErr{}
    }
}

impl From<ParseError> for LogParseErr {
    fn from(_: ParseError) -> LogParseErr {
        LogParseErr{}
    }
}


impl FromStr for LogEntry {
    type Err = LogParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = Regex::new(r"^\[(.+)\] (.+)$").unwrap().captures(s).unwrap();
        let datetime = Utc.datetime_from_str(&caps[1], "%Y-%m-%d %H:%M")?;
        let log_type =
            if Regex::new("^falls").unwrap().is_match(&caps[2]) {
                Log::FallsAsleep
            } else if Regex::new("^wakes").unwrap().is_match(&caps[2]) {
                Log::WakesUp
            } else {
                let id = Regex::new(r"^Guard #(\d+)").unwrap()
                    .captures(&caps[2]).unwrap()[1]
                    .parse::<u16>()?;
                Log::NewShift(id)
            };

        Ok(LogEntry {
           datetime: datetime,
           log: log_type,
        })
    }
}


enum GuardState {
    JustArrived(DateTime<Utc>),
    Awoken(DateTime<Utc>),
    Sleeping(DateTime<Utc>),
}


struct Guard {
    id: u16,
    states: Vec<GuardState>,
    minutes: Vec<u16>,
}


fn time_to_pos<T: TimeZone>(datetime: &DateTime<T>) -> usize {
    datetime.minute() as usize
//    let h = datetime.hour() as i32;
//    let m = datetime.minute() as i32;
//    let r = match h {
//        0 => 60 + m,
//        23 => m,
//        _ => 1,
//    } as usize;
//    println!("{} - {} - {}", h, m, r);
//    r
//    let r: i32 = hack_mod(h - 23, 24) * 60 + m;
 //   println!("{} - {} - {} - {}", h, m, (h - 23), r);
 //   r as usize
}


impl Guard {
    fn new(id: u16) -> Guard {
        Guard {
            id: id,
            states: Vec::new(),
            // From 23:00 to 00:59
            minutes: vec![0; 60],
        }
    }

    fn push_state(&mut self, log_entry: &LogEntry) {
        let state = match &log_entry.log {
          Log::NewShift(_) => GuardState::JustArrived(log_entry.datetime.clone()),
          Log::WakesUp => GuardState::Awoken(log_entry.datetime.clone()),
          Log::FallsAsleep => GuardState::Sleeping(log_entry.datetime.clone()),
        };
        self.states.push(state);
    }

    fn set_minutes(&mut self) {
        for (before, after) in self.states.iter().zip(self.states[1..].iter()) {
            match (before, after) {
                (GuardState::Sleeping(start), GuardState::Awoken(end)) => {
                    for i in time_to_pos(start)..time_to_pos(end) {
                        self.minutes[i] += 1;  
                    }
                }
                _ => {}
            }
        }
    }
}


//fn find_most_asleep_guard(sort_guards_by: Iterator<


fn part1() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut logs: Vec<LogEntry> = content.lines()
                                         .map(|l| LogEntry::from_str(l).unwrap())
                                         .collect::<Vec<LogEntry>>();
    logs.sort();

    let mut guards: HashMap<u16, Guard> = HashMap::new();
    let mut current_gid: Option<u16> = None;
    for log in logs.iter() {
        if let Log::NewShift(id) = log.log {
            current_gid = Some(id);
        }

        (*guards
         .entry(current_gid.unwrap())
         .or_insert(Guard::new(current_gid.unwrap())))
        .push_state(&log);
    }

    guards.values_mut().for_each(Guard::set_minutes);
    let mut guards: Vec<&Guard> = guards.values().collect();
    guards.sort_unstable_by_key(|g| g.minutes.iter().sum::<u16>());

    let most_asleep_guard = guards.iter().last().unwrap();
    let (most_asleep_minute, _) = &most_asleep_guard.minutes.iter()
        .enumerate().max_by_key(|&(_, i)| i).unwrap();

    println!("{}", (most_asleep_guard.id as usize) * (most_asleep_minute % 60));
}


fn part2() {
}


fn main() {
    match env::args().find(|arg| arg == "1") {
        Some(_) => part1(),
        None => part2(),
    };
}

