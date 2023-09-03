use async_std::stream::StreamExt;
use async_std::{fs, path, task};

async fn walk<P: AsRef<path::Path>>(p: P, verbose: bool) -> std::io::Result<()> {
    let mut worq = std::collections::VecDeque::from([p.as_ref().to_path_buf()]);
    let mut dc = 0;
    let mut fc = 0;

    while !worq.is_empty() {
        let current = worq.pop_front().unwrap();
        dc += 1;
        if verbose {
            println!("{}: traversing", current.display());
        }
        let mut entries = fs::read_dir(current).await?;

        while let Some(res) = entries.next().await {
            let entry = res?;
            if entry.file_type().await?.is_dir() {
                worq.push_back(entry.path());
            } else {
                fc += 1;
                if verbose {
                    println!("\t{}", entry.path().display());
                }
            }
        }
    }
    if verbose {
        println!("Directory(ies): {dc}, file(s): {fc}");
    }
    Ok(())
}

fn main() {
    let mut verbose = false;
    for arg in std::env::args() {
        match arg.to_lowercase().as_ref() {
            "--verbose" | "-v" => verbose = true,
            _ => (),
        }
    }
    let _ = task::block_on(walk(".", verbose));
}
