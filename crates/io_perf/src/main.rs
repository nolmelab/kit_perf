//! io perf
//! - write perf : 파일 새롭게 생성 후 바이트 추가
//! - read perf  : 다양한 크기로 읽기 진행
//!

use std::fs;
use std::fmt;
use std::io::{ BufReader, BufRead, BufWriter, Write };
use std::time::Instant;
use clap::Parser;
use indicatif::{ MultiProgress, ProgressBar, ProgressState, ProgressStyle };

// BufReader로 읽는다.
fn read(pb: &ProgressBar, path: &str, bs: usize) -> Result<f32, anyhow::Error> {
    let file = fs::OpenOptions::new().read(true).open(&path)?;
    let mut reader = BufReader::with_capacity(bs, file);
    let mut pos = 0_u64;
    let start = Instant::now();

    loop {
        let buffer = reader.fill_buf()?;
        let buffer_length = buffer.len();

        if buffer_length == 0 {
            break;
        }

        pos += buffer_length as u64;
        pb.set_position(pos);

        reader.consume(buffer_length);
    }

    let elapsed = start.elapsed().as_secs_f32();

    Ok(elapsed)
}

// BufWriter로 쓴다.
fn write(
    pb: &ProgressBar,
    path: &str,
    block_size: usize,
    file_size: usize
) -> Result<f32, anyhow::Error> {
    let file = fs::OpenOptions::new().write(true).create(true).open(&path)?;
    let buf = vec![1_u8; block_size];
    let count = file_size / block_size;
    let buf = &buf[..];
    let mut pos = 0_u64;
    let start = Instant::now();

    let mut writer = BufWriter::new(file);

    for _ in 0..count {
        writer.write_all(buf)?;

        pos += block_size as u64;
        pb.set_position(pos);
    }

    let elapsed = start.elapsed().as_secs_f32();

    Ok(elapsed)
}

/// 파일 입출력 성능을 측정한다.
#[derive(Parser, Debug)]
#[command(long_about = None)]
struct Args {
    /// 파일 이름을 지정한다.
    #[arg(short, long, default_value = "test.bin")]
    file: String,

    /// 블럭 크기를 지정한다.
    #[arg(short, long, default_value_t = 1024)]
    block_size: usize,

    /// 파일 크기를 지정한다.
    #[arg(short, long, default_value_t = 1024 * 1024)]
    file_size: usize,

    /// 파일을 정리할지 지정한다.
    #[arg(short, long, default_value_t = false)]
    cleanup: bool,
}

fn main() {
    let args = Args::parse();

    let m = MultiProgress::new();

    let style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}"
    )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn fmt::Write|
            write!(w, "{:.2}s", state.eta().as_secs_f64()).unwrap()
        )
        .progress_chars("#>-");

    let write_pb = m.add(ProgressBar::new(args.file_size as u64));
    let read_pb = m.insert_after(&write_pb, ProgressBar::new(args.file_size as u64));

    write_pb.set_style(style.clone());
    read_pb.set_style(style.clone());

    measure_write(&write_pb, &args.file, args.block_size, args.file_size);
    measure_read(&read_pb, &args.file, args.block_size);

    if args.cleanup {
        cleanup(&args.file);
    }
}

fn measure_write(pb: &ProgressBar, path: &str, block_size: usize, file_size: usize) {
    pb.set_message("writing...");
    let result: Result<f32, anyhow::Error> = write(&pb, path, block_size, file_size);
    finish_measure(pb, &result);
}

fn measure_read(pb: &ProgressBar, path: &str, block_size: usize) {
    pb.set_message("reading...");
    let result: Result<f32, anyhow::Error> = read(&pb, path, block_size);
    finish_measure(pb, &result);
}

fn finish_measure(pb: &ProgressBar, result: &Result<f32, anyhow::Error>) {
    match result {
        Ok(d) => {
            pb.finish_with_message(format!("elapsed: {:.2}", d));
        }
        Err(e) => {
            pb.finish_with_message(format!("failed. {}", e));
        }
    }
}

fn cleanup(path: &str) {
    let result = fs::remove_file(path);

    match result {
        Ok(_) => {
            println!("cleaned up the file: {}", path);
        }
        Err(e) => {
            println!("failed to remove file: {}, error: {}", path, e);
        }
    }
}
