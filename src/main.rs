use std::{
    env,
    error::Error,
    io::{stdout, Write},
    thread,
};

use image::{DynamicImage, ImageBuffer};

mod edge_detection;
use edge_detection::*;
use nokhwa::{
    pixel_format::{self, RgbFormat},
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};
use show_image::{event::WindowEvent, exit};
#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    const DEFAULT_SCALE: u32 = 8;
    const DEFAULT_THRESHOLD: f32 = 0.0; //0.005;
    const DEFAULT_SIGMA: f32 = 2.0;
    const DEFAULT_COLOUR: [f32; 3] = [0.0, 0.0, 0.0];

    let mut args = env::args().skip(1);
    let infile = args.next().expect("infile should not be empty");
    let outfile = args.next().unwrap_or("out.png".to_string());
    let scale = args
        .next()
        .map_or(None, |str| str.parse().ok())
        .unwrap_or(DEFAULT_SCALE);
    let threshold = args
        .next()
        .map_or(None, |str| str.parse().ok())
        .unwrap_or(DEFAULT_THRESHOLD);
    let sigma = args
        .next()
        .map_or(None, |str| str.parse().ok())
        .unwrap_or(DEFAULT_SIGMA);
    let random_colour = args
        .next()
        .map_or(None, |str| str.parse().ok())
        .unwrap_or(false);

    let colour = if random_colour {
        DEFAULT_COLOUR
    } else {
        [
            args.next()
                .map_or(None, |str| str.parse().ok())
                .unwrap_or(DEFAULT_COLOUR[0]),
            args.next()
                .map_or(None, |str| str.parse().ok())
                .unwrap_or(DEFAULT_COLOUR[1]),
            args.next()
                .map_or(None, |str| str.parse().ok())
                .unwrap_or(DEFAULT_COLOUR[2]),
        ]
    };

    if infile != "camera" {
        print!("Loading source image \"{}\": ...", infile);
        stdout().flush()?;
        let in_img = image::io::Reader::open(infile)?.decode()?;
        println!(" done.");
        let out_img = edge_detect(in_img, scale, sigma, threshold, colour, random_colour)?;
        print!("Saving \"{}\": ...", outfile);
        stdout().flush()?;
        DynamicImage::ImageRgb32F(out_img)
            .into_rgb8()
            .save(outfile)?;
        println!(" done.");
    } else {
        print!("Loading camera \"{}\": ...", infile);
        stdout().flush()?;
        let mut camera = Camera::new(
            CameraIndex::Index(0),
            // RequestedFormat::new::<pixel_format::RgbFormat>(RequestedFormatType::Closest(
            //     CameraFormat::new(
            //         Resolution::new(1280, 720),
            //         nokhwa::utils::FrameFormat::RAWRGB,
            //         15,
            //     ),
            // )),
            RequestedFormat::new::<pixel_format::RgbFormat>(RequestedFormatType::None),
        )?;
        // let width = camera.resolution().width_x;
        // let height = camera.resolution().height_y;
        println!(" done.");

        let display_window = show_image::create_window("Image", Default::default())?;
        let events = display_window.event_channel().unwrap();
        thread::spawn(|| {
            for event in events {
                match event {
                    WindowEvent::CloseRequested(_) => exit(0),
                    WindowEvent::Destroyed(_) => exit(0),
                    _ => (),
                };
            }
        });

        let mut frame_num = 0;
        loop {
            println!(
                "Preparing to generate new frame ({}):\nSTANDARD DEVIATION: {}, BACKGROUND COLOUR: {:?}",
                frame_num, sigma, colour
            );
            frame_num += 1;
            print!("Getting image from camera: ...");
            stdout().flush()?;
            let frame = camera.frame()?;
            let width = frame.resolution().width_x;
            let height = frame.resolution().height_y;
            // dbg!(width, height, width * height * 3, frame.buffer().len());
            let mut buffer = vec![0u8; (width * height * 3) as usize];
            frame.decode_image_to_buffer::<RgbFormat>(buffer.as_mut_slice())?;
            let in_img: ImageBuffer<image::Rgb<u8>, _> =
                ImageBuffer::from_raw(width, height, buffer)
                    .expect("frame.buffer() should be a valid image with size `width` by `height`");
            println!(" done.");

            let out_img = DynamicImage::ImageRgb32F(edge_detect(
                DynamicImage::ImageRgb8(in_img),
                scale,
                sigma,
                threshold,
                colour,
                random_colour,
            )?)
            .to_rgb8();
            print!("Outputing to window: ...");
            stdout().flush()?;
            display_window.set_image("output", out_img)?;
            println!(" done.");
        }
    };

    Ok(())
}
