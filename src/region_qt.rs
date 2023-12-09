use std::rc::Rc;
use image::GenericImageView;
use image::io::Reader as ImageReader;

use crate::primitives::BoundingBox;


struct RegionNodeQt {
    bounding: BoundingBox,
    children: [Option<Rc<RegionNodeQt>>; 4]
}

pub struct RegionQt {
    root: Option<Rc<RegionNodeQt>>
}

impl RegionQt {
    fn new() -> Self {
        RegionQt {
            root: None
        }
    }

    fn from(path: &str) -> Self {
        let img = ImageReader::open(path)
            .expect("Can't open the file")
            .decode().unwrap();
        let (width, height) = img.dimensions();
    }
}