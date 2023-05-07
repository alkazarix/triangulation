


#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64, 
}

impl PartialEq for Point {

    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 0.0001 && (self.y - other.y).abs() < 0.0001
    }
}

impl Point {

    fn dist(&self, other: &Self) -> f64 {
        (self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)
    } 
    
}


#[derive(Debug, Clone, Copy)]
struct Edge {
    pub vertex: [Point; 2]
}

impl Edge {
    fn new(a: Point, b: Point) -> Self {
        let vertex = [a, b];
        Self{vertex }
    }
}

impl PartialEq for Edge {
    
    fn eq(&self, other: &Self) -> bool {
        (self.vertex[0] == other.vertex[0] && self.vertex[1] == other.vertex[1]) || 
        (self.vertex[1] == other.vertex[0] && self.vertex[0] == other.vertex[1])
    }
}

impl Eq for Edge {
    
}

#[derive(Debug, Clone, Copy)]
struct Circle {
    radius: f64,
    center: Point
}

impl Circle {

    fn contains(&self, point: &Point) -> bool {
        point.dist(&self.center) < self.radius
    }
    
}



#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    vertex: [Point; 3],
    circumcircle: Circle, 
    edges: [Edge; 3],
    
}

impl PartialEq for Triangle {
    
    fn eq(&self, other: &Self) -> bool {
        other.vertex.iter().map(|other_vertex| {
            self.vertex.iter()
                .any(|vertex| {other_vertex == vertex })
        })
        .fold(true, |acc, is_equal|{ acc && is_equal })
    }
}

impl Triangle {
    
    pub fn new(p0: Point, p1: Point, p2:Point) -> Self {
        let vertex :[Point; 3] = [p0, p1, p2];

        let edges = [Edge::new(vertex[0], vertex[1]),
        Edge::new(vertex[1], vertex[2]),
        Edge::new(vertex[2], vertex[0])];

        let ax = p1.x - p0.x;
        let ay = p1.y - p0.y;
        let bx = p2.x - p0.x;
        let by = p2.y - p0.y;
        
        let m = p1.x * p1.x - p0.x * p0.x + p1.y * p1.y - p0.y * p0.y;
        let u = p2.x * p2.x - p0.x * p0.x + p2.y * p2.y - p0.y * p0.y;
        let s = 1.0 / (2.0 * f64::from(ax * by - ay * bx));
        
        let center_x = f64::from((p2.y - p0.y) * m + (p0.y - p1.y) * u) * s;
        let center_y = f64::from((p0.x - p2.x) * m + (p1.x - p0.x) * u) * s;
    

       let center = Point{x: center_x, y: center_y};

        let circumcircle = Circle{center , radius: center.dist(&p0)}; 


        Self { vertex, edges, circumcircle } 
    }


    fn circumcircle(&self) -> Circle {
       self.circumcircle
    }

    pub fn center(&self) ->  Point {
        let center_x = self.vertex.iter().fold(0.0, |acc, point, | {  acc + (point.x / 3.0)});
        let center_y = self.vertex.iter().fold(0.0, |acc, point, | {  acc + (point.y / 3.0)});
        Point{x: center_x, y: center_y}
    }

    pub fn vertex(&self) ->  [Point; 3] {
        self.vertex
    }
}


pub struct Delaunay {
    height: f64,
    width: f64,
    triangles: Vec<Triangle>
}

impl Delaunay {

    pub fn new(height: f64, width: f64) -> Self {
        let mut delaunay = Self {height, width, triangles: vec![]};
        delaunay.initialize();
        delaunay

    }

    pub fn initialize(&mut self) {
        self.triangles.clear();

        // Create the supertriangle, an artificial triangle which encompasses all the points.
        let a = Point { x: 0.0, y: 0.0 };
        let b = Point { x: self.width, y: 0.0 };
        let c = Point { x: self.width , y: self.height };
        let d = Point { x: 0.0, y : self.height};

        self.triangles.push(Triangle::new(a, b, c));
        self.triangles.push(Triangle::new(a, c, d));
    }

    pub fn add_points(&mut self, points: Vec<Point>) {

    

        for (_, p) in points.iter().enumerate() {
            let (x, y) = (p.x, p.y);
            let triangles = &self.triangles;
            let mut edges = Vec::new();
            let mut temps = Vec::new();

            

            for t in triangles {
                let circle = t.circumcircle();
              
                if circle.contains(p) {
                    edges.push(t.edges[0]);
                    edges.push(t.edges[1]);
                    edges.push(t.edges[2]);
                } else {
                    temps.push(*t);
                }
            }

            let mut polygon = Vec::new();

            'edges: for e in edges {
                for j in 0..polygon.len() {
                    if e == polygon[j] {
                        polygon.remove(j);
                        continue 'edges;
                    }
                }
                polygon.push(e);
            }

            for e in polygon {
                temps.push(Triangle::new(e.vertex[0], e.vertex[1], Point{x, y }));
            }
            self.triangles = temps;
        }

    }


  
    pub fn triangles(&self)  -> Vec<Triangle>{
       self.triangles.clone()
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn egde_eq() {

        let a = Point{x: 20.0, y: 50.0};
        let b = Point{x: 50.0, y: 70.0};
        let ab = Edge::new(a, b);

        let test_cases = vec![
            Edge::new(Point { x: a.x, y: a.y }, Point { x: b.x , y: b.y }),
            Edge::new( Point { x: b.x , y: b.y }, Point { x: a.x, y: a.y }),
        ];

        test_cases.into_iter().for_each(|e| {
            assert!(ab == e);
        });

    }

    #[test]
    pub fn triangle_eq() {

        let a = Point{x: 20.0, y: 50.0};
        let b = Point{x: 50.0, y: 70.0};
        let c = Point{x: 20.0, y : 20.0}; 
        let abc = Triangle::new(a, b, c);

        assert!(abc == Triangle::new(a, b, c));
        assert!(abc == Triangle::new(a, c, b));
        assert!(abc == Triangle::new(b, a, c));
        assert!(abc == Triangle::new(b, c, a));
        assert!(abc == Triangle::new(c, a, b));
        assert!(abc == Triangle::new(c, b, a));
    }

    #[test]
    pub fn delaunay_add_points() {

        let a = Point{x: 0.0, y: 0.0};
        let b = Point{x: 100.0, y: 0.0};
        let c = Point{x: 100.0, y : 100.0}; 
        let d = Point{x: 0.0, y: 100.0};
        let e = Point{x: 25.0, y: 25.0};
        let f = Point{x: 75.0, y : 75.0}; 

        let points = vec![e, f];

        let mut delanay = Delaunay::new(100.0, 100.0);
        delanay.add_points(points);

        let triangles = delanay.triangles();

        let expected_triangles = vec![
            Triangle::new(a, d, e),
            Triangle::new(a, b, e),
            Triangle::new(d, e, f),
            Triangle::new(d, c, f),
            Triangle::new(b, c, f),
            Triangle::new(d, e, f),
        ];

        assert_eq!(expected_triangles.len(), triangles.len());

        expected_triangles.iter().for_each(|t| {
            let is_present = triangles.iter().any(|triangle|  *triangle == *t);
            assert!(is_present);
        })
    }

}
