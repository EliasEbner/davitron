pub trait Entity {
    fn update(self: &mut Self);
    fn draw(self: &mut Self);
}
