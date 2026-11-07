use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::{Duration, Instant};

use amane::{Animation, Easing, Service};

static START: LazyLock<Instant> = LazyLock::new(Instant::now);

pub fn phase() -> f32 {
    amane::request_frame();

    START.elapsed().as_secs_f32() * 6.0
}

#[derive(Clone, Copy)]
pub struct Spec {
    pub duration: Duration,
    pub easing: Easing,
}

impl Spec {
    /// Slow, soft: for data that changes every second or two.
    pub const SMOOTH: Spec = Spec {
        duration: Duration::from_millis(900),
        easing: Easing::InOut,
    };

    /// Quick and responsive: for hover and press feedback.
    pub const _SNAPPY: Spec = Spec {
        duration: Duration::from_millis(180),
        easing: Easing::Out,
    };
}

struct Motion {
    tracks: Mutex<HashMap<&'static str, Animation>>,
}

impl Service for Motion {
    fn new() -> Self {
        Self {
            tracks: Mutex::new(HashMap::new()),
        }
    }

    // nothing to poll: values only move when a view asks for them
    fn listen() {}
}

/// The value for `key`, moving toward `target`. `spec` only applies the
/// first time a key is seen.
pub fn animate(key: &'static str, target: f32, spec: Spec) -> f32 {
    let motion = Motion::read();

    let mut tracks = motion.tracks.lock().unwrap_or_else(PoisonError::into_inner);

    let track = tracks.entry(key).or_insert_with(|| {
        Animation::new(target)
            .duration(spec.duration)
            .easing(spec.easing)
    });

    // same target: does nothing
    track.to(target);

    track.value()
}

pub fn smooth(key: &'static str, target: f32) -> f32 {
    animate(key, target, Spec::SMOOTH)
}
