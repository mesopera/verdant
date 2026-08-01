use rand::Rng;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub timestamp: DateTime<Utc>,
    pub purrformance_score: f64,
    pub box_occupancy_rate: f64,
    pub zoomie_frequency: u32,
    pub treat_conversion_rate: f64,
    pub streak_health: u32,
    pub territorial_coverage: TerritorialCoverage,
    pub nap_correlation: NapCorrelation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TerritorialCoverage {
    pub monday: Vec<f64>,
    pub tuesday: Vec<f64>,
    pub wednesday: Vec<f64>,
    pub thursday: Vec<f64>,
    pub friday: Vec<f64>,
    pub saturday: Vec<f64>,
    pub sunday: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NapCorrelation {
    pub nap_hours: Vec<u32>,
    pub code_quality: Vec<u32>,
}

pub struct MetricsGenerator {
    rng: rand::rngs::ThreadRng,
}

impl MetricsGenerator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Generate new metrics data
    pub fn generate(&mut self) -> Metrics {
        Metrics {
            timestamp: Utc::now(),
            purrformance_score: self.generate_purrformance_score(),
            box_occupancy_rate: self.rng.gen_range(0.60..0.95),
            zoomie_frequency: self.rng.gen_range(30..80),
            treat_conversion_rate: self.rng.gen_range(2.5..4.5),
            streak_health: self.rng.gen_range(85..100),
            territorial_coverage: self.generate_territorial_coverage(),
            nap_correlation: self.generate_nap_correlation(),
        }
    }

    fn generate_purrformance_score(&mut self) -> f64 {
        // Generate score with some variance around 85
        let base: f64 = 85.0;
        let variance: f64 = self.rng.gen_range(-10.0..13.0);
        let score: f64 = base + variance;
        score.max(70.0).min(98.0)
    }

    fn generate_territorial_coverage(&mut self) -> TerritorialCoverage {
        TerritorialCoverage {
            monday: self.generate_hourly_data(),
            tuesday: self.generate_hourly_data(),
            wednesday: self.generate_hourly_data(),
            thursday: self.generate_hourly_data(),
            friday: self.generate_hourly_data(),
            saturday: self.generate_hourly_data(),
            sunday: self.generate_hourly_data(),
        }
    }

    fn generate_hourly_data(&mut self) -> Vec<f64> {
        (0..24)
            .map(|hour| {
                // Simulate realistic patterns (lower at night, higher during work hours)
                let base: f64 = match hour {
                    0..=5 => 0.1,   // Night
                    6..=8 => 0.3,   // Morning
                    9..=17 => 0.7,  // Work hours
                    18..=22 => 0.5, // Evening
                    _ => 0.2,       // Late night
                };
                let variance: f64 = self.rng.gen_range(-0.2..0.2);
                let value: f64 = base + variance;
                value.max(0.0).min(1.0)
            })
            .collect()
    }

    fn generate_nap_correlation(&mut self) -> NapCorrelation {
        NapCorrelation {
            nap_hours: vec![4, 5, 6, 7, 8, 9, 10],
            code_quality: vec![
                self.rng.gen_range(55..70),
                self.rng.gen_range(65..80),
                self.rng.gen_range(75..90),
                self.rng.gen_range(85..95),
                self.rng.gen_range(80..92),
                self.rng.gen_range(70..85),
                self.rng.gen_range(60..75),
            ],
        }
    }

    /// Generate metrics JSON string
    pub fn generate_json(&mut self) -> Result<String, serde_json::Error> {
        let metrics = self.generate();
        serde_json::to_string_pretty(&metrics)
    }
}
