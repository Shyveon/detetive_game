pub enum MenuItem {
    Historia,
    Dicas,
    Portas,
    Morte,
    Vitoria
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Historia => 0,
            MenuItem::Dicas => 1,
            MenuItem::Portas => 2,
            MenuItem::Morte => 3,
            MenuItem::Vitoria => 4
        }
    }
}