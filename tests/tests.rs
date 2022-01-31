use rgb::ComponentMap;
use std::path::PathBuf;
use imgref::ImgVec;
use imgref::ImgRef;
use rgb::RGBA8;
use gifski_lite::*;
use std::fs::File;
use std::io::Read;

// FIXME: There is some kind of deadlock while writing that causes this test not to complete.
#[test]
fn n_frames() {
    for num_frames in 1..=5 {
        let (mut c, w) = new(Settings::default()).unwrap();

        let t = std::thread::spawn(move || {
            for n in 0..num_frames {
                c.add_frame_rgba(n, load_png_from_file(frame_filename(n)), n as f64 * 0.1).unwrap();
            }
        });
        
        let mut out = Vec::new();
        w.write(&mut out).unwrap();
        t.join().unwrap();

        let mut n = 0;
        for_each_frame(&out, |_, actual| {
            let expected = lodepng::decode32_file(frame_filename(n)).unwrap();
            let expected = ImgVec::new(expected.buffer, expected.width, expected.height);
            assert_images_eq(expected.as_ref(), actual, 0.31);
            n += 1;
        });
        assert_eq!(n, num_frames);
    }
}

#[test]
fn all_dupe_frames() {
    let (mut c, w) = new(Settings::default()).unwrap();

    let t = std::thread::spawn(move || {
        c.add_frame_rgba(0, load_png_from_file(frame_filename(1)), 0.1).unwrap();
        c.add_frame_rgba(1, load_png_from_file(frame_filename(1)), 1.2).unwrap();
        c.add_frame_rgba(2, load_png_from_file(frame_filename(1)), 1.3).unwrap();
    });

    let mut out = Vec::new();
    w.write(&mut out).unwrap();
    t.join().unwrap();

    let mut n = 0;
    for_each_frame(&out, |frame, actual| {
        let expected = lodepng::decode32_file(frame_filename(1)).unwrap();
        let expected = ImgVec::new(expected.buffer, expected.width, expected.height);
        assert_images_eq(expected.as_ref(), actual, 0.);
        assert_eq!(frame.delay, 130);
        n += 1;
    });
    assert_eq!(n, 1);
}

#[test]
fn all_but_one_dupe_frames() {
    let (mut c, w) = new(Settings::default()).unwrap();

    let t = std::thread::spawn(move || {
        c.add_frame_rgba(0, load_png_from_file(frame_filename(0)), 0.0).unwrap();
        c.add_frame_rgba(1, load_png_from_file(frame_filename(1)), 1.2).unwrap();
        c.add_frame_rgba(2, load_png_from_file(frame_filename(1)), 1.3).unwrap();
    });

    let mut out = Vec::new();
    w.write(&mut out).unwrap();
    t.join().unwrap();

    let mut n = 0;
    for_each_frame(&out, |frame, actual| {
        let expected = lodepng::decode32_file(frame_filename(if n == 0 {0} else {1})).unwrap();
        let expected = ImgVec::new(expected.buffer, expected.width, expected.height);
        assert_images_eq(expected.as_ref(), actual, 0.25);
        assert_eq!(frame.delay, if n == 0 {120} else {2*10}); // 2*, because 1.2…1.3 + 1.3…1.4 (assumed fps)
        n += 1;
    });
    assert_eq!(n, 2);
}

fn frame_filename(n: usize) -> PathBuf {
    format!("tests/{}.png", (n%3)+1).into()
}

fn for_each_frame(mut gif_data: &[u8], mut cb: impl FnMut(&gif::Frame, ImgRef<RGBA8>)) {
    let mut gif_opts = gif::DecodeOptions::new();
    gif_opts.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = gif_opts.read_info(&mut gif_data).unwrap();
    let mut screen = gif_dispose::Screen::new_decoder(&decoder);

    while let Some(frame) = decoder.read_next_frame().unwrap() {
        screen.blit_frame(&frame).unwrap();
        cb(&frame, screen.pixels.as_ref());
    }
}

fn load_png_from_file(filename: PathBuf) -> ImgVec<RGBA8> {
    let mut file = File::open(filename).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let decoded = lodepng::decode32(&data).unwrap();
    ImgVec::new(decoded.buffer, decoded.width, decoded.height)
}

#[track_caller]
fn assert_images_eq(a: ImgRef<RGBA8>, b: ImgRef<RGBA8>, max_diff: f64) {
    let diff = a.pixels().zip(b.pixels()).map(|(a,b)| {
        let a = a.map(|c| c as i32);
        let b = b.map(|c| c as i32);
        let d = a - b;
        (d.r * d.r +
         d.g * d.g +
         d.b * d.b) as u64
    }).sum::<u64>() as f64 / (a.width() * a.height()) as f64;
    assert!(diff <= max_diff, "{} diff > {}", diff, max_diff);
}
