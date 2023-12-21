use std::default::Default;
use std::fs::File;
use std::io::{Read, Write};
use std::thread;

use image::{DynamicImage, GenericImageView};
use image::io::Reader as ImageReader;
use piston_window::{Button, clear, Events, EventSettings, G2dTexture, Line, line, MouseButton, MouseCursorEvent, PistonWindow, PressEvent, RenderEvent, Texture, TextureSettings, WindowSettings};
use piston_window::color::RED;
use piston_window::types::Radius;
use serde::{Deserialize, Serialize};
use serde_pickle::{DeOptions, SerOptions};

use primitives::BoundingBox;

use crate::region_qt::primitives::Point;

mod primitives;

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Color {
    Gray,
    Data([u8; 4]),
}

/// Extract the color of a pixel from an image.
///
/// # Arguments
///
/// * `img` - The image from which the color is extracted.
/// * `coord` - The coordinates of the pixel.
///
/// # Return
///
/// The color of the pixel.
fn get_color(img: &DynamicImage, coord: (u32, u32)) -> Color {
    Color::Data(img.get_pixel(coord.0, coord.1).0)
}

#[derive(Serialize, Deserialize)]
struct RegionNodeQt {
    data: Color,
    bounding: BoundingBox,
    children: [Option<Box<RegionNodeQt>>; 4],
}

impl RegionNodeQt {
    /// Create a new region quadtree node.
    ///
    /// # Arguments
    ///
    /// * `min` - The bottom-left corner of the bounding box.
    /// * `max` - The top-right corner of the bounding box.
    ///
    /// # Return
    ///
    /// A new region quadtree node.
    fn new(min: Point, max: Point) -> Self {
        RegionNodeQt {
            data: Color::Gray,
            bounding: BoundingBox::new(min, max),
            children: [None, None, None, None],
        }
    }

    /// Initialize the children of the node.
    ///
    /// # Note
    ///
    /// This function is called only if the node is a leaf.
    fn initialize_children(&mut self) {
        let center = self.bounding.center();
        self.children[0] = Some(Box::new(RegionNodeQt::new(
            Point::from((self.bounding.min().x, center.y)),
            Point::from((center.x, self.bounding.max().y)),
        )));

        self.children[1] = Some(Box::new(RegionNodeQt::new(center, *self.bounding.max())));

        self.children[2] = Some(Box::new(RegionNodeQt::new(*self.bounding.min(), center)));

        self.children[3] = Some(Box::new(RegionNodeQt::new(
            Point::from((center.x, self.bounding.min().y)),
            Point::from((self.bounding.max().x, center.y)),
        )))
    }

    /// Check if the node is a leaf.
    ///
    /// # Return
    ///
    /// `true` if the node is a leaf, `false` otherwise.
    fn is_leaf(&self) -> bool {
        for i in 0..4 {
            if self.children[i].is_some() {
                return false;
            }
        }
        true
    }

    /// Calculate the color of the node if exists only one color in the bounding box. Otherwise, the color is `Color::Gray`.
    ///
    /// # Arguments
    ///
    /// * `img` - The image from which the color is extracted.
    ///
    /// # Return
    ///
    /// The color of the node.
    fn calculate_color(&self, img: &DynamicImage) -> Color {
        let xl = self.bounding.min().x;
        let yl = self.bounding.min().y;

        let current_color = get_color(img, (xl, yl));

        for x in xl..self.bounding.max().x {
            for y in self.bounding.min().y..self.bounding.max().y {
                let next_color = get_color(img, (x, y));
                if next_color != current_color {
                    return Color::Gray;
                }
            }
        }
        current_color
    }

    /// Update the color of the node.
    ///
    /// # Arguments
    ///
    /// * `img` - The image from which the color is extracted.
    ///
    /// # Note
    ///
    /// This function is called recursively.
    fn update(&mut self, img: &DynamicImage) {
        let color = self.calculate_color(img);
        match color {
            Color::Gray => {
                if self.is_leaf() {
                    self.initialize_children();
                    for i in 0..4 {
                        self.children[i].as_mut().unwrap().update(img);
                    }
                }
            }
            _ => {
                self.data = color;
            }
        }
    }

