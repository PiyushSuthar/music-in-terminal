// pub enum InputMode {
//     Normal,
//     Editing
// }
pub struct App {
    pub input: String,
    pub current: usize,
}
impl Default for App {
    fn default() -> Self {
        App {
            current: 0,
            input: String::new(),
        }
    }
}
