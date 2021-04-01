use drawille::Canvas;

pub struct Cube {
    x: f64,
    y: f64,
    z: f64,
    nodes: [(f64, f64, f64); 8],
    edges: [(usize, usize); 12]
}

impl Cube {
    pub fn create(x: f64, y: f64, z: f64, size: f64) -> Cube{

        let half_size = size / 2.;
        let nodes = [ 
            (x - half_size, y - half_size, z - half_size),
            (x + half_size, y - half_size, z - half_size),
            (x + half_size, y + half_size, z - half_size),
            (x - half_size, y + half_size, z - half_size),
            (x - half_size, y - half_size, z + half_size),
            (x - half_size, y + half_size, z + half_size),
            (x + half_size, y + half_size, z + half_size),
            (x + half_size, y - half_size, z + half_size),
        ];

        let edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (0, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (4, 7),
            (7, 1),
            (2, 6),
            (5, 3)
        ];

        Cube {
            x,
            y,
            z,
            edges,
            nodes
        }
    }

    fn get_angle(a: (f64, f64), b: (f64, f64)) -> f64 {
        ((b.1 - a.1) / (b.0 - a.0)).atan()
    }

    fn cos(x: f64, y: f64) -> f64 {
        x.cos() * y.cos() - x.sin() * y.sin()
    }

    fn sin(x: f64, y: f64) -> f64 {
        x.sin() * y.cos() + x.cos() * y.sin()
    }

    fn get_size(a: (f64, f64), b: (f64, f64)) -> f64 {
        ((b.0 - a.0).powf(2.) + (a.1 - b.1).powf(2.)).sqrt()
    }

    pub fn rotate_x(mut self, theta: f64) -> Cube {
        for mut node in self.nodes.iter_mut() {
            let (_, y, z) = node;

            
            let angle = Cube::get_angle((*y, *z), (self.y, self.z));
            let size = Cube::get_size((self.y, self.z), (*y, *z));
            
            let factor = if *y > self.y  { 1. } else { -1. } ;

            let y_final = self.x + size * Cube::cos(angle, theta) * factor;
            let z_final = self.z + size * Cube::sin(angle, theta) * factor;


            node.1 = y_final;
            node.2 = z_final;
        }
        self
    }

    pub fn rotate_y(mut self, theta: f64) -> Cube {
        for mut node in self.nodes.iter_mut() {
            let (x, _, z) = node;


            let angle = Cube::get_angle((*x, *z), (self.x, self.z));
            let size = Cube::get_size((self.x, self.y), (*x, *z));
            
            let factor = if *x > self.x  { 1. } else { -1. } ;

            let x_final = self.x + size * Cube::cos(angle, theta) * factor;
            let z_final = self.z + size * Cube::sin(angle, theta) * factor;


            node.0 = x_final;
            node.2 = z_final;
        }

        self
    }

    pub fn rotate_z(mut self, theta: f64) -> Cube {
        for mut node in self.nodes.iter_mut() {
            let (x, y, _) = node;
            // node.0 = x * cos - y. * sin;
            let angle = Cube::get_angle((*x, *y), (self.x, self.y));
            let size = Cube::get_size((self.x, self.y), (*x, *y));
            
            let factor = if *x < self.x  { -1. } else { 1. } ;

            let x_final = self.x + size * Cube::cos(angle, theta) * factor;
            let y_final = self.y + size * Cube::sin(angle, theta) * factor;

            node.0 = x_final;
            node.1 = y_final;
        }

        self
    }


    pub fn draw(self, x: i32, y: i32, mut canvas: Canvas) -> Canvas {
        for edge in self.edges.iter() {
            let (n0, n1) = edge;

            let (x0, y0, _) = self.nodes[*n0];
            let (x1, y1, _) = self.nodes[*n1];

            let pos_x0 = (x0 as i32) + (x as i32);
            let pos_y0 = (y0 as i32) + (y as i32);
            let pos_x1 = (x1 as i32) + (x as i32);
            let pos_y1 = (y1 as i32) + (y as i32);

            canvas.line(pos_x0 as u32, pos_y0 as u32, pos_x1 as u32, pos_y1 as u32);
        }

        for node in self.nodes.iter() {
            let (x_cube, y_cube, _) = node;
            let pos_x = (*x_cube as i32) + (x as i32);
            let pos_y = (*y_cube as i32) + (y as i32);

            canvas.text(pos_x as u32, pos_y as u32, 1, "*");
        }

        canvas
    }
}

