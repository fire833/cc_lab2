use playground::Playground;

mod instructions;
mod optimize;
mod playground;

fn main() {
    let pg: Playground;
    unsafe {
        pg = Playground::new(4096);
    }
    println!("running program now");

    let prog = vec![0xc3];
    pg.run(&prog, 10);
}
