use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SquishDirection {
    Expand,
    Shrink,
}

#[derive(Resource)]
pub struct SideEffects {
    direction: SquishDirection,
    squish_factor: f32,
    squish_timer: Timer,
    pub sedated: bool,
}
impl SideEffects {
    pub fn new() -> Self {
        let squish_timer = Timer::from_seconds(2., TimerMode::Once);
        Self {
            direction: SquishDirection::Shrink,
            squish_factor: 1.,
            squish_timer, 
            sedated: false,
        }
    }
    pub fn total_squish_factor(&self) -> f32 {
        match self.direction {
            SquishDirection::Expand => self.squish_timer.percent() + 1.,
            SquishDirection::Shrink => self.squish_timer.percent_left() + 1.,
        }
    }
    pub fn squish_factor(&self) -> f32 {
        self.squish_factor
    }
    pub fn is_active(&self) -> bool {
        !self.squish_timer.paused() && !self.squish_timer.finished()
    }
    pub fn start_squish_timer(&mut self, direction: SquishDirection) {
        if self.squish_timer.paused() {
            self.squish_timer.unpause();
        }
        if direction != self.direction {
            self.direction = direction;
            if self.squish_timer.finished() {
                self.squish_timer.reset();
            } else {
                self.squish_timer.set_elapsed(self.squish_timer.duration() - self.squish_timer.elapsed());
            }
        }
    }
    pub fn tick(&mut self, delta: Duration) {
        if self.is_active() {
            let progress_before = self.total_squish_factor();
            self.squish_timer.tick(delta);
            self.squish_factor = self.total_squish_factor() / progress_before;
        } else {
            self.squish_factor = 1.;
        }
    }
}

