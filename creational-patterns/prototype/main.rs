const PI:f32=3.14;

#[derive(Clone,Debug)]
pub struct Circle{
    pub radius: f32,
}
impl Circle {
    pub fn area(self)->f32{
        PI*(self.radius*self.radius)
    }
}


fn main(){
   let circle1=Circle{ radius:5.0};
   let circle2=circle1.clone();

   println!("The new circle is cloned : {:?}",circle2);

}