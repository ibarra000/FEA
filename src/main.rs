pub mod lib;

use lib::member::TrussMember;
use lib::structure::TrussStructure;

fn main() {
    let e_mod = 80.0;
    let area =  80.0;
    let member1 = TrussMember::create_double(e_mod, area, 4000.0, 120.0 ,180.0, vec![2, 3]);
    let member2 = TrussMember::create_double(e_mod, area, 3000.0, 270.0, 210.0,vec![1, 2]);
    let member3 = TrussMember::create_single(e_mod, area, 5000.0, 216.9, vec![1, 3]);
    let list = vec![member1 ,member2,member3];
    for mem in &list {
        mem.print();
    }
    let system = TrussStructure::new(list, vec![1,2,3]);
    let expanded = system.global_stiffness_matrix();
    expanded.print()

}
