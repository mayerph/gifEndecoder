use base64::decode as dec;
use image::codecs::gif::Repeat::Infinite;
use image::codecs::gif::{GifDecoder, GifEncoder};
use image::load_from_memory_with_format;
use image::Delay as IDelay;
use image::Frame as IFrame;
use image::ImageFormat;
use image::{open, AnimationDecoder};
use neon::prelude::*;
use neon::register_module;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;

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

// arguments for the encode async task
struct EncodeAsyncTask {
    filename: String,
    gif: GifTemplate,
    infinite: bool,
    speed: i32,
}

impl Task for EncodeAsyncTask {
    // The task's result type, which is sent back to the main thread to communicate a successful result back to JavaScript.
    type Output = String;
    // The task's error type, which is sent back to the main thread to communicate a task failure back to JavaScript.
    type Error = String;
    // The type of JavaScript value that gets produced to the asynchronous callback on the main thread after the task is completed.
    type JsEvent = JsString;
    // Perform the task, producing either a successful Output or an unsuccessful Error. This method is executed in a background thread as part of libuv's built-in thread pool.
    fn perform(&self) -> Result<Self::Output, Self::Error> {
        // file creation (gif)
        let file_in = match File::create(self.filename.clone()) {
            Ok(v) => v,
            Err(_) => panic!("an error occurred during file write."),
        };

        let mut encoder = GifEncoder::new_with_speed(file_in, self.speed);

        // should the gif repeated
        if self.infinite == true {
            match encoder.set_repeat(Infinite) {
                Ok(v) => v,
                Err(_) => panic!("an error occurred during encoder configuration"),
            }
        };
        for (i, custom_frame) in self.gif.frames.iter().enumerate() {
            let frame_file_in = match open(&custom_frame.file) {
                Ok(v) => v,
                Err(_) => panic!("an error occurred during file read."),
            };
            println!("iteration {}", i);
            let frame_rgb_image = frame_file_in.into_rgba8();

            let frame_delay = IDelay::from_numer_denom_ms(
                custom_frame.delay.numerator,
                custom_frame.delay.denominator,
            );
            let frame = IFrame::from_parts(
                frame_rgb_image,
                custom_frame.left,
                custom_frame.top,
                frame_delay,
            );

            match encoder.encode_frame(frame) {
                Ok(v) => v,
                Err(_) => panic!("an error occurred while encoding a frame"),
            }
        }

        return Ok(self.filename.clone());
    }
    // Convert the result of the task to a JavaScript value to be passed to the asynchronous callback. This method is executed on the main thread at some point after the background task is completed.
    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        Ok(cx.string(result.unwrap()))
    }
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

fn encode(mut cx: FunctionContext) -> JsResult<JsUndefined> {
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
        Err(_) => panic!("the third argument has to be of type boolean."),
    };

    // 4th argument, speed of encoding
    let speed = match cx.argument::<JsNumber>(3) {
        Ok(v) => v.value(),
        Err(_) => panic!("the 4th argument has to be of type number."),
    };

    // 5th argument, is the callback
    let cb = match cx.argument::<JsFunction>(4) {
        Ok(v) => v,
        Err(_) => panic!("the 5th argument has to be a function."),
    };
    let task = EncodeAsyncTask {
        filename: filename,
        gif: gif,
        infinite: infinite,
        speed: speed as i32,
    };
    task.schedule(cb);
    Ok(cx.undefined())
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

    let speed = match cx.argument::<JsNumber>(3) {
        Ok(v) => v.value(),
        Err(_) => panic!("the 4th argument has to be of type number."),
    };

    let file_in = match File::create(filename) {
        Ok(v) => v,
        Err(_) => panic!("an error occurred during file write."),
    };

    //let mut encoder = GifEncoder::new(file_in);
    let mut encoder = GifEncoder::new_with_speed(file_in, speed as i32);
    if infinite == true {
        match encoder.set_repeat(Infinite) {
            Ok(v) => v,
            Err(_) => panic!("an error occurred during encoder configuration"),
        }
    };
    let mut frames: Vec<IFrame> = Vec::new();
    for custom_frame in gif.frames.iter() {
        let frame_file_in = match dec(&custom_frame.file) {
            Ok(v) => v,
            Err(_) => panic!("an error occurred during uri decoding (1)"),
        };
        let frame_rgb_image = match load_from_memory_with_format(&frame_file_in, ImageFormat::Png) {
            Ok(v) => v.into_rgba8(),
            Err(_) => panic!("an error occurred during uri decoding (2)"),
        };
        let frame_delay = IDelay::from_numer_denom_ms(
            custom_frame.delay.numerator,
            custom_frame.delay.denominator,
        );
        let frame = IFrame::from_parts(
            frame_rgb_image,
            custom_frame.left,
            custom_frame.top,
            frame_delay,
        );

        frames.push(frame);
    }

    match encoder.encode_frames(frames) {
        Ok(v) => v,
        Err(_) => panic!("an error occurred while encoding frames"),
    }
    Ok(cx.string(""))
}

register_module!(mut cx, {
    #[allow(unused_must_use)]
    {
        cx.export_function("decode", decode);
        cx.export_function("encode", encode);
        cx.export_function("ecode_with_uri", encode_with_uri);
    }
    Ok(())
});
