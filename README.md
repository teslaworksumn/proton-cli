# cli
Command line interface to manipulate ProtonLights projects.

## Interface

- `init <folder> <root-public-key>`: Init empty project
- `new-user <admin-key> <name> <public-key>`: Add user from public key
- `remove-user <admin-key> <name>`: Removes user from project
- `new-sequence <admin-key> <name> <music-file>`: Init a sequence
- `remove-sequence <admin-key> <name>`: Removes a sequence and deletes its files
- `id-user <private-key>`: Identify user by ssh key (public key in repo)
- `list-permissions <private-key>`: Get list of user's permissions
- `set-permission <admin-key> (add | remove) <name> <permission> [<target>]`: Change user permissions
- `resection-sequence <admin-key> <name> <num-sections>`: (Re-)Section a sequence

Permissions include:
  - edit sequence
  - edit sequence section
  - project administration
  - edit show [TODO]

## Native Dependencies

- cmake
- libssl-dev
- libcsfml-dev
- libsfml-audio2.3v5
- libcsfml-audio2.3
