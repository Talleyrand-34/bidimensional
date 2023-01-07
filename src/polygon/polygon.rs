use crate::polygon::inmut_polygon;

//create a new enum named polygon with two variants
//one mutable and one immutable
#[derive(Debug, Clone)]
pub enum Polygon {
    Inmut(inmut_polygon::InmutPolygon),
    Mut(mut_polygon::MutPolygon),
}

//implement the polygon trait for the polygon enum
impl polygon::Polygon for Polygon {
    fn get_area(&self) -> f64 {
        match self {
            Polygon::Inmut(p) => p.get_area(),
            Polygon::Mut(p) => p.get_area(),
        }
    }

    fn get_perimeter(&self) -> f64 {
        match self {
            Polygon::Inmut(p) => p.get_perimeter(),
            Polygon::Mut(p) => p.get_perimeter(),
        }
    }

    fn get_vertices(&self) -> Vec<Point2<f64>> {
        match self {
            Polygon::Inmut(p) => p.get_vertices(),
            Polygon::Mut(p) => p.get_vertices(),
        }
    }
}

//create a test function with the macro [#test]
#[test]
fn test_polygon() {
    let p = inmut_polygon::InmutPolygon::new(vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);

    let p2 = mut_polygon::MutPolygon::new(vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);

    let p3 = Polygon::Inmut(p);
    let p4 = Polygon::Mut(p2);

    println!("p3 area: {}", p3.get_area());
    println!("p4 area: {}", p4.get_area());
}

/*
I'm trying to create a polygon struct that can be either mutable or immutable. I want to have a single enum that can be either mutable or immutable. I've tried to do this by creating a trait that both mutable and immutable polygons implement. I've also created a new enum that can be either

*/
