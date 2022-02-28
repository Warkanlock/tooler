# Tooler: scaffolding made easy

Create a controlled ``folder>file`` structure for your team to use as a configuration file to prevent team mattes from creating their own structures or files by hand. Keep the configuration synchronized as a source of truth for the creation of ``components``, ``views``, ``services`` or whatever in your project. Add more commands inside the configuration file as an action object.

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

In order to run the program you need to specify two environment variables

Configuration path (you can point to different paths)
```
export CONFIG_PATH=./src/config/example.config.json
```

Directory output for commands (folder must exists before running the commands)
```
BASE_PATH=./src/target/
```

# Inspiration

Creating scaffolding and folder>file structures is sometimes a headache, so I created this tool as a solution for that wasted time creating files using commands like `mkdir test && cd test && touch ...`

# Next Steps

- [x] Basic configuration
- [ ] Directed output
- [ ] Basing content on file creation
- [ ] Customize the commands (not only for creation)
- [ ] More tests

> Written and maintained by Warkanlock as PoC