    fn lines(&self, lines: &mut Vec<[Point; 2]>) {
        if self.is_leaf() {
            return
        }

        let center = self.bounding.center();

        lines.push([
            Point::from((center.x, self.bounding.min().y)),
            Point::from((center.x, self.bounding.max().y)),
        ]);
        lines.push([
            Point::from((self.bounding.min().x, center.y)),
            Point::from((self.bounding.max().x, center.y)),
        ]);

        for child in self.children.iter().flatten() {
            child.lines(lines);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RegionQt {
    root: Option<Box<RegionNodeQt>>,
    width: u32,
    height: u32,
}

impl RegionQt {
    /// Create a new region quadtree.
    pub fn new() -> Self {
        RegionQt { root: None , width: 0, height: 0}
    }

    /// Build the region quadtree.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the image.
    ///
    /// # Note
    ///
    /// This function is called only once.
    ///
    /// # Example
    ///
    /// ```
    /// let mut tree = region_quadtree::RegionQt::new();
    /// tree.build("src/Untitled.png");
    /// ```
    pub fn build(&mut self, path: &str) {
        let img = ImageReader::open(path)
            .expect("Can't open the file")
            .decode()
            .unwrap();
        let dim = img.dimensions();
        (self.width, self.height) = dim;

        self.root = Some(Box::new(RegionNodeQt::new(
            Point::from((0, 0)),
            Point::from(dim),
        )));

        self.root.as_mut().unwrap().update(&img);
    }

    pub fn write(&self, name: &str) {
        let mut file = File::create(name).unwrap();
        file.write_all(serde_pickle::to_vec(self, SerOptions::default()).unwrap().as_slice()).unwrap()
    }

    pub fn from_file(name: &str) -> Self {
        let mut file = File::open(name).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let new_obj: RegionQt = serde_pickle::from_slice(data.as_slice(), DeOptions::default()).unwrap();
        new_obj
    }

    pub fn plot(&self) {
        if let Some("main") = thread::current().name() {
            let mut window: PistonWindow = WindowSettings::new("Dibujo", [self.width, self.height])
                .exit_on_esc(true)
                .build()
                .unwrap();

            let mut lines: Vec<[Point; 2]> = Vec::new();
            self.get_lines(&mut lines);

            let image = image::open("src/img/test3.png").unwrap();

            let texture: G2dTexture = Texture::from_image(
                &mut window.create_texture_context(),
                &image.to_rgba8(),
                &TextureSettings::new(),
            )
                .unwrap();

            while let Some(e) = window.next() {
                window.draw_2d(&e, |c, g, _| {
                    clear([1.0; 4], g);

                    // Dibuja la imagen
                    piston_window::image(
                        &texture,
                        c.transform,
                        g,
                    );
                    //
                    // // Dibuja los trazos
                    for l in &lines {
                        let line_slice = [l[0].x as f64, l[0].y as f64, l[1].x as f64 , l[1].y as f64];
                        // println!("{:?}", line_slice);
                        line([0.0, 0.0, 0.0, 1.0], 0.5, line_slice, c.transform, g);
                    }
                    println!("complete");
                    //
                    // // Dibuja el trazo actual
                    // if current_line.len() > 1 {
                    //     for i in 0..current_line.len() - 1 {
                    //         line([0.0, 0.0, 0.0, 1.0], 1.0, [current_line[i][0], current_line[i][1], current_line[i + 1][0], current_line[i + 1][1]], c.transform, g);
                    //     }
                    // }
                });

            }

        }
    }
    fn get_lines(&self, lines: &mut Vec<[Point; 2]>) {
        self.root.as_ref().unwrap().lines(lines);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let img = ImageReader::open("src/img/Untitled.png")
            .expect("Can't open the file")
            .decode()
            .unwrap();
        get_color(&img, (0, 9));
    }
}
