pub enum Interval {
    Empty,
    Universe,
    Range(f64, f64),
}

impl Interval {
    pub fn empty() -> Self {
        Self::Empty
    }

    pub fn universe() -> Self {
        Self::Universe
    }

    pub fn range(start: impl Into<f64>, end: impl Into<f64>) -> Self {
        let start = start.into();
        let end = end.into();

        Self::Range(start, end)
    }

    pub fn size(&self) -> f64 {
        match self {
            Self::Empty => 0.0,
            Self::Universe => f64::INFINITY,
            Self::Range(start, end) => *end - *start,
        }
    }

    pub fn contains(&self, x: &f64) -> bool {
        match self {
            Self::Empty => false,
            Self::Universe => true,
            Self::Range(start, end) => (start..=end).contains(&x),
        }
    }

    pub fn surrounds(&self, x: &f64) -> bool {
        match self {
            Self::Empty => false,
            Self::Universe => true,
            Self::Range(start, end) => start < x && x < end,
        }
    }

    pub fn clamp(&self, x: &f64) -> f64 {
        match self {
            Self::Empty => 0.0,
            Self::Universe => *x,
            Self::Range(start, end) => (*x).max(*start).min(*end),
        }
    }
}
