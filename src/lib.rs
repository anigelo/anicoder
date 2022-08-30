use std::fs;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

const ALLOWED_EXTENSIONS: [&str; 4] = ["mkv", "avi", "mp4", "M4V"];

pub fn encode_to_h264(path: PathBuf) {
    let episodes = get_episodes(path);
    for episode in episodes {
        let new_path = episode.with_file_name(
          format!("transcoding {}.mkv", episode.file_stem().unwrap().to_string_lossy())
        );

        Command::new("ffmpeg")
            .arg("-i").arg(&episode)
            .arg("-preset").arg("slow")
            .arg("-crf").arg("23")
            .arg("-c:a").arg("aac")
            .arg("-c:s").arg("copy")
            .arg(&new_path)
            .spawn().expect("ffmpeg failed")
            .wait().expect("ffmpeg failed");

        let transcoded_metadata = fs::metadata(&new_path);
        if transcoded_metadata.is_ok() && transcoded_metadata.unwrap().len() > 0 {
            match fs::remove_file(&episode) {
                Err(e) => eprintln!("Could not remove '{:?}', Error: {}", episode, e),
                Ok(()) => if let Err(e) = fs::rename(new_path, episode.with_extension("mkv")) {
                    eprintln!("Could not rename '{:?}', Error: {}", episode, e)
                }
            }
        }
    }
}

fn get_episodes(path: PathBuf) -> Vec<PathBuf> {
    WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|file| file.ok())
        .filter_map(|file| {
            let file = file.into_path();
            if is_allowed_extension(&file) {
                Some(file)
            } else { None }
        }).collect()
}

fn is_allowed_extension(file: &PathBuf) -> bool {
    if let Some(extension) = file.extension() {
        ALLOWED_EXTENSIONS.contains(&&*extension.to_string_lossy())
    } else {
        false
    }
}