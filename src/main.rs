use std::{rc::Rc, cell::RefCell};

fn main() {
    let app = Rc::new(RefCell::new(App::new())); // TODO
    start_ui(app)?;
    Ok(())
}