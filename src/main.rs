mod parser;
use parser::DotsConfig;


fn main() {
    let fp = "/home/gabrielfruet/dev/rust/dots/sample_repo/dots.toml";
    let dotscfg = DotsConfig::from_fp(fp).unwrap();
    dbg!(dotscfg);

}
