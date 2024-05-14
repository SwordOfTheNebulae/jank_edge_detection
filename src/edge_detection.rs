use std::{
    error::Error,
    io::{stdout, Write},
};

use image::{
    imageops::{self, FilterType::Triangle},
    DynamicImage, ImageBuffer, Luma, Pixel, Rgb32FImage,
};
use rand::{thread_rng, Rng};

pub fn edge_detect(
    in_img: DynamicImage,
    scale: u32,
    sigma: f32,
    threshold: f32,
    colour: [f32; 3],
    use_random_colour: bool,
) -> Result<ImageBuffer<image::Rgb<f32>, Vec<f32>>, Box<dyn Error>> {
    let (width, height) = (in_img.height() / scale, in_img.width() / scale);

    print!("Resizing: ...");
    stdout().flush()?;
    let mut out_img = in_img.resize(width, height, Triangle).into_rgb32f();
    println!(" done.");
    // out_img = image::imageops::filter3x3::<Rgb32FImage, Rgb<f32>, f32>(
    //     &out_img,
    //     &[0f32, 1.0, 0.0, 1.0, -4., 1.0, 0.0, 1.0, 0.0],
    // );

    print!("Bluring: ...");
    stdout().flush()?;
    let blured = imageops::blur(&out_img, sigma);
    println!(" done.");

    print!("Performing laplace transform: ...");
    stdout().flush()?;
    let out_luma = filter3x3_no_clamp(&blured, &[0f32, 1.0, 0.0, 1.0, -4., 1.0, 0.0, 1.0, 0.0]);
    println!(" done.");

    print!("Finding zero crossings: ...");
    stdout().flush()?;
    // let temp = Rgb32FImage::from_vec(width, height, out_vec).unwrap();
    // let temp: ImageBuffer<Luma<f32>, Vec<f32>> =
    //     ImageBuffer::from_raw(width, height, out_vec.into_iter().step_by(3).collect()).unwrap();

    out_img.pixels_mut().zip(0u32..).for_each(|(px, i)| {
        let x = i % width;
        let y = i / width;
        if !is_zero_crossing(&out_luma, x, y, threshold) {
            px.0 = if use_random_colour {
                thread_rng().gen()
            } else {
                colour
            }
        }
    });
    println!(" done.");

    // print!("Thresholding: ...");
    // stdout().flush()?;
    // for (px, i) in out_vec.chunks(3).zip(0..) {
    //     let x = i % width;
    //     let y = i / width;
    //     // out_img.put_pixel(
    //     //     x,
    //     //     y,
    //     //     Rgb(if ((px[0] + px[1] + px[2]) / 3.0).abs() < 0.01 {
    //     //         [1f32; 3]
    //     //     } else {
    //     //         [0f32; 3]
    //     //     }),
    //     // )
    //     if ((px[0] + px[1] + px[2]) / 3.0).abs() < threshold {
    //         out_img.put_pixel(x, y, Rgb(rand::thread_rng().gen()))
    //     }
    // }
    // println!(" done.");

    Ok(out_img)
}

// copied and modified from `image::imageops::sample::filter3x3()`
fn filter3x3_no_clamp(image: &Rgb32FImage, kernel: &[f32]) -> ImageBuffer<Luma<f32>, Vec<f32>> {
    let taps: &[(isize, isize)] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let (width, height) = image.dimensions();

    // let mut out: Vec<f32> = vec![0.0; (width * height) as usize * 3];
    let mut out = ImageBuffer::new(width, height);

    let sum = match kernel.iter().fold(0.0, |s, &item| s + item) {
        x if x == 0.0 => 1.0,
        sum => sum,
    };
    let sum = (sum, sum, sum, sum);

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut t = (0.0, 0.0, 0.0);
            for (&k, &(a, b)) in kernel.iter().zip(taps.iter()) {
                let x0 = x as isize + a;
                let y0 = y as isize + b;

                let p = image.get_pixel(x0 as u32, y0 as u32);

                #[allow(deprecated)]
                let vec = p.channels4();

                t.0 += vec.0 * k;
                t.1 += vec.1 * k;
                t.2 += vec.2 * k;
            }

            let (t1, t2, t3) = (t.0 / sum.0, t.1 / sum.1, t.2 / sum.2);

            // #[allow(deprecated)]
            // let t = Pixel::from_channels(
            //     NumCast::from(clamp(t1, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t2, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t3, 0.0, max)).unwrap(),
            //     NumCast::from(clamp(t4, 0.0, max)).unwrap(),
            // );
            // let t = [t1, t2, t3];
            // let pos = (y as usize * width as usize + x as usize) * 3usize;
            // out[pos] = t1;
            // out[pos + 1] = t2;
            // out[pos + 2] = t3;
            let t = (t1 + t2 + t3) / 3.0;
            out.put_pixel(x, y, Luma([t]));
        }
    }

    out
}

fn is_zero_crossing(
    src: &ImageBuffer<Luma<f32>, Vec<f32>>,
    x: u32,
    y: u32,
    threshold: f32,
) -> bool {
    let (width, height) = src.dimensions();
    for (off_x, off_y) in [(1, 0), (1, 1), (-1, -1), (0, 1)] {
        if x as i32 + off_x < 0
            || x as i32 - off_x < 0
            || x as i32 + off_x >= width as i32
            || x as i32 - off_x >= width as i32
            || y as i32 + off_y < 0
            || y as i32 - off_y < 0
            || y as i32 + off_y >= height as i32
            || y as i32 - off_y >= height as i32
        {
            continue;
        }
        if src
            .get_pixel((x as i32 - off_x) as u32, (y as i32 - off_y) as u32)
            .0[0]
            * src
                .get_pixel((x as i32 + off_x) as u32, (y as i32 + off_y) as u32)
                .0[0]
            < 0.0
        {
            if src.get_pixel(x - 1, y).0[0] + src.get_pixel(x + 1, y).0[0] > threshold {
                return true;
            } else {
                return false;
            };
        }
    }
    false
}
