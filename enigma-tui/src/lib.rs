pub mod colour;
pub use colour::Colour;
pub use colour::Modifier;
pub use colour::Style;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

/*

struct App{
    balls : String,
    mode : AppMode,
}

impl Tui for App{
    fn render(&mut self) -> Element{

    }
}


loop{
    app.render();
    if let Some(Ok(key)) = termion::async_stdin().keys().next(){}
}

*/
