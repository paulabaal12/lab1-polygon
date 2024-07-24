use crate::framebuffer::Framebuffer;
use crate::line::Line;
use std::cmp;

pub trait Polygon {
    fn polygon(&mut self, vertices: &[(usize, usize)]);
    fn fill_polygon(&mut self, vertices: &[(usize, usize)]);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, vertices: &[(usize, usize)]) {
        for i in 0..vertices.len() {
            let (x0, y0) = vertices[i];
            let (x1, y1) = vertices[(i + 1) % vertices.len()];
            self.line(x0, y0, x1, y1);
        }
    }

    fn fill_polygon(&mut self, vertices: &[(usize, usize)]) {
        if vertices.is_empty() {
            return;
        }

        let mut min_y = vertices[0].1;
        let mut max_y = vertices[0].1;

        for &(_, y) in vertices.iter().skip(1) {
            min_y = cmp::min(min_y, y);
            max_y = cmp::max(max_y, y);
        }

        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            for i in 0..vertices.len() {
                let (x0, y0) = vertices[i];
                let (x1, y1) = vertices[(i + 1) % vertices.len()];

                if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                    let x = x0 as f32 + (y as i32 - y0 as i32) as f32 * (x1 as i32 - x0 as i32) as f32 / (y1 as i32 - y0 as i32) as f32;
                    intersections.push(x as usize);
                }
            }

            intersections.sort_unstable();

            for chunk in intersections.chunks(2) {
                if chunk.len() == 2 {
                    let start = chunk[0];
                    let end = chunk[1];
                    for x in start..=end {
                        self.point(x, y);
                    }
                }
            }
        }
    }
}