

pub trait DrawApi{
    fn draw_circle(&self, x: usize, y: usize, radius: usize);
    fn draw_rectangle(&self, x: usize, y: usize);
    fn draw_square(&self, x: usize);
}