# Tooler: scaffolding made easy

Create a controlled ``folder>file`` structure for your team to use as a configuration file to prevent team mattes from creating their own structures or files by hand. Keep the configuration synchronized as a source of truth for the creation of ``components``, ``views``, ``services`` or whatever in your project. Add more commands inside the configuration file as an action object.

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

# Inspiration

Creating scaffolding and folder>file structures is sometimes a headache, so I created this tool as a solution for that wasted time creating files using commands like `mkdir test && cd test && touch ...`.

# Next Steps

[x] Basic configuration
[] Directed output
[] Basing content on file creation
[] Customize the commands (not only for creation)
[] More tests

> Written and maintained by Warkanlock as PoC
````
