use super::constants::DO_NOTHING_ACTION;

/// Does nothing.
#[derive(Serialize, Deserialize, Debug)]
pub struct DoNothingAction {
    action: String,
    hide_app: bool,
}

impl DoNothingAction {

    pub fn new() -> Self {

        DoNothingAction {
            action: DO_NOTHING_ACTION.to_string(),
            hide_app: true, // by default
        }
    }

    pub fn hide_app(&mut self, hide: bool) {
        self.hide_app = hide;
    }
}

use super::base_action::BaseAction;
impl BaseAction for DoNothingAction {

    fn keep_app_open(&self) -> bool { self.hide_app }

    fn run(self) -> Result<(), ()> {
        // Does Nothing
        Ok( () )
    }
}