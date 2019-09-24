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
            let _x = ((x as i32) + (xp - (kernel_size_i32/2))) as u32;
            let _y = ((y as i32) + (yp - (kernel_size_i32/2))) as u32;

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

fn pascal(x: usize, y: usize) -> f64 {
    if x == 1 || x == y {
        1.
    } else {
        pascal(x - 1, y - 1) + pascal(x, y - 1)
    }
}

fn make_kernel(kernel_size: usize) -> Array2<f64> {
    let mut array_pascal: Array1<f64> = Array::zeros(kernel_size);
    let mut kernel: Array2<f64> = Array::zeros((kernel_size, kernel_size));

    for i in 0..kernel_size {
        array_pascal[i] = pascal(i + 1, kernel_size);
    }

    for (i, v) in array_pascal.clone().iter().enumerate() {
        for (j, h) in array_pascal.clone().iter().enumerate() {
            kernel[(i, j)] = v * h;
        }
    }

    let sum_kernel = &kernel.sum();

    (1. / sum_kernel) * kernel
}

fn main() {
    let mut args = env::args();
    args.next(); // skip command name

    let img_path = match args.next() {
            None => {
                    eprintln!("Error: Input file path is not specified. ");
                    eprintln!("Usage: cargo run image-path");
                    return;
            },
            Some(s) => s,
    };

    // Load Image
    let img = image::open(&img_path).unwrap();
    let img = img.to_rgb(); // derive RGB Image
    let width = img.width();
    let height = img.height();

    let max: usize = 31;

    for n in (3..=max).filter(|&x| x % 2 == 1){
        println!("kernel size = {0} x {0}", n);
        let mut imgbuf = image::ImageBuffer::new(width, height);

        let kernel: Array2<f64> = make_kernel(n);

        let offset = (kernel.shape()[0] / 2) as u32;

        for y in 0..height {
            if y + (offset + 1) > height || y < offset {
                continue;
            }
            for x in 0..width {
                if x + (offset + 1) > width || x < offset {
                    continue;
                }
                let res_pix_r = convolution(&img, x, y, 'r', &kernel);
                let res_pix_g = convolution(&img, x, y, 'g', &kernel);
                let res_pix_b = convolution(&img, x, y, 'b', &kernel);
                imgbuf.put_pixel(x, y, image::Rgb([res_pix_r, res_pix_g, res_pix_b]));
            }
        }
        let output_name = format!("{}_gaussian_kernel{}.png",
                                  img_path.rsplit("/").next().unwrap().split(".").next().unwrap(),
                                  n);
        imgbuf.save(output_name).ok().expect("can't save the image");
    }
}
