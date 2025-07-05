struct Model {
    size: (u32, u32),
    title: String,
}
impl Model {
    fn window(&self) -> delm::window::Builder {
        delm::window::Builder::default()
            // .size(self.size)
            .title(self.title.clone())
    }
}
fn main() {
    delm::render(
        Model {
            size: (1200, 700),
            title: "Hello!".to_string(),
        },
        Model::window,
        |_| {},
        |_, _: ()| {},
    );
}
