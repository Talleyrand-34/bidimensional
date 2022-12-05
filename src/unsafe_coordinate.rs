
struct unsafe_Coordinate{
    x: f32,
    y: f32
}

impl Coordinate{
    fn distancia(&self, otro: &Coordinate) -> f32 {
        let dif_x = self.x - otro.x;
        let dif_y = self.y - otro.y;
        (dif_x.powi(2) + dif_y.powi(2)).sqrt()
    }
    fn negative(&self) -> Coordinate{
        let new_c:Coordinate= Coordinate{x: -self.x , y: -self.y};
        return new_c;
    }

    fn add(&mut self, otro: &Coordinate) -> (){
        self.x+=otro.x;
        self.y+=otro.y;
    }
    fn sub(&mut self, otro: &Coordinate) ->(){
        self.x+=otro.x;
        self.y+=otro.y;
    }
    fn negative(&self){
        self.x=-self.x;
        self.y=-self.y;
    }


}