use bridge::{actions::DrawApi, objects::Circle};



struct WindowsDrawApi;

impl DrawApi for WindowsDrawApi {
    fn draw_circle(&self, x: usize, y: usize, radius: usize) {
        println!("This is circle");
    }

    fn draw_rectangle(&self, x: usize, y: usize) {
        println!("This is rectangle");
    }

    fn draw_square(&self, x: usize) {
        println!("This is square");
    }
}

struct MobileDrawApi;

impl DrawApi for MobileDrawApi {
    fn draw_circle(&self, x: usize, y: usize, radius: usize) {
        println!("This is circle");
    }

    fn draw_rectangle(&self, x: usize, y: usize) {
        println!("This is rectangle");
    }

    fn draw_square(&self, x: usize) {
        println!("This is square");
    }
}



fn main() {
    let mobile_draw= Circle::new(3, 7, 4, Box::new(MobileDrawApi));
    mobile_draw.draw();
}
