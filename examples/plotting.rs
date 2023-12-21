use region_quadtree::region_qt;

fn main() {
    let mut tree = region_qt::RegionQt::new();
    tree.build("img/test3.png");
    tree.plot();
}