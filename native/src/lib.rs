use base64::{decode as dec, encode as enc};
use image::codecs::gif::Repeat::{Finite, Infinite};
use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use image::io::Reader as ImageReader;
use image::load_from_memory_with_format;
use image::save_buffer_with_format;
use image::ColorType;
use image::Delay as IDelay;
use image::Frame as IFrame;
use image::{open, AnimationDecoder, ImageDecoder};
use image::{ImageFormat, ImageResult};
use neon::prelude::*;
use neon::register_module;
use neon_serde::export;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::sync::Arc;
use std::thread;
/**
 * struct representing the delay of a single frame
 */
#[derive(Serialize, Deserialize, Debug)]
struct Delay {
    numerator: u32,
    denominator: u32,
}

/**
 * struct representing a single frame
 */
#[derive(Serialize, Deserialize, Debug)]
struct Frame {
    delay: Delay,
    file: String,
    left: u32,
    top: u32,
}

/**
 * struct representing a gif
 */
#[derive(Serialize, Deserialize, Debug)]
struct GifTemplate {
    file: String,
    frames: Vec<Frame>,
}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

/**
 * decode a gif
 * returns an object with all the meta information of the gif file
 */
fn decode(mut cx: FunctionContext) -> JsResult<JsValue> {
    let source_file = match cx.argument::<JsString>(0) {
        Ok(v) => v.value(),
        Err(_) => panic!(
            "missing / wrong argument. The first argument has to be the filepath to the source of type string."
        ),
    };

    let destination = match cx.argument::<JsString>(1) {
        Ok(v) => v.value(),
        Err(_) => panic!(
            "missing / wrong argument. The first argument has to be the destination of type string."
        ),
    };
    let file_in = match File::open(&source_file) {
        Ok(v) => v,
        Err(_) => panic!("an error occurred during file read."),
    };

    let decoder = match GifDecoder::new(file_in) {
        Ok(v) => v,
        Err(_) => panic!("an error occurred during initialisation of the gif decoder"),
    };

    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding gif");
    let mut custom_frames: Vec<Frame> = Vec::new();
    for (i, frame) in frames.iter().enumerate() {
        let delay = frame.delay().numer_denom_ms();

        let buffer = frame.clone().into_buffer();
        let file = format!("{}/{}.png", destination, i);
        let custom_frame = Frame {
            file: file.clone(),
            delay: Delay {
                numerator: delay.0,
                denominator: delay.1,
            },
            left: frame.left(),
            top: frame.top(),
        };
        custom_frames.push(custom_frame);
        match buffer.save(file) {
            Ok(v) => v,
            Err(_) => panic!("an error occurred during file write operation"),
        };
    }

    let gif = GifTemplate {
        file: source_file,
        frames: custom_frames,
    };

    let js_value = neon_serde::to_value(&mut cx, &gif)?;

    Ok(js_value)
}

