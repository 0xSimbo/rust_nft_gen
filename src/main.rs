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
static END_TOKEN_ID: u32 = 100;
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
    let layers = vec![
        Layer::new(
            String::from("Backgrounds"),
            String::from("layers/Background"),
        ),
        Layer::new(
            String::from("Bottom Lid"),
            String::from("layers/Bottom lid"),
        ),
        Layer::new(String::from("Eye Color"), String::from("layers/Eye color")),
        Layer::new(String::from("Eyeball"), String::from("layers/Eyeball")),
        Layer::new(String::from("Goo"), String::from("layers/Goo")),
        Layer::new(String::from("Iris"), String::from("layers/Iris")),
        Layer::new(String::from("Shine"), String::from("layers/Shine")),
        Layer::new(String::from("Top Lid"), String::from("layers/Top lid")),
    ];

    let my_gen = Generator::new(
        START_TOKEN_ID,
        END_TOKEN_ID,
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
