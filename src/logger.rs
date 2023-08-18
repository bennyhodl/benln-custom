use lightning::util::logger::Logger;

pub struct BenLogger;

impl BenLogger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Logger for BenLogger {
    fn log(&self, record: &lightning::util::logger::Record) {
        let raw_log = record.args.to_string();
        let log = format!(
            "{:<5} [{}:{}] {}\n",
            record.level.to_string(),
            record.module_path,
            record.line,
            raw_log
        );
        println!("{}", log)
    }
}
