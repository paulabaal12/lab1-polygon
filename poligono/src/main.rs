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

    // Polígono 3
    let poly3 = vec![(377, 249), (411, 197), (436, 249)];
    framebuffer.set_current_color(0xFF0000); // Rojo
    framebuffer.fill_polygon(&poly3);
    framebuffer.set_current_color(0xFFFFFF); // Blanco
    framebuffer.polygon(&poly3);

    // Polígono 4
    let poly4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
        (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180)
    ];
    framebuffer.set_current_color(0x00FF00); // Verde
    framebuffer.fill_polygon(&poly4);
    framebuffer.set_current_color(0xFFFFFF); // Blanco
    framebuffer.polygon(&poly4);

    // Polígono 5 (agujero)
    let poly5 = vec![(682, 175), (708, 120), (735, 148), (739, 170)];
    framebuffer.set_current_color(0x000000); // Negro (agujero)
    framebuffer.fill_polygon(&poly5);
    framebuffer.set_current_color(0xFFFFFF); // Blanco
    framebuffer.polygon(&poly5);

    framebuffer.write_bmp("out.bmp").unwrap();
    println!("Image saved as out.bmp");
}