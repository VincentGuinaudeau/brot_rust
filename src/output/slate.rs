
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

use byte_slice_cast::*;

use super::view::View;

pub struct Slate {
	x_size: u32,
	y_size: u32,
	max:    u16,
	matrix: Vec< u16 >,
}

impl Slate {
	pub fn from_view(view: &View) -> Slate {
		Slate {
			x_size: view.x_size() as u32,
			y_size: view.y_size() as u32,
			max: 0,
			matrix: [0].repeat((view.x_size() * view.y_size()) as usize),
		}
	}

	pub fn compute_offset(&self, x: u32, y: u32) -> usize {
		(y * self.x_size + x).try_into().unwrap()
	}

	pub fn increment(&mut self, x: u32, y: u32) -> () {
		let offset: usize = self.compute_offset(x, y);
		self.matrix[offset] += 1;
		if self.max < self.matrix[offset] {
			self.max = self.matrix[offset];
		}
	}

	pub fn to_png(&self, path_str: &str) {
		let path = Path::new(path_str);
		let file = File::create(path).unwrap();
		let ref mut file_writter = BufWriter::new(file);

		let mut encoder = png::Encoder::new(file_writter, self.x_size, self.y_size);
		encoder.set_color(png::ColorType::Grayscale);
		encoder.set_depth(png::BitDepth::Sixteen);
		encoder.set_trns(vec!(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8));
		encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
		let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
			(0.31270, 0.32900),
			(0.64000, 0.33000),
			(0.30000, 0.60000),
			(0.15000, 0.06000)
		);
		encoder.set_source_chromaticities(source_chromaticities);
		let mut writer = encoder.write_header().unwrap();

		// multipling each pixel's value so that the greatest pixel is white
		println!("slat max : {}", self.max);
		let normalized_pixels = self.matrix.iter().map(|pixel| {
			if self.max > 0 {
				((*pixel as u32) * (std::u16::MAX as u32) / (self.max as u32)) as u16
			}
			else {
				0
			}
		}).collect::<Vec< u16 >>();

		// inverting endianess if necessarry
		let tmp_vector: Vec< u8 >;
		let matrix_in_right_order: &[u8] = match [1u16; 1].as_byte_slice()[0] {
			0 => normalized_pixels.as_byte_slice(),
			_ => {
				let slice = normalized_pixels.as_byte_slice();
				let mut odd_iter = slice.iter().skip(1).step_by(2); 
				let mut even_iter = slice.iter().step_by(2);
				let mut count = 0;
				tmp_vector = std::iter::from_fn(|| {
					count += 1;
					if count % 2 == 1 { odd_iter.next() } else { even_iter.next() }
				}).copied().collect::<Vec<u8>>();
				tmp_vector.as_byte_slice()
			},
		};

		writer.write_image_data(matrix_in_right_order).unwrap();
	}
}