fn encode(mut cx: FunctionContext) -> JsResult<JsString> {
    // first argument
    let filename = match cx.argument::<JsString>(0) {
        Ok(v) => v.value(),
        Err(_) => panic!("The first argument has to be of type string"),
    };

    let error_msg =
        "missing / wrong argument. The first argument has to be the an object of type GifTemplate.";

    // second argument
    let gif_ = match cx.argument::<JsValue>(1) {
        Ok(v) => v,
        Err(_) => panic!(error_msg.clone()),
    };

    let gif: GifTemplate = match neon_serde::from_value(&mut cx, gif_) {
        Ok(v) => v,
        Err(_) => panic!(error_msg.clone()),
    };

    // third argument
    let infinite = match cx.argument::<JsBoolean>(2) {
        Ok(v) => v.value(),
        Err(_) => panic!("the third value has to be of type boolean."),
    };

    let file_in = match File::create(filename) {
        Ok(v) => v,
        Err(_) => panic!("an error occurred during file write."),
    };

    let mut frames: Vec<ImageResult<IFrame>> = Vec::new();
    for (i, custom_frame) in gif.frames.iter().enumerate() {
        let mut encoder = GifEncoder::new(file_in.try_clone().unwrap());
        if infinite == true {
            encoder.set_repeat(Infinite).unwrap();
        };
        println!("1-->{}", i);
        let frame_file_in = match open(&custom_frame.file) {
            Ok(v) => v,
            Err(_) => panic!("an error occurred during file read."),
        };

        println!("2-->{}", i);
        let frame_rgb_image = frame_file_in.into_rgba8();
        let copy = frame_rgb_image.clone();
        thread::spawn(move || {
            save_buffer_with_format(
                format!("./next{}.png", i),
                &copy,
                100,
                100,
                ColorType::Rgb8,
                ImageFormat::Png,
            );
        });

        println!("3-->{}", i);
        let frame_delay = IDelay::from_numer_denom_ms(
            custom_frame.delay.numerator,
            custom_frame.delay.denominator,
        );

        println!("4-->{}", i);
        let frame = IFrame::from_parts(
            frame_rgb_image,
            custom_frame.left,
            custom_frame.top,
            frame_delay,
        );
        //println!("hey");
        // frame lesen in einem separaten thread
        // warten bis alle fertig
        // encode methode aufrufen und zwar außerhalb
        //encoder.encode_frame(frame);
        //encoder.encode(data: &[u8], width: u32, height: u32, color: ColorType);

        println!("6-->{}", i);
    }
    println!("encodeframes_start");
    //let result = encoder.try_encode_frames(frames);
    println!("encodeframes_ende");
    Ok(cx.string(""))
}

fn encode_with_uri(mut cx: FunctionContext) -> JsResult<JsString> {
    // first argument
    let filename = match cx.argument::<JsString>(0) {
        Ok(v) => v.value(),
        Err(_) => panic!("The first argument has to be of type string"),
    };

    let error_msg =
        "missing / wrong argument. The first argument has to be the an object of type GifTemplate.";

    // second argument
    let gif_ = match cx.argument::<JsValue>(1) {
        Ok(v) => v,
        Err(_) => panic!(error_msg.clone()),
    };

    let gif: GifTemplate = match neon_serde::from_value(&mut cx, gif_) {
        Ok(v) => v,
        Err(_) => panic!(error_msg.clone()),
    };

    // third argument
    let infinite = match cx.argument::<JsBoolean>(2) {
        Ok(v) => v.value(),
        Err(_) => panic!("the third value has to be of type boolean."),
    };

    let file_in = match File::create(filename) {
        Ok(v) => v,
        Err(_) => panic!("an error occurred during file write."),
    };

    let mut encoder = GifEncoder::new(file_in);
    if infinite == true {
        encoder.set_repeat(Infinite).unwrap();
    };
    let mut frames: Vec<IFrame> = Vec::new();
    for custom_frame in gif.frames.iter() {
        println!("1-->");
        let frame_file_in = match dec(&custom_frame.file) {
            Ok(v) => v,
            Err(_) => panic!("an error occurred during uri decoding (1)"),
        };
        println!("2-->");
        let frame_rgb_image = match load_from_memory_with_format(&frame_file_in, ImageFormat::Png) {
            Ok(v) => v.into_rgba8(),
            Err(_) => panic!("an error occurred during uri decoding (2)"),
        };
        println!("3-->");
        let frame_delay = IDelay::from_numer_denom_ms(
            custom_frame.delay.numerator,
            custom_frame.delay.denominator,
        );
        println!("4-->");
        let frame = IFrame::from_parts(
            frame_rgb_image,
            custom_frame.left,
            custom_frame.top,
            frame_delay,
        );
        println!("5-->");

        frames.push(frame);
        println!("6-->");
    }

    let result = encoder.encode_frames(frames);
    Ok(cx.string(""))
}

fn test(mut cx: FunctionContext) -> JsResult<JsString> {
    let img = ImageReader::open("myimage.png");
    let my_gif = match img {
        Ok(gif) => {
            let was = gif.decode().unwrap();
            was.save("empty.jpg").unwrap();
        }
        Err(err) => panic!("Fehler"),
    };
    Ok(cx.string("hello node"))
}

register_module!(mut cx, {
    cx.export_function("hello", hello);
    cx.export_function("decode", decode);
    cx.export_function("encode", encode);
    cx.export_function("ecode_with_uri", encode_with_uri);
    Ok(())
});
