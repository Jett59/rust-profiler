use std::{collections::HashMap, sync::Mutex, time::Duration};

use once_cell::sync::Lazy;

pub use proc_macros::profiled;

static PROFILE_DATA: Lazy<Mutex<HashMap<&'static str, Duration>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn record_data(name: &'static str, duration: Duration) {
    let mut data = PROFILE_DATA.lock().unwrap();
    if data.contains_key(name) {
        *data.get_mut(name).unwrap() += duration;
    } else {
        data.insert(name, duration);
    }
}

pub fn get_data() -> Vec<(&'static str, Duration)> {
    let data = PROFILE_DATA.lock().unwrap();
    let mut data: Vec<_> = data
        .iter()
        .map(|(name, duration)| (*name, *duration))
        .collect();
    data.sort_by_key(|(_, duration)| *duration);
    data
}

#[cfg(test)]
mod test {
    use super::*;

    #[profiled(slow_function)]
    fn slow_function(seconds: u64) {
        std::thread::sleep(std::time::Duration::from_secs(seconds));
    }

    #[test]
    fn test() {
        slow_function(1);
        slow_function(2);
        slow_function(3);
        let data = get_data();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].0, "slow_function");
        assert!(data[0].1 >= std::time::Duration::from_secs(6));
    }
}
