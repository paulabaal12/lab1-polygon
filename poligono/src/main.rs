mod framebuffer;
mod line;
mod polygon;
mod bmp;

use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.clear();

    // Polígono 1
    let poly1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ];
    framebuffer.set_current_color(0xFFFF00); // Amarillo
    framebuffer.fill_polygon(&poly1);
    framebuffer.set_current_color(0xFFFFFF); // Blanco
    framebuffer.polygon(&poly1);

    // Polígono 2
    let poly2 = vec![(321, 335), (288, 286), (339, 251), (374, 302)];
    framebuffer.set_current_color(0x0000FF); // Azul
    framebuffer.fill_polygon(&poly2);
    framebuffer.set_current_color(0xFFFFFF); // Blanco
    framebuffer.polygon(&poly2);

    framebuffer.write_bmp("out.bmp").unwrap();
    println!("Image saved as out.bmp");
}