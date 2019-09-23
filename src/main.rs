extern crate image;
extern crate ndarray;

use std::env;
use ndarray::prelude::*;

fn color(img: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>, x: u32, y:u32, rgb: char) -> u8{
    let mut c: usize = 0;
    if rgb == 'g' {
        c = 1;
    } else if rgb == 'b' {
        c = 2;
    }

    img.get_pixel(x, y)[c]
}

fn window(img: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>, x: u32, y: u32, rgb: char, kernel_size: usize) -> Array2<u8> {
    let kernel_size_i32: i32 = kernel_size as i32;
    let mut buf: Array2<u8> = Array::zeros((kernel_size, kernel_size));
    let range_array: Array1<i32> = Array1::from_iter(0..(kernel_size_i32));
    let index_array: Array1<usize> = Array1::from_iter(0..(kernel_size));

    for (xp, i) in range_array.clone().iter().zip(index_array.iter()) {
        for (yp, j) in range_array.clone().iter().zip(index_array.iter()) {
            // println!("{},{},{},{}", xp, yp, i, j);
            let _x: u32 = ((x as i32) + (xp - (kernel_size_i32/2))) as u32;
            let _y: u32 = ((y as i32) + (yp - (kernel_size_i32/2))) as u32;

            buf[(*i, *j)] = color(img, _x, _y, rgb);
        }
    }

    buf
}

fn convolution(img: &image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>, x: u32, y: u32, rgb: char, kernel: &Array2<f64>) -> u8 {
    let mut buf: Array2<u8>;
    let mut res_pix: f64 = 0.;

    buf = window(img, x, y, rgb, kernel.shape()[0]);
    for (c, k) in buf.iter().zip(kernel.iter()) {
        res_pix += (*c as f64) * k;
    }

    res_pix as u8

}

fn main() {
    let mut args = env::args();
    args.next(); // skip command name

    let img_path = match args.next() {
            None => {
                    eprintln!("Error: Input file path is not specified. ");
                    eprintln!("Usage: cargo run /path/to/input/image");
                    return;
            },
            Some(s) => s,
    };

    // Load Image
    let img = image::open(&img_path).unwrap();
    let img = img.to_rgb(); // derive RGB Image
    let width = img.width();
    let height = img.height();


    println!("kernel size = 3 x 3");
    let mut imgbuf_3 = image::ImageBuffer::new(width, height);

    let kernel_3: Array2<f64> = (1./16.) * arr2( & [[ 1., 2., 1., ],
                                                    [ 2., 4., 2., ],
                                                    [ 1., 2., 1., ]]);

    for y in 0..height {
        if y + 2 > height || y <= 1 {
            continue;
        }
        for x in 0..width {
            if x + 2 > width || x <= 1 {
                continue;
            }

            let res_pix_r = convolution(&img, x, y, 'r', &kernel_3);
            let res_pix_g = convolution(&img, x, y, 'g', &kernel_3);
            let res_pix_b = convolution(&img, x, y, 'b', &kernel_3);
            imgbuf_3.put_pixel(x, y, image::Rgb([res_pix_r, res_pix_g, res_pix_b]));
        }
    }
    let output_name = format!("{}_gaussian_kernel3.png",
                              img_path.rsplit("/").next().unwrap().split(".").next().unwrap());
    imgbuf_3.save(output_name).ok().expect("can't save the image");


    println!("kernel size = 5 x 5");
    let mut imgbuf_5 = image::ImageBuffer::new(width, height);

    let kernel_5: Array2<f64> = (1./256.) * arr2( & [[ 1.,  4.,  6.,  4., 1., ],
                                                     [ 4., 16., 24., 16., 4., ],
                                                     [ 6., 24., 36., 24., 6., ],
                                                     [ 4., 16., 24., 16., 4., ],
                                                     [ 1.,  4.,  6.,  4., 1., ]]);

    for y in 0..height {
        if y + 3 > height || y <= 2 {
            continue;
        }
        for x in 0..width {
            if x + 3 > width || x <= 2 {
                continue;
            }

            let res_pix_r = convolution(&img, x, y, 'r', &kernel_5);
            let res_pix_g = convolution(&img, x, y, 'g', &kernel_5);
            let res_pix_b = convolution(&img, x, y, 'b', &kernel_5);
            imgbuf_5.put_pixel(x, y, image::Rgb([res_pix_r, res_pix_g, res_pix_b]));
        }
    }
    let output_name = format!("{}_gaussian_kernel5.png",
                              img_path.rsplit("/").next().unwrap().split(".").next().unwrap());
    imgbuf_5.save(output_name).ok().expect("can't save the image");

    println!("kernel size = 7 x 7");
    let mut imgbuf_7 = image::ImageBuffer::new(width, height);

    let kernel_7: Array2<f64> = (1./4096.) * arr2( & [[  1.,   6.,  15.,  20.,  15.,   6.,  1., ],
                                                     [  6.,  36.,  90., 120.,  90.,  36.,  6., ],
                                                     [ 15.,  90., 225., 300., 225.,  90., 15., ],
                                                     [ 20., 120., 300., 400., 300., 120., 20., ],
                                                     [ 15.,  90., 225., 300., 225.,  90., 15., ],
                                                     [  6.,  36.,  90., 120.,  90.,  36.,  6., ],
                                                     [  1.,   6.,  15.,  20.,  15.,   6.,  1., ]]);

    for y in 0..height {
        if y + 4 > height || y < 3 {
            continue;
        }
        for x in 0..width {
            if x + 4 > width || x < 3 {
                continue;
            }

            let res_pix_r = convolution(&img, x, y, 'r', &kernel_7);
            let res_pix_g = convolution(&img, x, y, 'g', &kernel_7);
            let res_pix_b = convolution(&img, x, y, 'b', &kernel_7);
            imgbuf_7.put_pixel(x, y, image::Rgb([res_pix_r, res_pix_g, res_pix_b]));
        }
    }
    let output_name = format!("{}_gaussian_kernel7.png",
                              img_path.rsplit("/").next().unwrap().split(".").next().unwrap());
    imgbuf_7.save(output_name).ok().expect("can't save the image");
}
