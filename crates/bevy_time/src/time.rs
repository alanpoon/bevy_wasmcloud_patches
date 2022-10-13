use bevy_ecs::reflect::ReflectResource;
use bevy_reflect::Reflect;
use bevy_utils::{Duration, Instant};
/// Tracks elapsed time since the last update and since the App has started

#[derive(Reflect,Debug,Clone,Default)]
#[reflect(Resource)]
pub struct Time{
  pub delta: Duration,
  pub delta_seconds: f32,
  pub timestamp: u64,
  pub startup:u64,
  pub last_update: u64,
}
impl Time {
    /// Updates the internal time measurements.
    ///
    /// Calling this method on the [`Time`] resource as part of your app will most likely result in
    /// inaccurate timekeeping, as the resource is ordinarily managed by the
    /// [`TimePlugin`](crate::TimePlugin).


    /// The delta between the current tick and last tick as a [`Duration`]
    #[inline]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    /// The delta between the current and last tick as [`f32`] seconds
    #[inline]
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    /// The delta between the current and last tick as [`f64`] seconds
    #[inline]
    pub fn delta_seconds_f64(&self) -> f64 {
        self.delta_seconds as f64
    }

    /// The [`Instant`] when [`Time::update`] was last called, if it exists
    #[inline]
    pub fn last_update(&self) -> u64 {
        self.last_update
    }
    pub fn update_with_timestamp(&mut self,timestamp:u64){
        if self.startup==0{
            self.startup = timestamp;
        }
        if self.last_update!=0{
            self.delta_seconds = (timestamp - self.last_update) as f32 / 1000.0;
        }
        self.last_update = timestamp;
        self.timestamp = timestamp;
    }
}