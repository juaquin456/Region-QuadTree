use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, Rgba};

use primitives::BoundingBox;

use crate::region_qt::primitives::Point;

mod primitives;

#[derive(PartialEq)]
pub enum Color {
    Gray,
    Data(Rgba<u8>),
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
    Color::Data(img.get_pixel(coord.0, coord.1))
}

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
            if !self.children[i].is_none() {
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
}

pub struct RegionQt {
    root: Option<Box<RegionNodeQt>>,
}

impl RegionQt {
    /// Create a new region quadtree.
    pub fn new() -> Self {
        RegionQt { root: None }
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

        self.root = Some(Box::new(RegionNodeQt::new(
            Point::from((0, 0)),
            Point::from(dim),
        )));

        self.root.as_mut().unwrap().update(&img);
    }

    pub fn write(&self) {
        unimplemented!();
    }

    pub fn read(&mut self) {
        unimplemented!();
    }

    pub fn plot(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let img = ImageReader::open("img/Untitled.png")
            .expect("Can't open the file")
            .decode()
            .unwrap();
        get_color(&img, (0, 9));
    }
}
