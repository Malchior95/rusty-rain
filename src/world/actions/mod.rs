pub mod gather_resource_action;
pub mod store_action;
pub mod supply_action;
//I just had Rust implicilty copy this, without putting it back... so the actual value in enum was
//never updated. Need to be careful with Copy
pub struct BasicAction {
    pub progress: f32,
    pub requirement: f32,
}

#[derive(Default)]
pub enum ActionResult {
    #[default]
    InProgress,
    Completed,
}

impl BasicAction {
    pub fn new(requirement: f32) -> Self {
        Self { progress: 0.0, requirement }
    }

    pub fn process(
        &mut self,
        delta: f32,
    ) -> ActionResult {
        self.progress += delta;
        if self.progress > self.requirement {
            ActionResult::Completed
        } else {
            ActionResult::InProgress
        }
    }
}
