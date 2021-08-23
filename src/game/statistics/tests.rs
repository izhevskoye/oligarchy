use std::collections::HashMap;

use super::StatisticTracker;

#[test]
fn merge_statistics_tracker() {
    let mut a = {
        let mut data = HashMap::new();
        data.insert("a".to_owned(), 10.0);

        StatisticTracker { data }
    };

    let b = {
        let mut data = HashMap::new();
        data.insert("a".to_owned(), 10.0);
        data.insert("b".to_owned(), 10.0);

        StatisticTracker { data }
    };

    a.merge(&b);

    assert!((a.data.get("b").unwrap() - 10.0).abs() < f64::EPSILON);
    assert!((a.data.get("a").unwrap() - 20.0).abs() < f64::EPSILON);
}

#[test]
fn tracker() {
    let mut tracker = StatisticTracker::default();
    assert!(tracker.get("a") < f64::EPSILON);
    tracker.track("a", 10.0);
    assert!((tracker.get("a") - 10.0).abs() < f64::EPSILON);
    tracker.track("a", 2.0);
    assert!((tracker.get("a") - 12.0).abs() < f64::EPSILON);
}
