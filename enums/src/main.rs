fn main() {
    let idle_state = GollumStates::Idle;
    idle_state.state_represent();

    let fighting_state = GollumStates::Fighting;
    fighting_state.state_represent();

    let collecting_state = GollumStates::Collecting(200);
    collecting_state.state_represent();

    let defending_state = GollumStates::Defending;
    defending_state.state_represent();
}

enum GollumStates {
    Idle,
    Fighting,
    Collecting(u32),
    Defending
}

impl GollumStates {
    fn state_represent(&self) {
        match self {
            GollumStates::Idle => println!("Gollum is resting and muttering to himself."),
            GollumStates::Fighting => println!("Gollum is Fighting!"),
            GollumStates::Collecting(amount) => println!("Gollum is Collecting {} coins.", amount),
            GollumStates::Defending => println!("Gollum is Defending his treasure!")
        };
    }
}
