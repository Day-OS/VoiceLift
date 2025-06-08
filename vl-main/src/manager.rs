pub trait Manager {
    fn modify_app(&mut self, app: &mut bevy::app::App);
}
