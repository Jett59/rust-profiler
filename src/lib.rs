use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
    sync::Mutex,
    time::Duration,
};

use once_cell::sync::Lazy;

pub use proc_macros::profiled;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProfileData {
    pub total_execution_time: Duration,
    pub number_of_executions: u64,
}

impl AddAssign<Duration> for ProfileData {
    fn add_assign(&mut self, rhs: Duration) {
        self.total_execution_time += rhs;
        self.number_of_executions += 1;
    }
}

impl Add<Duration> for ProfileData {
    type Output = ProfileData;
    fn add(mut self, rhs: Duration) -> Self::Output {
        self += rhs;
        self
    }
}

static PROFILE_DATA: Lazy<Mutex<HashMap<&'static str, ProfileData>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn record_data(name: &'static str, duration: Duration) {
    let mut data = PROFILE_DATA.lock().unwrap();
    if data.contains_key(name) {
        *data.get_mut(name).unwrap() += duration;
    } else {
        data.insert(name, ProfileData::default() + duration);
    }
}

pub fn get_data() -> Vec<(&'static str, ProfileData)> {
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

    struct SlowStruct;

    impl SlowStruct {
        #[profiled(slow_struct_slow_method)]
        fn slow_method(&self, seconds: u64) {
            std::thread::sleep(std::time::Duration::from_secs(seconds));
        }
    }

    #[test]
    fn test() {
        slow_function(1);
        slow_function(2);
        let data = get_data();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].0, "slow_function");
        let slow_function_data = data[0].1;
        assert!(slow_function_data.total_execution_time >= std::time::Duration::from_secs(3));
        assert_eq!(slow_function_data.number_of_executions, 2);
        let slow_struct = SlowStruct;
        slow_struct.slow_method(1);
        slow_struct.slow_method(3);
        let data = get_data();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].0, "slow_function");
        assert_eq!(data[0].1, slow_function_data);
        assert_eq!(data[1].0, "slow_struct_slow_method");
        let slow_struct_slow_method_data = data[1].1;
        assert!(
            slow_struct_slow_method_data.total_execution_time >= std::time::Duration::from_secs(4)
        );
        assert_eq!(slow_struct_slow_method_data.number_of_executions, 2);
    }
}
