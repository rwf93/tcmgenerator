mod tcdistributions;

use std::{fs, path::PathBuf, sync::Arc};
use anyhow::Context;
use tokio::{process::Command, sync::Semaphore};
use glob::glob;
use rand::Rng;

use tcdistributions::CASSETTE_DISTRIBUTIONS;

use clap::Parser;
use clio::ClioPath;

extern crate num_cpus;

#[derive(Parser, Debug)]
#[command(author = "rwf93", version = "1.1.0", about = "TCM Generator", long_about = "Converts mp3 files to ogg and generates cassets for TCBoombox.")]
struct Arguments {
    #[arg(short, long, help = "ID appended to the generated files (required by TCBoombox/PZ to be unique, don't ask me).")]
    id: String,
    #[arg(short = 'f', long, help = "Folder where your music files are.", value_parser = clap::value_parser!(ClioPath).exists().is_dir())]
    input_folder: ClioPath,
    #[arg(short = 'j', long, help = "Max amount of ffmpeg tasks to spawn.", default_value_t = num_cpus::get() - 1)]
    max_jobs: usize,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Arguments::parse();

    let mut tc_sounds = String::from("module Tsarcraft\n{\n");
    let mut tc_music_definitions = String::from("require \"TCMusicDefenitions\"\n");
    let mut tc_music = String::from("module Tsarcraft\n{\n\timports\n\t{\n\t\tBase\n\t}\n");
    let mut tc_vehicle_distribution = String::new();
    let mut tc_loading = String::new();

    let unit = "Cassette";
    let mut rng = rand::thread_rng();

    let semaphore = Arc::new(Semaphore::new(args.max_jobs));
    let mut task_pool = Vec::new();

    fs::create_dir_all("./playlist/media/scripts")?;
    fs::create_dir_all("./playlist/media/lua/server/Items")?;
    fs::create_dir_all("./playlist/media/lua/shared")?;
    fs::create_dir_all("./playlist/media/yourMusic/TCBoombox")?;

    let globbed = glob(format!("{}/*.*", args.input_folder.as_os_str().to_str().unwrap()).as_str());
    for entry in globbed.context("Invalid GLOB syntax.")? {
        if let Ok(input_path) = entry {
            let mut output_path = PathBuf::from(input_path.clone());
            let file_stem = unidecode::unidecode(output_path.file_stem().unwrap().to_str().unwrap()); // remove unicode because the engine is stupid.
            output_path.pop();
            output_path.push(&file_stem);
            output_path.set_extension("ogg");

            let item_name = file_stem
                .replace(" ", "")
                .replace(".", "")
                .replace("_", "")
                .replace("-", ""); // Yes, I could use a regex. Will I? No.

            let file_name = unidecode::unidecode(output_path.file_name().unwrap().to_str().unwrap());

            if tc_music_definitions.contains(item_name.as_str()) {
                println!("Skipping dupe: {}", input_path.display());
                continue;
            }

            let icon = rng.gen_range(0..12);

            tc_sounds.push_str(format!("\tsound {unit}{item_name} {{ category = True Music, master = Ambient, clip {{ file = media/yourMusic/TCBoombox/{file_name}, distanceMax = 75, }} }}\n").as_str());
            tc_music_definitions.push_str(format!("GlobalMusic[\"{unit}{item_name}\"] = \"tsarcraft_music_01_62\"\n").as_str());
            tc_music.push_str(format!("\titem {unit}{item_name} {{ Type = Normal, DisplayCategory = Entertainment, Weight = 0.02, Icon = TCTape{icon}, DisplayName = {unit} {file_stem}, WorldStaticModel = Tsarcraft.TCTape{icon}, }}\n").as_str());

            tc_vehicle_distribution.push_str(format!("table.insert(VehicleDistributions.GloveBox.items, \"Tsarcraft.{unit}{item_name}\")\n").as_str());
            tc_vehicle_distribution.push_str(format!("table.insert(VehicleDistributions.GloveBox.items, 0.001)\n").as_str());

            macro_rules! insert_probability {
                ($proability_name: expr, $proability_index: expr, $proability_value: expr) => {
                    tc_loading.push_str(format!("table.insert(ProceduralDistributions.list[\"{}\"].{}, \"Tsarcraft.{unit}{item_name}\")\n", $proability_name, $proability_index).as_str());
                    tc_loading.push_str(format!("table.insert(ProceduralDistributions.list[\"{}\"].{}, {})\n", $proability_name, $proability_index, $proability_value).as_str());
                };
            }

            insert_probability!("CrateCompactDiscs", "items", 0.01);
            insert_probability!("ElectronicStoreMusic", "items", 0.01);
            insert_probability!("MusicStoreCDs", "items", 0.01);
            insert_probability!("MusicStoreSpeaker", "items", 0.01);
            insert_probability!("MusicStoreSpeaker", "junk.items", 0.01);

            insert_probability!(CASSETTE_DISTRIBUTIONS[rng.gen_range(0..CASSETTE_DISTRIBUTIONS.len())], "items", 0.05);
            insert_probability!(CASSETTE_DISTRIBUTIONS[rng.gen_range(0..CASSETTE_DISTRIBUTIONS.len())], "items", 0.05);
            insert_probability!(CASSETTE_DISTRIBUTIONS[rng.gen_range(0..CASSETTE_DISTRIBUTIONS.len())], "items", 0.05);

            if input_path.extension().unwrap() == "mp3" && !output_path.exists() {
                println!("Converting: {}", input_path.display());
                let semaphore = semaphore.clone();
                let task = tokio::spawn(async move {
                    let permit = semaphore.acquire().await.unwrap();

                    Command::new("ffmpeg")
                        .args(&["-y", "-hide_banner", "-loglevel", "error"])
                        .args(&["-i", input_path.into_os_string().into_string().unwrap().as_str()])
                        .args(&["-af", "aresample=resampler=soxr"])
                        .args(&["-ar", "44100"])
                        .args(&["-map", "0:a"]) // strip metadata...
                        .arg(output_path.into_os_string().into_string().unwrap())
                        .output().await.unwrap();

                    drop(permit);
                });

                task_pool.push(task);
            }
        }
    }

    for task in task_pool {
        task.await.unwrap()
    }

    tc_sounds.push_str("}");
    tc_music.push_str("}");

    let id = args.id;
    fs::write(format!("./playlist/media/scripts/TCGMusicScriptTCBoombox{id}.txt"), tc_music)?;
    fs::write(format!("./playlist/media/scripts/TCGSoundsTCBoombox{id}.txt"), tc_sounds)?;
    fs::write(format!("./playlist/media/lua/server/Items/TCGLoadingTCBoombox{id}.lua"), tc_loading)?;
    fs::write(format!("./playlist/media/lua/server/Items/TCGVehicleDistributions{id}.lua"), tc_vehicle_distribution)?;
    fs::write(format!("./playlist/media/lua/shared/TCGMusicDefenitionsTCBoombox{id}.lua"), tc_music_definitions)?;

    Ok(())
}