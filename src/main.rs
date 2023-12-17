pub mod range_module;
use range_module::*;

fn main() {
    let mut range = RangeModule::new();
    range.add_range(10, 20);
    range.remove_range(14, 16);
    range.query_range(10, 14);
    range.query_range(13, 15);
    range.query_range(16, 17);
}
