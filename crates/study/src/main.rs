mod vars;

fn main() {
    println!("main called!");
    vars::run();
    vars::sub_a::func_a();
    vars::sub_b::func_b();
}