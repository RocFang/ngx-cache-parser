use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::mem;
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let filename = args.get(1).expect("Usage: ./main <filename>");

    let cache = parse_ngx_cache_header(filename).unwrap();
    println!("{:?}", &cache);
}

fn parse_ngx_cache_header(filename: &str) -> Result<NgxCache, Error> {
    let mut f = File::open(filename)?;
    let mut cache: NgxCache = unsafe { mem::zeroed() };

    unsafe {
        let buffer: &mut [u8] = std::slice::from_raw_parts_mut(
            &mut cache as *mut NgxCache as *mut u8,
            mem::size_of::<NgxCache>(),
        );
        f.read_exact(buffer)?;
    }
    Ok(cache)
}

#[repr(C)]
#[derive(Debug, PartialEq)]
struct NgxCache {
    version: u64,
    valid_sec: u64,
    updating_sec: u64,
    error_sec: u64,
    last_modified: u64,
    date: u64,
    crc32: u32,
    valid_msec: u16,
    header_start: u16,
    body_start: u16,
    etag_len: u8,
    etag: [u8; 128],
    vary_len: u8,
    vary: [u8; 128],
    variant: [u8; 16],
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cache_1() {
        let effect_etag = [
            34, 54, 50, 48, 57, 97, 101, 49, 50, 45, 51, 98, 57, 100, 56, 34,
        ];
        let mut etag: [u8; 128] = [0; 128];
        for i in 0..effect_etag.len() {
            etag[i] = effect_etag[i];
        }
        let effect_vary = [
            65, 99, 99, 101, 112, 116, 45, 69, 110, 99, 111, 100, 105, 110, 103,
        ];
        let mut vary: [u8; 128] = [0; 128];
        for i in 0..effect_vary.len() {
            vary[i] = effect_vary[i];
        }

        let target_cache = NgxCache {
            version: 5,
            valid_sec: 1649932707,
            updating_sec: 0,
            error_sec: 0,
            last_modified: 1644801554,
            date: 1649932107,
            crc32: 92295986,
            valid_msec: 0,
            header_start: 400,
            body_start: 1109,
            etag_len: 16,
            etag: etag,
            vary_len: 15,
            vary: vary,
            variant: [
                155, 248, 89, 207, 207, 213, 164, 59, 139, 150, 26, 38, 132, 238, 174, 76,
            ],
        };

        let cache = parse_ngx_cache_header("testdata/cache_file_1").unwrap();
        println!("{:#?}", cache);
        assert_eq!(cache, target_cache);
    }
}
