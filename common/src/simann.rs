use rand::Rng;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SimAnnMode {
    Maximize,
    Minimize,
}

pub struct SimAnn {
    timer: Instant,
    mode: SimAnnMode,
    duration: Duration,
    start_temp: f64,
    end_temp: f64,
}

impl SimAnn {
    pub fn new(mode: SimAnnMode, duration: Duration, start_temp: f64, end_temp: f64) -> Self {
        Self {
            timer: Instant::now(),
            mode,
            duration,
            start_temp,
            end_temp,
        }
    }

    pub fn should_keep_trying(&self) -> bool {
        self.timer.elapsed() < self.duration
    }

    pub fn should_adopt<R: Rng>(&self, gen: &mut R, prev_score: f64, new_score: f64) -> bool {
        let ratio = self.timer.elapsed().as_millis() as f64 / self.duration.as_millis() as f64;
        let ratio = ratio.min(1.0).max(0.0);
        let temp = self.start_temp * (1.0 - ratio) + self.end_temp * ratio;
        if temp < 1e-8 {
            match self.mode {
                SimAnnMode::Maximize => prev_score < new_score,
                SimAnnMode::Minimize => prev_score > new_score,
            }
        } else {
            // 右辺は Maximize なら prev_score < new_score のとき 1.0 以上になるので OK
            // prev_score > new_score のときに確率で遷移を許す
            // Minimize ならその逆
            let diff = match self.mode {
                SimAnnMode::Maximize => new_score - prev_score,
                SimAnnMode::Minimize => prev_score - new_score,
            };

            gen.gen::<f64>() < (diff / temp).exp()
        }
    }
}
