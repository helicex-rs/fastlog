#[cfg(test)]
mod test {
    use fastlog::appender::{Command, FastLogRecord, LogAppender};
    use fastlog::consts::LogSize;
    use fastlog::plugin::file_name::FileName;
    use fastlog::plugin::file_split::{FileSplitAppender, RollingType, Keep, RawFile, Rolling, KeepType};
    use fastlog::plugin::packer::LogPacker;
    use fastdate::DateTime;
    use log::Level;
    use std::fs::remove_dir_all;
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_send_pack() {
        let _ = remove_dir_all("target/test/");
        let mut appender = FileSplitAppender::new::<RawFile>(
            "target/test/",
            Box::new(Rolling::new(RollingType::BySize(LogSize::MB(1)))),
            Box::new(KeepType::All),
            Box::new(LogPacker {}),
        )
            .unwrap();
        appender.do_logs(&[FastLogRecord {
            command: Command::CommandRecord,
            level: Level::Error,
            target: "".to_string(),
            args: "".to_string(),
            module_path: "".to_string(),
            file: "".to_string(),
            line: None,
            now: SystemTime::now(),
            formated: "".to_string(),
        }]);
        appender.send_pack(appender.temp_name().replace(".log", &DateTime::now().format("YYYY-MM-DDThh-mm-ss.000000.log")), None);
        sleep(Duration::from_secs(1));
        let rolling_num = KeepType::KeepNum(0).do_keep("target/test/", "temp.log");
        assert_eq!(rolling_num, 1);
        let _ = remove_dir_all("target/test/");
    }


    #[test]
    fn test_extract_file_name() {
        let p = "temp.log".extract_file_name();
        assert_eq!(p, "temp.log");
    }

    #[test]
    fn test_extract_file_name2() {
        let p = "logs/temp.log".extract_file_name();
        assert_eq!(p, "temp.log");
    }

    #[test]
    fn test_extract_file_name3() {
        let p = "logs/".extract_file_name();
        assert_eq!(p, "");
    }

    #[test]
    fn test_extract_file_name4() {
        let p = "C:/logs".extract_file_name();
        assert_eq!(p, "logs");
    }

    #[test]
    fn test_extract_file_name5() {
        let p = "C:/logs/aa.log".extract_file_name();
        assert_eq!(p, "aa.log");
    }

    #[test]
    fn test_extract_file_name6() {
        let p = "C:\\logs\\aa.log".extract_file_name();
        assert_eq!(p, "aa.log");
    }
}
