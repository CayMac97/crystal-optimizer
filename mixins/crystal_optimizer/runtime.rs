// Marlow's Crystal Optimizer runtime module
use std::sync::atomic::{AtomicBool, Ordering};

pub const ID: &str = "crystal_optimizer";
pub const NAME: &str = "Marlow's Crystal Optimizer";
pub const VERSION: &str = "1.0.0";
pub const DESCRIPTION: &str =
    "Competitive Crystal PvP optimizer: client-side 0ms break prediction, zero miss penalty, and fast place.";

pub const KEY_TOGGLE: &str = "toggle";
pub const KEY_TOGGLE_LABEL: &str = "Toggle Crystal Optimizer";
pub const KEY_TOGGLE_DEFAULT: &str = "key.keyboard.unknown";

static ENABLED: AtomicBool = AtomicBool::new(true);
static PREDICT_BREAK: AtomicBool = AtomicBool::new(true);
static AUTO_BREAK_HOLD: AtomicBool = AtomicBool::new(true);
static FAST_PLACE: AtomicBool = AtomicBool::new(true);

pub fn is_enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn toggle_enabled() -> bool {
    let prev = ENABLED.fetch_xor(true, Ordering::Relaxed);
    !prev
}

pub fn predict_break_enabled() -> bool {
    PREDICT_BREAK.load(Ordering::Relaxed)
}

pub fn toggle_predict_break() -> bool {
    let prev = PREDICT_BREAK.fetch_xor(true, Ordering::Relaxed);
    !prev
}

pub fn auto_break_enabled() -> bool {
    AUTO_BREAK_HOLD.load(Ordering::Relaxed)
}

pub fn toggle_auto_break() -> bool {
    let prev = AUTO_BREAK_HOLD.fetch_xor(true, Ordering::Relaxed);
    !prev
}

pub fn fast_place_enabled() -> bool {
    FAST_PLACE.load(Ordering::Relaxed)
}

pub fn toggle_fast_place() -> bool {
    let prev = FAST_PLACE.fetch_xor(true, Ordering::Relaxed);
    !prev
}
