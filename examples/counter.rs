use delm::run;

struct Model {
    count: i64,
}
enum Message {
    Increment,
    Decrement,
}
impl Model {
    fn new() -> Self {
        Self { count: 0 }
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }
    fn view(&self) -> ! {
        todo!()
    }
}

fn main() {
    run(Model::new(), Model::update, Model::view).unwrap();
}
