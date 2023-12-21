use region_quadtree::region_qt;

fn main() {
    let mut tree = region_qt::RegionQt::new();
    tree.build("img/GH.png");
    tree.plot();
    tree.write("test.bin");
}