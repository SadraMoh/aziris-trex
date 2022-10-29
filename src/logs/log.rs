pub enum LogType {
  Debug,
  Warning,
  Error,
}

pub struct Log {
  log_type: LogType,
  msg: String,
}