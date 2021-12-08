use rexiv2;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {
	let args = std::env::args();

	for (i, arg) in args.enumerate() {
		if i > 0 {
			print!("Converting {}...", arg);
			match extract(arg) {
				Ok(0) => println!(" not a motion photo."),
				Ok(_) => println!(" done."),
				Err(e) => println!(" Error: {}", e),
			}
		}
	}
}

fn extract(path_str: String) -> Result<usize, std::io::Error> {
	let path = PathBuf::from(&path_str);

	let path_out_jpg = path.with_extension("photo.jpg");
	let path_out_mp4 = path.with_extension("video.mp4");

	let meta = match rexiv2::Metadata::new_from_path(&path_str) {
		Ok(t) => Ok(t),
		Err(e) => Err(std::io::Error::new(
			std::io::ErrorKind::Unsupported,
			format!("Could not read picture metadata: {}", e),
		)),
	}?;

	if !meta.has_tag("Xmp.GCamera.MicroVideoOffset") {
		return Ok(0);
	}

	let offset: usize = meta
		.get_tag_string("Xmp.GCamera.MicroVideoOffset")
		.unwrap()
		.parse()
		.unwrap();

	let tags = match meta.get_xmp_tags() {
		Ok(t) => Ok(t),
		Err(e) => Err(std::io::Error::new(
			std::io::ErrorKind::Unsupported,
			format!("Could not write to file: {}", e),
		)),
	}?;

	for tag in tags {
		match tag.find("Xmp.GCamera.MicroVideo") {
			Some(0) => meta.clear_tag(&tag),
			_ => true,
		};
	}

	let mut file = std::fs::File::open(path)?;
	let file_size = file.metadata().unwrap().len() as usize;

	let jpg_size = file_size - offset;

	let mut buf: Vec<u8> = Vec::with_capacity(file_size);

	file.read_to_end(&mut buf)?;

	let mut file_jpg = std::fs::File::create(&path_out_jpg)?;
	file_jpg.write_all(&buf[..jpg_size])?;
	file_jpg.flush()?;
	drop(file_jpg);
	match meta.save_to_file(path_out_jpg) {
		Ok(_) => Ok(()),
		Err(e) => Err(std::io::Error::new(
			std::io::ErrorKind::Unsupported,
			format!("Could not write picture metadata: {}", e),
		)),
	}?;

	let mut file_mp4 = std::fs::File::create(path_out_mp4)?;
	file_mp4.write_all(&buf[jpg_size..])?;
	file_mp4.flush()?;

	Ok(1)
}
