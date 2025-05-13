pub struct WithPriority<T> {
    pub payload: T,
    pub priority: f32,
}
impl<T> WithPriority<T> {
    pub fn default(payload: T) -> Self {
        Self {
            payload,
            priority: 0.0,
        }
    }

    pub fn new(payload: T, priority: f32) -> Self {
        Self { payload, priority }
    }

    pub fn unpack(self) -> T {
        self.payload
    }
}
impl<T> PartialEq for WithPriority<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> PartialOrd for WithPriority<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> Ord for WithPriority<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.total_cmp(&other.priority)
    }
}

impl<T> Eq for WithPriority<T> {}
