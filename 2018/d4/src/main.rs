extern crate chrono;
extern crate regex;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

use chrono::{DateTime, ParseError, TimeZone, Timelike, Utc};
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
        LogParseErr {}
    }
}

impl From<ParseError> for LogParseErr {
    fn from(_: ParseError) -> LogParseErr {
        LogParseErr {}
    }
}

impl FromStr for LogEntry {
    type Err = LogParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = Regex::new(r"^\[(.+)\] (.+)$").unwrap().captures(s).unwrap();
        let datetime = Utc.datetime_from_str(&caps[1], "%Y-%m-%d %H:%M")?;
        let log_type = if caps[2].starts_with("falls") {
            Log::FallsAsleep
        } else if caps[2].starts_with("wakes") {
            Log::WakesUp
        } else {
            let id = Regex::new(r"^Guard #(\d+)")
                .unwrap()
                .captures(&caps[2])
                .unwrap()[1]
                .parse::<u16>()?;
            Log::NewShift(id)
        };

        Ok(LogEntry {
            datetime,
            log: log_type,
        })
    }
}

#[derive(Clone)]
enum GuardState {
    JustArrived(DateTime<Utc>),
    Awoken(DateTime<Utc>),
    Sleeping(DateTime<Utc>),
}

#[derive(Clone)]
struct Guard {
    id: u16,
    states: Vec<GuardState>,
    minutes: Vec<u16>,
}

fn time_to_pos<T: TimeZone>(datetime: &DateTime<T>) -> usize {
    datetime.minute() as usize
}

impl Guard {
    fn new(id: u16) -> Guard {
        Guard {
            id,
            states: Vec::new(),
            // From 23:00 to 00:59
            minutes: vec![0; 60],
        }
    }

    fn push_state(&mut self, log_entry: &LogEntry) {
        let state = match &log_entry.log {
            Log::NewShift(_) => GuardState::JustArrived(log_entry.datetime),
            Log::WakesUp => GuardState::Awoken(log_entry.datetime),
            Log::FallsAsleep => GuardState::Sleeping(log_entry.datetime),
        };
        self.states.push(state);
    }

    fn set_minutes(&mut self) {
        for (before, after) in self.states.iter().zip(self.states[1..].iter()) {
            if let (GuardState::Sleeping(start), GuardState::Awoken(end)) = (before, after) {
                for i in time_to_pos(start)..time_to_pos(end) {
                    self.minutes[i] += 1;
                }
            }
        }
    }
}

fn find_most_asleep_minute(guard: &Guard) -> usize {
    let (min, _) = guard
        .minutes
        .iter()
        .enumerate()
        .max_by_key(|&(_, i)| i)
        .unwrap();
    min
}

fn go<K, F>(guard_sort_key: F)
where
    K: Ord,
    F: for<'a> FnMut(&'a &Guard) -> K,
{
    let content = fs::read_to_string("input.txt").unwrap();
    let mut logs: Vec<LogEntry> = content
        .lines()
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
            .or_insert_with(|| Guard::new(current_gid.unwrap())))
        .push_state(&log);
    }

    guards.values_mut().for_each(Guard::set_minutes);

    let most_asleep_guard = guards.values().max_by_key(guard_sort_key).unwrap();;
    let most_asleep_minute = find_most_asleep_minute(&most_asleep_guard);
    println!(
        "{}",
        (most_asleep_guard.id as usize) * (most_asleep_minute % 60)
    );
}

fn part1() {
    go(|g| g.minutes.iter().sum::<u16>());
}


fn part2() {
    go(|g| *g.minutes.iter().max().unwrap());
}

fn main() {
    match env::args().find(|arg| arg == "1") {
        Some(_) => part1(),
        None => part2(),
    };
}
