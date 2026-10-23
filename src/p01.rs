use std::collections::BTreeSet;

pub fn executar() {
    let universo = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5, 6]);
    let c = BTreeSet::from([2, 4, 6, 8]);

    let uniao_a_b: BTreeSet<_> = a.union(&b).copied().collect();
    let intersec_a_b: BTreeSet<_> = a.intersection(&b).copied().collect();
    let intersec_a_c: BTreeSet<_> = a.intersection(&c).copied().collect();
    let complemento_a: BTreeSet<_> = universo.difference(&a).copied().collect();

    println!("U = {:?}", universo);
    println!("A = {:?}", a);
    println!("B = {:?}", b);
    println!("C = {:?}", c);

    println!("\nA ∪ B = {:?}", uniao_a_b);
    println!("A ∩ B = {:?}", intersec_a_b);
    println!("A ∩ C = {:?}", intersec_a_c);
    println!("Aᶜ = {:?}", complemento_a);
}
