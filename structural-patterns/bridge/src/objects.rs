use crate::actions::DrawApi;

struct Shape{
    action: Box<dyn DrawApi>
}

impl Shape {
    fn new(action: Box<dyn DrawApi>)->Self{
        Shape{action}
    }
}

pub struct Circle{
    pub x: usize,
    pub y: usize,
    pub radius: usize,
    pub action: Box<dyn DrawApi>
}
impl Circle {
    pub fn new(x: usize, y:usize, radius: usize, action: Box<dyn DrawApi>)->Self{
        Circle { x, y, radius, action }
    }
    pub fn draw(&self){
        &self.action.draw_circle(self.x, self.y, self.radius);
    }
}

pub struct Rectangle{
    pub x: usize,
    pub y: usize,
    pub action: Box<dyn DrawApi>
}
impl Rectangle {
    pub fn new(x: usize, y:usize, action: Box<dyn DrawApi>)->Self{
        Rectangle { x, y, action }
    }
    pub fn draw(&self){
        if self.x==self.y{
            &self.action.draw_square(self.x);
        }
        else {
            &self.action.draw_rectangle(self.x, self.y);
        }
        
    }
}

pub struct Square{
    pub x: usize,
    pub action: Box<dyn DrawApi>
}

impl Square {
    pub fn new(x: usize, action: Box<dyn DrawApi>)->Self{
        Square { x, action }
    }
    
    pub fn draw(&self){
        &self.action.draw_square(self.x);
    }
}