use tensorflow::{Graph, ImportGraphDefOptions, Tensor, Session, SessionOptions, SessionRunArgs};

use crate::MODEL;
use image;

pub fn infer(image_data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    tracing::info!("received request");
    let model = MODEL.as_ref();
    let mut graph = Graph::new();
    graph.import_graph_def(&*model, &ImportGraphDefOptions::new())?;
    tracing::info!("built graph");

    let img = image::load_from_memory(image_data)?;
    let resized = image::imageops::thumbnail(&img, 224, 224);
    let mut flattened = Vec::new();
    for rgb in resized.pixels() {
        flattened.push(rgb[0] as f32 / 255.);
        flattened.push(rgb[1] as f32 / 255.);
        flattened.push(rgb[2] as f32 / 255.); 
    }
    let input = Tensor::new(&[1, 224, 224, 3])
        .with_values(&flattened)
        .unwrap();
    tracing::info!("built input");

    let session = Session::new(&SessionOptions::new(), &graph)?;
    let mut args = SessionRunArgs::new();
    args.add_feed(&graph.operation_by_name_required("input").unwrap(), 0, &input);
    let prediction = args.request_fetch(&graph.operation_by_name_required("MobilenetV2/Predictions/Softmax")?, 0);
    session.run(&mut args).unwrap();
    let prediction_res: Tensor<f32> = args.fetch(prediction)?;
    let mut i = 0;
    let mut message = String::new();
    while i < prediction_res.len() {
        message += &prediction_res[i].to_string().to_owned();
        i += 1;
    }
    tracing::info!("finished session");
    return Ok(message);
}
