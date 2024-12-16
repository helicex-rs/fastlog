use crate::error::LogError;
use crate::plugin::file_split::Packer;
use std::fs::File;

/// keep temp{date}.log
#[derive(Clone)]
pub struct LogPacker {}
impl Packer for LogPacker {
    fn pack_name(&self) -> &'static str {
        "log"
    }

    fn do_pack(&self, _log_file: File, _log_file_path: &str) -> Result<bool, LogError> {
        //do nothing,and not remove file
        return Ok(false);
    }
}

#[cfg(feature = "zip")]
use zip::result::ZipResult;
#[cfg(feature = "zip")]
use zip::write::FileOptions;

/// you need enable fastlog = { ... ,features=["zip"]}
/// the zip compress
#[cfg(feature = "zip")]
pub struct ZipPacker {}

#[cfg(feature = "zip")]
impl Packer for ZipPacker {
    fn pack_name(&self) -> &'static str {
        "zip"
    }

    fn do_pack(&self, mut log_file: File, log_file_path: &str) -> Result<bool, LogError> {
        use std::io::Write;
        let mut log_name = log_file_path.replace("\\", "/").to_string();
        if let Some(v) = log_file_path.rfind("/") {
            log_name = log_name[(v + 1)..log_name.len()].to_string();
        }
        let zip_path = log_file_path.replace(".log", ".zip");
        let zip_file = File::create(&zip_path)
            .map_err(|e| LogError::from(format!("[fastlog] create(&{}) fail:{}", zip_path, e)))?;
        //write zip bytes data
        let mut zip = zip::ZipWriter::new(zip_file);
        zip.start_file::<String, ()>(log_name, FileOptions::default())
            .map_err(|e| LogError::from(e.to_string()))?;
        //buf reader
        std::io::copy(&mut log_file, &mut zip).map_err(|e| LogError::from(e.to_string()))?;
        zip.flush().map_err(|e| LogError::from(e.to_string()))?;
        let finish: ZipResult<File> = zip.finish();
        if finish.is_err() {
            //println!("[fastlog] try zip fail{:?}", finish.err());
            return Err(LogError::from(format!(
                "[fastlog] try zip fail{:?}",
                finish.err()
            )));
        }
        return Ok(true);
    }
}

/// you need enable fastlog = { ... ,features=["lz4"]}
#[cfg(feature = "lz4")]
use lz4_flex::frame::FrameEncoder;

/// the zip compress
#[cfg(feature = "lz4")]
pub struct LZ4Packer {}

#[cfg(feature = "lz4")]
impl Packer for LZ4Packer {
    fn pack_name(&self) -> &'static str {
        "lz4"
    }

    fn do_pack(&self, mut log_file: File, log_file_path: &str) -> Result<bool, LogError> {
        let lz4_path = log_file_path.replace(".log", ".lz4");
        let lz4_file = File::create(&lz4_path)
            .map_err(|e| LogError::from(format!("[fastlog] create(&{}) fail:{}", lz4_path, e)))?;
        //write lz4 bytes data
        let mut encoder = FrameEncoder::new(lz4_file);
        //buf reader
        std::io::copy(&mut log_file, &mut encoder).map_err(|e| LogError::from(e.to_string()))?;
        let result = encoder.finish();
        if result.is_err() {
            return Err(LogError::from(format!(
                "[fastlog] try zip fail{:?}",
                result.err()
            )));
        }
        return Ok(true);
    }
}

#[cfg(feature = "gzip")]
use flate2::write::GzEncoder;
#[cfg(feature = "gzip")]
use flate2::Compression;

#[cfg(feature = "gzip")]
pub struct GZipPacker {}

#[cfg(feature = "gzip")]
impl Packer for GZipPacker {
    fn pack_name(&self) -> &'static str {
        "gz"
    }

    fn do_pack(&self, mut log_file: File, log_file_path: &str) -> Result<bool, LogError> {
        use std::io::Write;
        let zip_path = log_file_path.replace(".log", ".gz");
        let zip_file = File::create(&zip_path)
            .map_err(|e| LogError::from(format!("[fastlog] create(&{}) fail:{}", zip_path, e)))?;
        //write zip bytes data
        let mut zip = GzEncoder::new(zip_file, Compression::default());
        std::io::copy(&mut log_file, &mut zip).map_err(|e| LogError::from(e.to_string()))?;
        zip.flush().map_err(|e| LogError::from(e.to_string()))?;
        let finish = zip.finish();
        if finish.is_err() {
            return Err(LogError::from(format!(
                "[fastlog] try zip fail{:?}",
                finish.err()
            )));
        }
        return Ok(true);
    }
}
