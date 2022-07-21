/// Exhausts disk space, reporting timestamps and elapsed time along the way
use anyhow::Context;
use chrono::Utc;

const DIR_NAME: &str = "sink_hole";
const FILE_BASE: &str = "file";
const FILE_SIZE: usize = 1_073_741_824;

fn main() -> Result<(), anyhow::Error> {
    let buffer: Vec<u8> = vec![0x1; FILE_SIZE];

    println!("WARNING: THIS PROGRAM WILL EXHAUST ALL AVAILABLE DISK SPACE!");
    println!("{} starting", Utc::now().to_rfc3339());
    let _ = std::fs::create_dir(DIR_NAME)
        .with_context(|| format!("mkdir {:?}", DIR_NAME))?;

    let mut which = 0;
    let mut last = std::time::Instant::now();
    loop {
        let filename = format!("{}/{}-{}", DIR_NAME, FILE_BASE, which);
        which = which + 1;
        let result = std::fs::write(&filename, &buffer)
            .with_context(|| format!("write {:?}", &filename));
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(last);
        if let Err(error) = result {
            println!(
                "{} FAILED writing {} bytes to {} (failed write took {:5.3}s): {:#}",
                Utc::now().to_rfc3339(),
                buffer.len(),
                &filename,
                elapsed.as_secs_f64(),
                error,
            );
            return Err(error);
        } else {
            println!(
                "{} wrote {} bytes to {} (took {:5.3}s)",
                Utc::now().to_rfc3339(),
                buffer.len(),
                &filename,
                elapsed.as_secs_f64(),
            );
            last = now;
        }
    }
}
