//create a inmutable struct of polygon and add the methos get area and perimeter of the polygon
use crate::coordinates::coordinate::eCoordinate;
#[derive(Debug, Clone)]
pub struct InmutPolygon {
    vertices: Vec<Point2<f64>>,
}

impl InmutPolygon {
    pub fn new(vertices: Vec<Point2<f64>>) -> InmutPolygon {
        InmutPolygon { vertices }
    }

    pub fn get_area(&self) -> f64 {
        let mut area = 0.0;
        let mut j = self.vertices.len() - 1;

        for i in 0..self.vertices.len() {
            area += (self.vertices[j].x + self.vertices[i].x)
                * (self.vertices[j].y - self.vertices[i].y);
            j = i;
        }
        area / 2.0
    }

    pub fn get_perimeter(&self) -> f64 {
        let mut perimeter = 0.0;

        for i in 0..self.vertices.len() {
            let j = (i + 1) % self.vertices.len();
            perimeter += (self.vertices[i] - self.vertices[j]).magnitude();
        }

        perimeter
    }

    pub fn get_vertices(&self) -> Vec<Point2<f64>> {
        self.vertices.clone()
    }
}

//create a test function with the macro [#test]
#[test]
fn test_polygon() {
    let p = InmutPolygon::new(vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);

    let p2 = MutPolygon::new(vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);
//complete the test function
    println!("p area: {}", p.get_area());
    println!("p2 area: {}", p2.get_area());
    println!("p perimeter: {}", p.get_perimeter());
    println!("p2 perimeter: {}", p2.get_perimeter());
    println!("p vertices: {:?}", p.get_vertices());
    println!("p2 vertices: {:?}", p2.get_vertices());
}

