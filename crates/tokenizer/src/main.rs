use std::fs::File;
use std::io::Read;

use vaporetto::{Model, Predictor, Sentence};

fn main() {
    //let mut f = File::open("model.bin").unwrap();
    //let mut f = File::open("../../../_downloaded/vaporetto/bccwj-suw+unidic+tag/bccwj-suw+unidic+tag.model.zst").unwrap();
    let mut f = File::open("../../../_downloaded/vaporetto/jp-0.4.7-5.mod").unwrap();
    let mut model_data = vec![];
    f.read_to_end(&mut model_data).unwrap();
    let (model, _) = Model::read_slice(&model_data).unwrap();
    let predictor = Predictor::new(model, false).unwrap();

    let s = Sentence::from_raw("火星猫の生態").unwrap();
    let s = predictor.predict(s);

    println!("{:?}", s.to_tokenized_vec().unwrap());
}
