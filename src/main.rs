pub mod lib;
use std::mem;

use lib::member::TrussMember;
use lib::structure::TrussStructure;

fn main() {
    let e_mod = 120.0;
    let area =  500.0;
    let member1 = TrussMember::new(e_mod, area, 1200.0, 180.0, vec![1, 2]);
    let member2 = TrussMember::new(e_mod, area, 2000.0, 233.13, vec![1, 3]);
    let member3 = TrussMember::new(e_mod, area, 1600.0, 270.0, vec![1, 4]);
    let mut system = TrussStructure::new(vec![member1,member2,member3], vec![1,2,3,4]);
    let expanded = system.global_stiffness_matrix();
    expanded.print()

}
