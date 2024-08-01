use bencher::Bencher;
use bfunc::Bfunc;
use utils::decompress_default;

mod bencher;
mod bfunc;
mod comp;
mod decomp;
mod graph;
mod utils;

const COMP_1: &[u8] = include_bytes!("../files/1.rnote");
const COMP_2: &[u8] = include_bytes!("../files/2.rnote");
const COMP_3: &[u8] = include_bytes!("../files/3.rnote");
const COMP_4: &[u8] = include_bytes!("../files/4.rnote");
const COMP_5: &[u8] = include_bytes!("../files/5.rnote");
const COMP_6: &[u8] = include_bytes!("../files/6.rnote");
const COMP_7: &[u8] = include_bytes!("../files/7.rnote");
const COMP_8: &[u8] = include_bytes!("../files/8.rnote");
const COMP_9: &[u8] = include_bytes!("../files/9.rnote");
const COMP_10: &[u8] = include_bytes!("../files/10.rnote");

fn main() {
    let decomp_1 = decompress_default(COMP_1);
    let decomp_2 = decompress_default(COMP_2);
    let decomp_3 = decompress_default(COMP_3);
    let decomp_4 = decompress_default(COMP_4);
    let decomp_5 = decompress_default(COMP_5);
    let decomp_6 = decompress_default(COMP_6);
    let decomp_7 = decompress_default(COMP_7);
    let decomp_8 = decompress_default(COMP_8);
    let decomp_9 = decompress_default(COMP_9);
    let decomp_10 = decompress_default(COMP_10);

    let bencher = Bencher::new(
        vec![
            Bfunc::new("gzip-4", comp::gzip(4), decomp::gzip),
            Bfunc::new("gzip-7", comp::gzip(7), decomp::gzip),
        ],
        vec![
            &decomp_1, &decomp_2, &decomp_3, &decomp_4, &decomp_5, &decomp_6, &decomp_7, &decomp_8,
            &decomp_9, &decomp_10,
        ],
    );

    bencher.run(3);
}