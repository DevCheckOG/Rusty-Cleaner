use std::{
    env,
    fs::{remove_dir_all, remove_file, Metadata},
    path::{Path, PathBuf},
};

use fs_extra::dir::get_size;
use walkdir::WalkDir;

pub fn clear_temporary_files() {
    let mut total_free_size: Vec<f64> = vec![];
    let temp_path: &Path = Path::new("C:/Windows/Temp/");
    let temp_local_app_data: Option<String> = env::var("localappdata")
        .ok()
        .map(|path| path.replace('\\', "/") + "/Temp/");

    if !temp_path.exists() || !Path::new(&temp_local_app_data.to_owned().unwrap()).exists() {
        return;
    }

    println!("\n🔃 [1/2] Analizing -> Temporal Files");

    WalkDir::new(temp_path).into_iter().for_each(|path| {
        if let Ok(file) = path {
            if !file.file_name().to_str().unwrap().contains("Temp") {
                let metadata: Metadata = file.metadata().unwrap();
                let file_size: f64 = match get_size(file.path().to_str().unwrap()) {
                    Ok(size) => size as f64,
                    Err(_) => 0.0,
                };

                println!(
                    "🎯 File/Directory in sight: {}",
                    file.path().as_os_str().to_str().unwrap()
                );

                if metadata.is_file() {
                    let _ = remove_file(file.path());
                } else {
                    let _ = remove_dir_all(file.path());
                }

                println!("✅ File/Directory deleted.");

                total_free_size.push((file_size / 1_048_576.0).round());
            }
        }
    });

    if let Some(path) = temp_local_app_data {
        WalkDir::new(path).into_iter().for_each(|directory| {
            if let Ok(file) = directory {
                if !file.file_name().to_str().unwrap().contains("Temp") {
                    let metadata: Metadata = file.metadata().unwrap();
                    let file_size: f64 = match get_size(file.path().to_str().unwrap()) {
                        Ok(size) => size as f64,
                        Err(_) => 0.0,
                    };

                    println!(
                        "🎯 File/Directory in sight: {}",
                        file.path().as_os_str().to_str().unwrap()
                    );

                    if metadata.is_file() {
                        let _ = remove_file(file.path());
                    } else {
                        let _ = remove_dir_all(file.path());
                    }

                    println!("✅ File/Directory deleted.");

                    total_free_size.push((file_size / 1_048_576.0).round());
                }
            }
        });
    }

    println!(
        "🗑️ [2/2] Freed up space in -> Temporal Files | {} MB\n",
        total_free_size.iter().sum::<f64>()
    );
}

pub fn clear_firefox() {
    let mut total_free_size: Vec<f64> = vec![];
    let firefox: Option<String> = env::var("localappdata")
        .ok()
        .map(|path| path.replace('\\', "/") + "/Mozilla/Firefox/Profiles/");

    if !Path::new(&firefox.to_owned().unwrap()).exists() {
        return;
    }

    println!("\n🔃 [1/2] Analizing -> Firefox");

    if let Some(path) = firefox {
        WalkDir::new(path).into_iter().for_each(|directory| {
            if let Ok(file) = directory {
                let metadata: Metadata = file.metadata().unwrap();

                if metadata.is_dir()
                    && (["cache2", "jumpListCache"]).iter().any(|&dir| {
                        file.path().to_str().unwrap().contains(dir)
                            && !file.file_name().to_str().unwrap().contains(dir)
                    })
                {
                    let file_size: f64 = match get_size(file.path().to_str().unwrap()) {
                        Ok(size) => size as f64,
                        Err(_) => 0.0,
                    };

                    println!(
                        "🎯 File/Directory in sight: {}",
                        file.path().as_os_str().to_str().unwrap()
                    );

                    let _ = remove_dir_all(file.path());

                    println!("✅ File/Directory deleted.");

                    total_free_size.push((file_size / 1_048_576.0).round());
                }
            }
        });

        println!(
            "🗑️ [2/2] Freed up space in -> Firefox | {} MB\n",
            total_free_size.iter().sum::<f64>()
        );
    }
}

pub fn clear_discord() {
    let discord: Option<String> = env::var("appdata")
        .ok()
        .map(|path| path.replace('\\', "/") + "/discord/");

    if !Path::new(&discord.to_owned().unwrap()).exists() {
        return;
    }

    println!("\n🔃 [1/2] Analizing -> Discord");

    if let Some(path) = discord {
        let home: &Path = Path::new(path.as_str());
        let mut total_free_size: Vec<f64> = vec![];

        WalkDir::new(home).into_iter().for_each(|path| {
            if let Ok(file) = path {
                let metadata: Metadata = file.metadata().unwrap();

                if metadata.is_dir()
                    && (["Code Cache", "GPUCache"]).iter().any(|&dir| {
                        file.path().to_str().unwrap().contains(dir)
                            && !file.file_name().to_str().unwrap().contains(dir)
                    })
                {
                    let file_size: f64 = match get_size(file.path().to_str().unwrap()) {
                        Ok(size) => size as f64,
                        Err(_) => 0.0,
                    };

                    println!(
                        "🎯 File/Directory in sight: {}",
                        file.path().as_os_str().to_str().unwrap()
                    );

                    if metadata.is_file() {
                        let _ = remove_file(file.path());
                    } else {
                        let _ = remove_dir_all(file.path());
                    }

                    println!("✅ File/Directory deleted.");

                    total_free_size.push((file_size / 1_048_576.0).round());
                }
            };
        });

        println!(
            "🗑️ [2/2] Freed up space in -> Discord | {} MB\n",
            total_free_size.iter().sum::<f64>()
        );
    }
}

pub fn clear_chromium_web_browsers() {
    let local_app_data: Option<String> = env::var("localappdata").ok();
    let app_data: Option<String> = env::var("appdata").ok();

    if local_app_data.is_some() && app_data.is_some() {
        let mut total_free_size: Vec<f64> = vec![];

        let chrome: PathBuf = Path::new(&local_app_data.to_owned().unwrap()).join("Google/Chrome/");
        let brave: PathBuf = Path::new(&local_app_data.unwrap())
            .join("BraveSoftware/Brave-Browser/User Data/Default/");
        let opera: PathBuf = Path::new(&app_data.unwrap()).join("Opera Software/Opera/");

        if !chrome.exists() && !brave.exists() {
            return;
        }

        for browser in [&chrome, &brave, &opera] {
            if browser.exists() {
                WalkDir::new(browser).into_iter().for_each(|path| {
                    if let Ok(file) = path {
                        let metadata: Metadata = file.metadata().unwrap();

                        if metadata.is_dir()
                            && (["ShaderCache", "GPUCache"].iter().any(|&dir| {
                                file.path().to_str().unwrap().contains(dir)
                                    && !file.file_name().to_str().unwrap().contains(dir)
                            }))
                        {
                            let file_size: f64 = match get_size(file.path().to_str().unwrap()) {
                                Ok(size) => size as f64,
                                Err(_) => 0.0,
                            };

                            println!(
                                "🎯 File/Directory in sight: {}",
                                file.path().as_os_str().to_str().unwrap()
                            );

                            if metadata.is_file() {
                                let _ = remove_file(file.path());
                            } else {
                                let _ = remove_dir_all(file.path());
                            }

                            println!("✅ File/Directory deleted.");

                            total_free_size.push((file_size / 1_048_576.0).round());
                        }
                    }
                });
            }
        }

        println!(
            "🗑️ [2/2] Freed up space in -> Web browsers based on Chromium | {} MB\n",
            total_free_size.iter().sum::<f64>()
        );
    }
}
