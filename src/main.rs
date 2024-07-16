use std::fs::File;
use std::dbg;

use bevy::{
    prelude::*,
    //utils::HashSet,
    window::PrimaryWindow
};
/*
use bevy_mod_aseprite::{
    Aseprite,
    AsepriteAnimation,
    AsepriteBundle,
    AsepritePlugin,
    AsepriteSystems,
    AsepriteTag
};
*/

use ron::de::from_reader;
use serde::Deserialize;


#[derive(Deserialize)]
struct PlayerConfig{
    name: Name,
    pronoun_set: PronounSet
}

#[derive(Component)]
struct Player;

#[derive(Component, Deserialize)]
struct Name {
    first: String,
    family: String
}

#[derive(Component)]
enum PlayerState {
    Stand,
    Move,
    Attack
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, add_people)
        .add_systems(Update, print_names)
        .run();
}

fn spawn_camera(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0,
                window.height() / 2.0, 0.0
            ),
            ..default()
        }
    );
}

fn load_ron(
    file_path:str
) {
    let path = format!("{}/assets/config/player.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(path).expect("Failed opening file");
    let contents: PlayerConfig = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            dbg!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };

    contents
}

fn spawn_player(
    mut commands: Commands
) {
    commands.spawn((
        Player,
        config.name,
        config.pronoun_set
    ));
}





// Temporary Testing stuff
/*
enum TextCase {
    Upper,
    Lower
}

enum TextReference {
    Personal, // I, You, He, She, They, It
    PosessiveAdjective, // My, Your, His, Her, Their, Its
    PosessivePronoun, // Mine, Yours, His, Hers, Their, Its
    Reflexive // Myself, Yourself, Himself, Herself, Themself, Itself
    
}

*/

#[derive(Component, Deserialize)]
enum PronounSet {
    Female,
    Male,
    NonBinary
}

/*
fn fetch_pronoun_text(
    pronoun_set: PronounSet,
    reference: TextReference,
    case: TextCase
) {
    let pronouns = match pronoun_set {
        PronounSet::Female => (),
        PronounSet::Male => (),
        PronounSet::NonBinary => ()
    };

    let pronoun = match reference {
        TextReference::Personal => (),
        TextReference::PosessiveAdjective => (),
        TextReference::PosessivePronoun => (),
        TextReference::Reflexive => ()
    };
}
*/

fn print_names(
    person_query: Query<(&Name, &PronounSet)>
) {
    for (name, pronoun_set) in person_query.iter() {
        let pronoun = match pronoun_set {
            PronounSet::Female => "She/Her",
            PronounSet::Male => "He/Him",
            PronounSet::NonBinary => "They/Them",
        };
        println!("Name: {} {}, {}", name.first, name.family, pronoun);
    }
}

fn add_people(
    mut commands: Commands
) {
    commands.spawn((
        Person,
        Name{
            first: "Alex".to_string(),
            family: "Applegate".to_string()
        },
        PronounSet::Female
    ));
    commands.spawn((
        Person,
        Name{
            first: "Bob".to_string(),
            family: "Borscht".to_string()
        },
        PronounSet::Male
    ));
    commands.spawn((
        Person,
        Name{
            first: "Raine".to_string(),
            family: "Rivier".to_string()
        },
        PronounSet::NonBinary
    ));
}

#[derive(Component)]
struct Person;
