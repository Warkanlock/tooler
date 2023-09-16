# Toolers: scaffolding made easy

Establish a standardized directory and file hierarchy, serving as a centralized configuration blueprint, to ensure team-wide consistency. This will mitigate the risk of individual team members manually crafting divergent structures or files. You can use this configuration as the authoritative source for instantiating project components like ``views``, ``services``, or any other file types required. To extend functionality, enrich the configuration file with an "action object" containing additional command definitions.

![tooler](https://user-images.githubusercontent.com/13340320/155042226-f1e41bb4-c3c9-45ae-acad-d06da84e3373.gif)

# Documentation

Run `tooler --help` and as a result:

```bash
USAGE:
    tooler --command <COMMAND>

OPTIONS:
    -c, --command <COMMAND>    Run this command to create one of the the types you specified on your
                               config file [command|view|service]
    -h, --help                 Print help information
```

# How-to

1. Create your own replicable scaffolding configuration using the JSON configuration file.
2. Run `tooler` to create folders or files on demand.

# Configuration Spec

```json
{
  "commands": [
    {
      "command": "example_command", // any string that you will use as a command name later on the CLI
      "action": {
        "name": "ExampleFolder", // name of the folder about to be created
        "file_type": "folder", // type of action (folder or file, for now)
        "childs": [
            {   
                "name": "index.js", // name of the file with its format
                "file_type": "file", 
                "example" : "import React from 'React'", 
                "childs" : [],
            }
        ]
      }
    },
  ]
}

```

# Setup

To run the program, you need to specify two environment variables

Configuration path (you can point to different paths)
```
export CONFIG_PATH=./src/config/example.config.json
```

Directory output for commands (folder must exist before running the bases)
```
BASE_PATH=./src/target/
```

# Inspiration

Creating scaffolding and folder>file structures is sometimes a headache, so I created this tool as a solution for that wasted time creating files using commands like `mkdir test && cd test && touch ...`

# Next Steps

- [x] Basic configuration
- [ ] Directed output
- [ ] File creation with example content
- [ ] Customize commands (not only for creation)
- [ ] More tests
