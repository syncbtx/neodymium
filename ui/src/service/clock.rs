use std::time::{Duration, SystemTime, UNIX_EPOCH};

use amane::Service;

pub const OFFSET_REFRESH: u32 = 1;

pub const SECONDS_PER_DAY: i64 = 24 * 60 * 60;

/*
 * the config can't use chrono, so the utc offset comes from `date`
 * and the time itself is counted from the system clock each second
 */
#[derive(Default)]
pub struct Clock {
    // seconds east of utc, like +7 hours for Asia/Jakarta
    offset: i64,

    ticks: u32,

    // seconds since 1970 in local time
    local: i64,
}

#[derive(Debug)]
pub struct TimeParts {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub am: bool,
}

#[derive(Debug)]
pub struct DateParts {
    pub weekday: usize,
    pub day: usize,
    pub month: usize,
    pub year: u16,
}

impl Service for Clock {
    fn new() -> Self {
        let mut clock = Self {
            offset: read_offset(),
            ..Self::default()
        };

        clock.update();

        clock
    }

    // ticks every second, but only a new minute changes what is shown
    fn update(&mut self) -> bool {
        self.ticks += 1;

        if self.ticks >= OFFSET_REFRESH {
            self.ticks = 0;
            self.offset = read_offset();
        }

        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs() as i64;

        let prev = self.local;
        self.local = since_epoch + self.offset;

        // redraw whenever the displayed second changes
        self.local != prev
    }
}

impl Clock {
    pub fn time(&self) -> TimeParts {
        let today = self.local.rem_euclid(SECONDS_PER_DAY);

        let hours = today / 3600;
        let minutes = today % 3600 / 60;
        let seconds = today % 60;

        let am = if hours < 12 { true } else { false };

        TimeParts {
            hour: hours as u8,
            min: minutes as u8,
            sec: seconds as u8,
            am,
        }
    }

    // like "Tuesday, 29 Sep 2026"
    pub fn date(&self) -> DateParts {
        let days = self.local.div_euclid(SECONDS_PER_DAY);

        let weekday = days.rem_euclid(7) as usize;

        let (year, month, day) = civil_date(days);

        let month = month - 1;

        DateParts {
            weekday,
            day: day as usize,
            month: month as usize,
            year: year as u16,
        }
    }

    // year, month from 1 and day from 1
    pub fn today(&self) -> (i64, i64, i64) {
        let days = self.local.div_euclid(SECONDS_PER_DAY);

        civil_date(days)
    }

    // 24 hour time of some other moment, like "21:05" for when a notification came
    pub fn hours_minutes(&self, moment: SystemTime) -> String {
        let since_epoch = moment
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs() as i64;

        let local = since_epoch + self.offset;

        let today = local.rem_euclid(SECONDS_PER_DAY);

        let hours = today / 3600;
        let minutes = today % 3600 / 60;

        format!("{hours:02}:{minutes:02}")
    }
}

/*
 * turns days since 1970 into year, month and day, using howard
 * hinnant's method: count in 400 year cycles of 146097 days,
 * with years starting in march so the leap day falls at the end
 */
pub fn civil_date(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;

    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted.rem_euclid(146_097);

    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36524 - day_of_era / 146_096) / 365;

    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);

    let month_from_march = (5 * day_of_year + 2) / 153;

    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;

    let month = if month_from_march < 10 {
        month_from_march + 3
    } else {
        month_from_march - 9
    };

    let year = year_of_era + era * 400 + i64::from(month <= 2);

    (year, month, day)
}

// `date +%z` prints the offset like "+0700" or "-0330"
fn read_offset() -> i64 {
    let text = amane::output("date +%z");

    let text = text.trim();

    let Some(sign) = text.chars().next() else {
        return 0;
    };

    let digits = &text[1..];

    if digits.len() != 4 {
        return 0;
    }

    let hours: i64 = digits[..2].parse().unwrap_or(0);
    let minutes: i64 = digits[2..].parse().unwrap_or(0);

    let offset = hours * 3600 + minutes * 60;

    if sign == '-' { -offset } else { offset }
}
