use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};

pub struct OrderedFloat(f64);

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .partial_cmp(&other.0)
            .expect("f64 should be comparable")
    }
}

impl Eq for OrderedFloat {}

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl TryFrom<f64> for OrderedFloat {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value.is_nan() {
            Err("value is NaN".to_string())
        } else {
            Ok(OrderedFloat(value))
        }
    }
}

impl From<OrderedFloat> for f64 {
    fn from(value: OrderedFloat) -> Self {
        value.0
    }
}
