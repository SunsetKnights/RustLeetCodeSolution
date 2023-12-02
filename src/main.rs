pub mod smallest_infinite_set;
use smallest_infinite_set::*;

fn main() {
    let mut t = SmallestInfiniteSet::new();
    t.add_back(2);
    t.pop_smallest();
    t.pop_smallest();
    t.pop_smallest();
    t.add_back(1);
    t.pop_smallest();
    t.pop_smallest();
    t.pop_smallest();
}
