//! 분 단위로 이전 시간을 지정하여 그 이후에 변경된 파일들을 특정 폴더로 복사한다.
//! 이 기능이 있으면 vcpkg 설치 후 새로 추가된 파일들만 복사 가능하다.
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// src 폴더를 하위폴더까지 뒤져서 dst의 폴더의 하위 폴더에 복사한다. 
/// mb는 minutes before로 이 시간 이후에 변경된 파일만 복사한다.
pub fn copy<P: AsRef<Path>>(src: P, dst: P, mb: u32) {

    fn for_each_dir<P: AsRef<Path>, Q: AsRef<Path>>(dst: &P, path: Q, after: &SystemTime, dir_modified: bool) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let meta = entry.metadata().unwrap();
                    if meta.is_dir() {
                        let modified_time = entry.metadata().unwrap().modified().unwrap();
                        let modified = modified_time >= *after;
                        for_each_dir(dst, entry.path(), after, modified);
                    } else {
                        let modified_time = entry.metadata().unwrap().modified().unwrap();
                        if dir_modified || modified_time >= *after {
                            copy_file(dst, &entry);
                        }
                    }
                }
            }
        }
    }

    fn copy_file<P: AsRef<Path>>(dst: &P, entry: &fs::DirEntry) {
        let mut to = PathBuf::new();
        to.push(dst.as_ref());
        to.push(entry.path().parent().unwrap());
        let _result = fs::create_dir_all(to.as_path());

        to.push(entry.path().file_name().unwrap());
        let result = fs::copy(entry.path(), &to);

        // TODO: canonicalize보다 익숙한 형식의 변환이 필요하다.
        // let to = to.canonicalize().unwrap();

        match result {
            Err(e) => {
                println!(
                    "Error: {:?}. Failed to copy {:?} to {:?}",
                    e,
                    entry.path(),
                    to.as_path()
                )
            }
            _ => { println!("Copy to {:?}", &to); }
        }
    }

    let now = SystemTime::now();
    let duration = Duration::from_secs((mb * 60) as u64);
    let before = now - duration;

    for_each_dir(&dst, src, &before, false);
}