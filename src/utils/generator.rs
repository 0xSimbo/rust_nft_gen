use std::io::Write;

use crate::utils::attribute::hash_attributes;
use crate::utils::attribute::Attribute;
use crate::utils::exceptions::Exception;
use crate::utils::layer::{get_random_image_path_based_on_exception, Layer};
// use crate::utils::attribute::Attribute;
use serde::{Deserialize, Serialize};

use crate::utils::image_gen::generate;
use serde_json;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

struct Intermediary {
    trait_type: String,
    image_path: String,
}
struct ImageData {
    random_image_names: Vec<String>,
    attributes: Vec<Attribute>,
}
use num_cpus;
pub struct Generator {
    start_token_id: u32,
    end_token_id: u32,
    layers: Vec<Layer>,
    //description is a reference to a static string
    description: &'static str,
    IMAGE_PREFIX: &'static str,
    exceptions: Vec<Exception>,
}

impl Generator {
    pub fn new(
        start_token_id: u32,
        end_token_id: u32,
        layers: Vec<Layer>,
        description: &'static str,
        IMAGE_PREFIX: &'static str,
        exceptions: Vec<Exception>,
    ) -> Self {
        Self {
            start_token_id,
            end_token_id,
            layers,
            description: description,
            IMAGE_PREFIX: IMAGE_PREFIX,
            exceptions: exceptions,
        }
    }
    fn apply_exceptions(
        &self,
        random_image_intermediaries: &mut Vec<Intermediary>,
        attributes: &mut Vec<Attribute>,
    ) {
        for j in 0..self.layers.len() {
            let file_path = Path::new(&random_image_intermediaries[j].image_path);

            for exception in self.exceptions.iter() {
                if exception.target_trait == random_image_intermediaries[j].image_path {
                    let new_trait_trait_image_path =
                        get_random_image_path_based_on_exception(&exception.matching_files.clone());

                    let value = new_trait_trait_image_path.split("#").collect::<Vec<&str>>()[0]
                        .split(".")
                        .collect::<Vec<&str>>()[0];

                    let trait_type = &random_image_intermediaries
                        [exception.matching_files_render_order as usize]
                        .trait_type;

                    attributes[exception.matching_files_render_order as usize] =
                        Attribute::new(String::from(trait_type.clone()), String::from(value));
                    random_image_intermediaries[exception.matching_files_render_order as usize]
                        .image_path = new_trait_trait_image_path.to_string();
                }
            }
        }
    }
    fn generate_all_images_metadata_and_check_duplicates(&self) -> Vec<ImageData> {
        let num_images_per_layer_as_vec = self
            .layers
            .iter()
            .map(|layer| layer.num_traits)
            .collect::<Vec<u32>>();

        //FIND THE NUMBER OF POSSIBLE COMBINATIONS AS A U128
        let mut num_possible_combinations: u128 = 1;
        for num_images_per_layer in num_images_per_layer_as_vec.iter() {
            num_possible_combinations = num_possible_combinations * (*num_images_per_layer as u128);
        }
        let total_images_to_generate = self.end_token_id - self.start_token_id;
        if total_images_to_generate as u128 > num_possible_combinations {
            panic!("The number of images to generate is greater than the number of possible combinations");
        }

        let mut metadata_hashes = Vec::new();
        let mut image_data: Vec<ImageData> = Vec::new();
        let mut i = self.start_token_id;
        //loop from start id to end id
        while i <= self.end_token_id {
            let mut random_image_intermediaries: Vec<Intermediary> = self
                .layers
                .iter()
                .map(|layer| {
                    return Intermediary {
                        trait_type: layer.name.clone(),
                        image_path: layer.get_random_image_path(),
                    };
                })
                .collect::<Vec<Intermediary>>();

            let mut attributes: Vec<Attribute> = Vec::new();
            for j in 0..self.layers.len() {
                //To find the name of the trait we need to split it on # and take the first element amd then split it on . and take the first element again
                let file_path = Path::new(&random_image_intermediaries[j].image_path);
                let file_name = String::from(file_path.file_name().unwrap().to_str().unwrap());
                let trait_type = &random_image_intermediaries[j].trait_type;
                // let value = String::from("temp");
                let value = file_name.split("#").collect::<Vec<&str>>()[0]
                    .split(".")
                    .collect::<Vec<&str>>()[0];
                attributes.push(Attribute::new(
                    String::from(trait_type),
                    String::from(value),
                ));
            }

            self.apply_exceptions(&mut random_image_intermediaries, &mut attributes);

            let metadata_hash = hash_attributes(&attributes);
            if metadata_hashes.contains(&metadata_hash) {
                println!("Duplicate metadata found, regenerating");
            } else {
                let random_image_names = random_image_intermediaries
                    .iter()
                    .map(|intermediary| {
                        return intermediary.image_path.clone();
                    })
                    .collect::<Vec<String>>();

                image_data.push(ImageData {
                    // image_name,
                    random_image_names,
                    attributes,
                    // stringified_json,
                });
                metadata_hashes.push(metadata_hash.clone());
                i = i + 1;
            }
        }

        return image_data;
    }
    pub fn run_generation(&self) {
        let num_cpus = num_cpus::get() as u32;
        let description = self.description;
        let IMAGE_PREFIX = self.IMAGE_PREFIX;
        let num_cycles = (self.end_token_id - self.start_token_id) / num_cpus;
        let remainder = (self.end_token_id - self.start_token_id) % num_cpus;

        let start_token = self.start_token_id;

        let image_data = self.generate_all_images_metadata_and_check_duplicates();
        for i in 0..num_cycles {
            let mut threads: Vec<std::thread::JoinHandle<()>> = Vec::new();

            for j in 0..num_cpus {
                let curr_id = (&num_cpus * &i) + &j + start_token;
                let position_in_index = curr_id - start_token;
                let random_image_names = image_data[position_in_index as usize]
                    .random_image_names
                    .clone();
                let attributes = image_data[position_in_index as usize].attributes.clone();

                let thread = std::thread::spawn(move || {
                    generate(
                        format!("./build/images/{}.png", &curr_id).as_str(),
                        random_image_names,
                    );

                    let json_file = json!({
                        "name": format!("{} #{}",IMAGE_PREFIX,&curr_id),
                        "description": description,
                        "image": format!("ipfs://ipfsHash/{}.png",&curr_id),
                        "attributes": &attributes
                    });

                    let serialized = serde_json::to_string_pretty(&json_file).unwrap();
                    let mut file =
                        std::fs::File::create(format!("./build/json/{}.json", curr_id)).unwrap();
                    file.write(serialized.as_bytes()).unwrap();
                });
                threads.push(thread);
            }

            for thread in threads {
                thread.join().unwrap();
            }
        }

        let mut threads: Vec<std::thread::JoinHandle<()>> = Vec::new();

        //GENERATE THE REMAINDER
        let mut counter: u32 = 0;
        for i in self.end_token_id - remainder..=self.end_token_id {
            println!("i = {}", i);
            let curr_id = i;
            let random_image_names = image_data[counter as usize].random_image_names.clone();
            let attributes = image_data[counter as usize].attributes.clone();
            let thread = std::thread::spawn(move || {
                generate(
                    format!("./build/images/{}.png", &curr_id).as_str(),
                    random_image_names,
                );

                let json_file = json!({
                    "name": format!("#{} {}",IMAGE_PREFIX,&curr_id),
                    "description": description,
                    "image": format!("ipfs://ipfsHash/{}.png",&curr_id),
                    "attributes": serde_json::to_string(&attributes).unwrap(),
                });

                let serialized = serde_json::to_string_pretty(&json_file).unwrap();
                let mut file =
                    std::fs::File::create(format!("./build/json/{}.json", curr_id)).unwrap();
                file.write(serialized.as_bytes()).unwrap();
            });
            threads.push(thread);
            counter = counter + 1;
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }

    // Read JSON files, calculate rarity scores, and sort NFTs by rarity
    //Vec<(u32, f64)>
    pub fn rank_nfts_by_rarity(&self) {
        let start_token_id = self.start_token_id;
        let end_token_id = self.end_token_id;
        let mut attribute_frequencies: HashMap<String, u32> = HashMap::new();
        let mut nft_attributes: HashMap<u32, Vec<String>> = HashMap::new();
        let total_nfts = (end_token_id - start_token_id + 1) as f64;

        for token_id in start_token_id..=end_token_id {
            let json_file = fs::read_to_string(format!("./build/json/{}.json", token_id)).unwrap();
            let json_data: Value = serde_json::from_str(&json_file).unwrap();
            let attributes_as_str = json_data["attributes"].as_str().unwrap();
            let attributes: Vec<Attribute> = serde_json::from_str(attributes_as_str).unwrap();

            let mut nft_attr = Vec::new();
            for attr in attributes {
                let trait_type = attr.trait_type;
                let value = attr.value;
                let key = format!("{}:{}", trait_type, value);
                let count = attribute_frequencies.entry(key.clone()).or_insert(0);
                *count += 1;
                nft_attr.push(key);
            }
            nft_attributes.insert(token_id, nft_attr);
        }

        let mut ranked_nfts: Vec<(u32, f64)> = Vec::new();

        for (token_id, attributes) in nft_attributes {
            let rarity_score = attributes
                .iter()
                .map(|attr| {
                    let probability = *attribute_frequencies.get(attr).unwrap() as f64 / total_nfts;
                    -probability.log2()
                })
                .sum::<f64>();

            ranked_nfts.push((token_id, rarity_score));
        }

        // Sort the ranked_nfts vector by rarity score in descending order
        ranked_nfts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        //Output ranked_nfts as a JSON file of [{token_id: 1, rarity_score: 0.5}, {token_id: 2, rarity_score: 0.4}}]
        let mut output = Vec::new();
        for (token_id, rarity_score) in ranked_nfts {
            let rarity_output = RarityOutput {
                token_id,
                rarity_score,
            };
            output.push(rarity_output);
        }
        let serialized = serde_json::to_string_pretty(&output).unwrap();
        let mut file = std::fs::File::create("./build/ranked_nfts.json").unwrap();
        file.write(serialized.as_bytes()).unwrap();

        // println!("Ranked NFTs by rarity: {:?}", ranked_nfts);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RarityOutput {
    token_id: u32,
    rarity_score: f64,
}
