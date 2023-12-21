# Region Quadtree
This is a lightweight implementation of a Region Quadtree in Rust. It is a static structure to represent an image.
## Usage

```rust
use region_quadtree::region_qt::RegionQt;

fn main() {
    let mut tree = RegionQt::new();
    
    // build a region quadtree with dimensions and data given the image path
    tree.build("img/GH.png");
    
    // plot the original and draw each line that divide a quadrant
    tree.plot();
    
    // save to file a bincode encode of the tree struct
    tree.write("GH_qt.bin");
}
```

You can use the ```from_file``` method to read the *bincode* file
```rust
let mut tree = RegionQt::from_file("GH_qt.bin");
```

### Plot
![](img/qt_GH.png)
