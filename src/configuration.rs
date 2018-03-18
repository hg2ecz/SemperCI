use std::vec::Vec;

struct Congiguration {
    repo_path: String,
    branches: Vec<Branches>
}

struct Branches {
    name: String,
    last_known_commit: String,
    description: String,
    build_definitions: Vec<BuildDefinitions>
}

struct BuildDefinitions {
    name: String,
    description: String,
    steps: Vec<Steps>
}

struct Steps {
    name: String,
    description: String,
    command: String,
    may_fail: Option<bool>
}


fn read_configuration() ->