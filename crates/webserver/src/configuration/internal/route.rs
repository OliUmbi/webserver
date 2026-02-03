use crate::configuration::internal::action::Action;
use crate::configuration::internal::path::Path;

#[derive(Debug)]
pub struct Route {
    pub path: Path,
    pub action: Action,
}
