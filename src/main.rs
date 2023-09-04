use async_std::stream::StreamExt;
use async_std::{fs, path, task};
use image;
use memmap2;

#[derive(Debug)]
struct Count {
    dir: usize,
    img: usize,
    oth: usize,
}

impl Count {
    fn zero() -> Self {
        Self {
            dir: 0,
            img: 0,
            oth: 0,
        }
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "directory(ies): {}, image(s): {}, other file(s): {}",
            self.dir, self.img, self.oth
        )
    }
}

async fn open_image<P: AsRef<path::Path>>(
    p: P,
    guess: bool,
) -> image::ImageResult<image::DynamicImage> {
    if guess {
        let file = fs::File::open(p).await?;
        let bytes = unsafe { memmap2::Mmap::map(&file)? };
        let reader = image::io::Reader::new(std::io::Cursor::new(bytes)).with_guessed_format()?;
        reader.decode()
    } else {
        image::open(p.as_ref())
    }
}

async fn walk<P: AsRef<path::Path>>(
    p: P,
    guess: bool,
    verbose: bool,
    summary: bool,
) -> std::io::Result<()> {
    let mut worq = std::collections::VecDeque::from([p.as_ref().to_path_buf()]);
    let mut count = Count::zero();
    let mut expected_dir_count: usize = 0;

    while !worq.is_empty() {
        let current = worq.pop_front().unwrap();
        let mut local_count = Count::zero();
        let mut entries = fs::read_dir(&current).await?;

        count.dir += 1;
        while let Some(res) = entries.next().await {
            let entry = res?;
            if entry.file_type().await?.is_dir() {
                worq.push_back(entry.path());
                local_count.dir += 1;
            } else {
                match open_image(path::Path::new(&entry.path()), guess).await {
                    Ok(pic) => {
                        local_count.img += 1;
                        if verbose {
                            println!("\t{} {:?}", entry.path().display(), pic.color());
                        }
                    }
                    Err(error) => {
                        local_count.oth += 1;
                        if verbose {
                            println!("\t{} ignored: {:?}", entry.path().display(), error);
                        }
                    }
                }
            }
        }

        expected_dir_count += local_count.dir;
        count.img += local_count.img;
        count.oth += local_count.oth;
        if verbose {
            println!("{} has {}", current.display(), local_count);
        }
    }
    if summary {
        println!(
            "Total {}, {} directory(ies) skipped",
            count,
            expected_dir_count.saturating_sub(count.dir)
        );
    }
    Ok(())
}

fn main() {
    let mut guess = false;
    let mut verbose = false;
    for arg in std::env::args() {
        match arg.to_lowercase().as_ref() {
            "--guess" | "-g" => guess = true,
            "--verbose" | "-v" => verbose = true,
            _ => (),
        }
    }
    let _ = task::block_on(walk(".", guess, verbose, true));
}
