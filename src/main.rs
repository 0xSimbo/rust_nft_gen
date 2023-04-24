mod utils {
    pub mod attribute;
    pub mod before_runtime;
    pub mod exceptions;
    pub mod folder_searcher;
    pub mod generator;
    pub mod image_gen;
    pub mod layer;
}
// mod image_gen;
// mod my_gen;
use utils::exceptions::get_exceptions;
use utils::generator::Generator;
use utils::layer::Layer;
// mod layer;
// mod generator;

//define a static stirng called description
static DESCRIPTION: &'static str = "This is our super cool collection";
static IMAGE_PREFIX: &'static str = "NFT";
static START_TOKEN_ID: u32 = 0;
static END_TOKEN_ID: u32 = 10000;
//This will generate 101 tokens (0-100)

fn main() {
    //Ask user input if they're sure they want to generate 1000 images
    //If yes, then run the code below
    //If no, then exit the program

    // let mut input = String::new();
    // println!("Are you sure you want to generate {} images? (y/n)", END_TOKEN_ID - START_TOKEN_ID);
    // std::io::stdin().read_line(&mut input).unwrap();
    // if input.trim() != "y" {
    //     println!("Exiting program");
    //     panic!("Exiting program");
    // }

    if START_TOKEN_ID > END_TOKEN_ID {
        panic!("START_TOKEN_ID must be less than END_TOKEN_ID");
    }
    utils::before_runtime::before_runtime();
    let start_time = std::time::Instant::now();

    // let layers = vec![
    //     Layer::new(String::from("Background"), String::from("layers/Pepe/Background")),
    //     Layer::new(String::from("Body"), String::from("layers/Pepe/Body")),
    //     Layer::new(
    //         String::from("Clothes"),
    //         String::from("layers/Pepe/Clothes"),
    //     ),
    //     Layer::new(
    //         String::from("Eyes"),
    //         String::from("layers/Pepe/Eyes"),
    //     ),
    //     Layer::new(String::from("Head Accessory"), String::from("layers/Pepe/Head accessory")),
    //     Layer::new(String::from("Mouth"), String::from("layers/Pepe/Mouth")),
    // ];

    // let my_gen = Generator::new(100, 10000, layers, DESCRIPTION, IMAGE_PREFIX, get_exceptions());
    // my_gen.run_generation();

    let layers = vec![
        Layer::new(String::from("Body"), String::from("layers/Female/Body")),
        Layer::new(String::from("Face"), String::from("layers/Female/Glasses")),
        Layer::new(
            String::from("Clothing"),
            String::from("layers/Female/Clothing"),
        ),
        Layer::new(
            String::from("Half Mask"),
            String::from("layers/Female/Mask"),
        ),
        Layer::new(String::from("Hair"), String::from("layers/Female/Hat Hair")),
        Layer::new(String::from("Weapon"), String::from("layers/Female/Weapon")),
    ];

    let my_gen = Generator::new(
        100,
        200,
        layers,
        DESCRIPTION,
        IMAGE_PREFIX,
        get_exceptions(),
    );
    my_gen.run_generation();

    //----------MALE-------------
    let layers = vec![
        Layer::new(String::from("Body"), String::from("layers/Male/Body")),
        Layer::new(String::from("Face"), String::from("layers/Male/Eyes")),
        Layer::new(
            String::from("Clothing"),
            String::from("layers/Male/Clothing"),
        ),
        Layer::new(
            String::from("Half Mask"),
            String::from("layers/Female/Mask"),
        ),
        Layer::new(String::from("Hair"), String::from("layers/Male/Hair")),
        // Layer::new(String::from("Weapon"), String::from("layers/Male/Weapon")),
    ];

    let my_gen = Generator::new(
        200,
        300,
        layers,
        DESCRIPTION,
        IMAGE_PREFIX,
        get_exceptions(),
    );
    my_gen.run_generation();

    let layers = vec![
        Layer::new(String::from("Body"), String::from("layers/Male/Body")),
        Layer::new(String::from("Face"), String::from("layers/Male/Eyes")),
        Layer::new(
            String::from("Clothing"),
            String::from("layers/Male/Clothing"),
        ),
        Layer::new(
            String::from("Half Mask"),
            String::from("layers/Male/Half Mask"),
        ),
        Layer::new(String::from("Hair"), String::from("layers/Male/Hat Hair")),
        Layer::new(String::from("Weapon"), String::from("layers/Female/Weapon")),
    ];

    let my_gen = Generator::new(
        300,
        400,
        layers,
        DESCRIPTION,
        IMAGE_PREFIX,
        get_exceptions(),
    );
    my_gen.run_generation();

    let end_time = std::time::Instant::now();
    let duration = end_time.duration_since(start_time);
    my_gen.rank_nfts_by_rarity();
    println!(
        "Time taken to generate {} images: {:?}",
        END_TOKEN_ID - START_TOKEN_ID,
        duration
    );
}
