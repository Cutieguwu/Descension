use bevy::prelude::*; //Essential dependencies from Bevy

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .run();
}

struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, print_names);
    }
}

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

#[derive(Component)]
struct Name {
    first: String,
    family: String
}

#[derive(Component)]
enum PronounSet {
    Female,
    Male,
    NonBinary,
}
