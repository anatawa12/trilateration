use crate::point::Point;

pub(crate) struct Builder {
    lines: Vec<(Point, Point)>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { lines: vec![] }
    }

    pub fn xy(&mut self, x: i32, y: i32) -> Point {
        Point(x as f64, y as f64)
    }

    pub fn line(&mut self, p1: Point, p2: Point) {
        self.lines.push((p1, p2))
    }

    pub fn lplp(&mut self, l1: i32, p1: Point, l2: i32, p2: Point) -> Point {
        let l1 = l1 as f64;
        let l2 = l2 as f64;
        let alpha = f64::atan((p2.1 - p1.1) / (p2.0 - p1.0));
        let dist = (p2.0 - p1.0) / f64::cos(alpha);
        let s = (l1 + l2 + dist) / 2.0;
        let tmp_x = (l1 * l1 - l2 * l2 + dist * dist) / (dist * 2.0);
        let tmp_y = 2.0 * f64::sqrt(s * (s - l1) * (s - l2) * (s - dist)) / dist;
        let point = Point(
            tmp_x * f64::cos(alpha) - tmp_y * f64::sin(alpha) + p1.0,
            tmp_x * f64::sin(alpha) + tmp_y * f64::cos(alpha) + p1.1,
        );
        self.lines.push((p1, point));
        self.lines.push((p2, point));
        point
    }

    pub fn print(&self, base: Point, width: u32, height: u32, scale: f64) {
        let mut path = String::new();
        for (a, b) in &self.lines {
            let a = (base + *a) / scale;
            let b = (base + *b) / scale;
            path.push_str(&format!(
                "M {} {} L {} {}",
                a.0,
                height as f64 - a.1,
                b.0,
                height as f64 - b.1
            ))
        }
        println!(
            r##"<svg viewBox="0 0 {w} {h}" width="{w}" height="{h}" xmlns="http://www.w3.org/2000/svg">"##,
            w = width,
            h = height,
        );
        println!(r##"<path stroke="blue" stroke-width="1" d="{}" />"##, path);
        println!("</svg>");
    }
}
