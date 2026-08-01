use rand::Rng;
use chrono::{DateTime, Utc, Duration, Timelike};
use crate::config::{ScheduleConfig, ScheduleMode};
use tracing::{info, debug};

pub struct Scheduler {
    config: ScheduleConfig,
    rng: rand::rngs::ThreadRng,
}

impl Scheduler {
    pub fn new(config: ScheduleConfig) -> Self {
        Self {
            config,
            rng: rand::thread_rng(),
        }
    }

    /// Calculate the next commit time based on strategy
    pub fn next_commit_time(&mut self) -> DateTime<Utc> {
        let base_interval = self.calculate_base_interval();
        let mut next_time = Utc::now() + base_interval;

        // Apply timezone optimization if enabled
        if self.config.timezone_optimization {
            next_time = self.optimize_for_timezone(next_time);
        }

        // Apply turbo multiplier (reduce interval)
        if self.config.turbo_multiplier > 1.0 {
            let reduced_interval = base_interval.num_minutes() as f64 / self.config.turbo_multiplier;
            next_time = Utc::now() + Duration::minutes(reduced_interval as i64);
        }

        info!("Next commit scheduled for: {}", next_time.format("%Y-%m-%d %H:%M:%S UTC"));
        next_time
    }

    fn calculate_base_interval(&mut self) -> Duration {
        match self.config.mode {
            ScheduleMode::Gentle => {
                // Once per day with some variance
                Duration::hours(24) + Duration::minutes(self.rng.gen_range(-60..60))
            }
            ScheduleMode::Balanced => {
                // 2-3 times per day
                let base_hours = 24 / self.rng.gen_range(2..=3);
                Duration::hours(base_hours) + Duration::minutes(self.rng.gen_range(-30..30))
            }
            ScheduleMode::Aggressive => {
                // 4-8 times per day
                let base_hours = 24 / self.rng.gen_range(4..=8);
                Duration::hours(base_hours) + Duration::minutes(self.rng.gen_range(-15..15))
            }
            ScheduleMode::AggressiveRandom => {
                // Random interval between min and max
                let minutes = self.rng.gen_range(
                    self.config.min_interval_minutes..=self.config.max_interval_minutes
                );
                Duration::minutes(minutes as i64)
            }
            ScheduleMode::Turbo => {
                // Use min interval with turbo multiplier applied later
                Duration::minutes(self.config.min_interval_minutes as i64)
            }
        }
    }

    fn optimize_for_timezone(&mut self, time: DateTime<Utc>) -> DateTime<Utc> {
        let hour = time.hour();
        
        // Avoid suspicious hours (2 AM - 6 AM in any major timezone)
        // This helps commits look more organic
        if (2..6).contains(&hour) {
            debug!("Adjusting time to avoid suspicious hours");
            // Push to morning
            let adjustment = self.rng.gen_range(4..8);
            return time + Duration::hours(adjustment);
        }

        // Optionally shift to different timezones to spread commits globally
        // This makes the contribution pattern look more like a distributed team
        if self.rng.gen_bool(0.3) {
            let timezone_shifts = vec![-8, -5, 0, 8]; // PST, EST, UTC, Asia
            let shift = timezone_shifts[self.rng.gen_range(0..timezone_shifts.len())];
            debug!("Applying timezone shift: {} hours", shift);
            return time + Duration::hours(shift);
        }

        time
    }

    /// Check if it's time for next commit
    pub fn should_commit(&self, _last_commit_time: DateTime<Utc>, next_scheduled: DateTime<Utc>) -> bool {
        Utc::now() >= next_scheduled
    }

    /// Get a descriptive string of the current schedule
    pub fn describe(&self) -> String {
        let base_desc = match self.config.mode {
            ScheduleMode::Gentle => "Gentle (1 commit/day)",
            ScheduleMode::Balanced => "Balanced (2-3 commits/day)",
            ScheduleMode::Aggressive => "Aggressive (4-8 commits/day)",
            ScheduleMode::AggressiveRandom => {
                &format!("Aggressive Random ({}-{} min intervals)", 
                    self.config.min_interval_minutes, 
                    self.config.max_interval_minutes)
            }
            ScheduleMode::Turbo => "TURBO MODE 🚀",
        };

        let mut desc = base_desc.to_string();
        
        if self.config.turbo_multiplier > 1.0 {
            desc.push_str(&format!(" [Turbo: {}x]", self.config.turbo_multiplier));
        }
        
        if self.config.timezone_optimization {
            desc.push_str(" [Timezone Optimized]");
        }

        desc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let config = ScheduleConfig {
            mode: ScheduleMode::AggressiveRandom,
            min_interval_minutes: 30,
            max_interval_minutes: 180,
            timezone_optimization: true,
            turbo_multiplier: 1.0,
        };

        let scheduler = Scheduler::new(config);
        assert!(scheduler.describe().contains("Aggressive Random"));
    }

    #[test]
    fn test_next_commit_time() {
        let config = ScheduleConfig {
            mode: ScheduleMode::Gentle,
            min_interval_minutes: 30,
            max_interval_minutes: 180,
            timezone_optimization: false,
            turbo_multiplier: 1.0,
        };

        let mut scheduler = Scheduler::new(config);
        let next_time = scheduler.next_commit_time();
        
        // Should be roughly 24 hours from now
        let expected = Utc::now() + Duration::hours(23);
        assert!(next_time > expected);
    }
}
