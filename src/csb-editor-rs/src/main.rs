mod extractor;
use extractor::extract_csb;

fn main() {
    extract_csb(&std::env::args().nth(1).unwrap().into_boxed_str());
}