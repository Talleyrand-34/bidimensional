//create a mutable struct of polygon and add the methos get area and perimeter of the polygon
#[derive(Debug, Clone)]
pub struct MutPolygon {
    vertices: Vec<Point2<f64>>,
}

impl MutPolygon {
    pub fn new(vertices: Vec<Point2<f64>>) -> MutPolygon {
        MutPolygon { vertices }
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

