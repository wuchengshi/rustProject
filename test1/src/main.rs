mod sed;


fn main() {
    println!("a~Z:");
    for c in ('Z'..='a').rev() {
        println!("{}", c);
    }

    println!("A~z:");
    sed::my_mod::my_print();
}
