use chrono::prelude::*;

pub fn log(message: &str) {
    let utc: DateTime<Utc> = Utc::now();
    eprintln!("[{utc}]: {message}");
}

/// control stop or not in receiver loop
///
/// you can also use [Option] for this.
///
/// # expample
/// ```
/// while let Some(e) = receiver.recv() {
///     if let Stop = e {
///         break;
///     } else {
///         todo!();
///     }
/// }
///
/// sender.send(Stopper::Stop);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Stopper<T> {
    Stop,
    Go(T),
}
