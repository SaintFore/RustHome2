use std::fs::{OpenOptions, File};
use std::io::{Write};
use std::path::Path;

#[derive(PartialEq, Debug)] // 添加 Debug trait 以便于调试
pub enum LogOutput {
    Console,
    File(String),
}

impl Default for LogOutput {
    fn default() -> Self {
        LogOutput::File("log.txt".to_string()) // 默认保存到 log.txt 文件中
    }
}

pub struct Logger {
    pub output: LogOutput, // 将 output 字段设置为 public
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            output: LogOutput::default(),
        }
    }
}

impl Logger {
    pub fn new(output: LogOutput) -> Self {
        Logger { output }
    }

    pub fn log(&self, message: &str) {
        match &self.output {
            LogOutput::Console => {
                println!("{}", message);
            }
            LogOutput::File(file_name) => {
                let path = Path::new(file_name);
                let mut file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&path)
                    .unwrap();
                writeln!(file, "{}", message).unwrap();
            }
        }
    }

    pub fn clear(&self) {
        if let LogOutput::File(file_name) = &self.output {
            let _ = File::create(file_name); // 创建一个新文件以清空内容
        }
    }

    pub fn set_output(&mut self, new_output: LogOutput) {
        self.output = new_output;
    }
}
