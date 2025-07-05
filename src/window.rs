use derive_builder::Builder;

#[derive(Builder)]
#[builder(name = "Builder", pattern = "owned", default)]
pub struct Window {
    pub size: (u32, u32),
    pub fullscreen: bool,
    pub title: String,
}
impl Default for Window {
    fn default() -> Self {
        Self {
            fullscreen: false,
            size: (800, 600),
            title: "Delm".into(),
        }
    }
}
