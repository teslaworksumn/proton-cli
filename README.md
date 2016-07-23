# cli
Command line interface to manipulate ProtonLights projects.

## Interface

- `init <folder> <root-public-key>`: Init empty project
- `new-user <admin-key> <name> <public-key>`: Add user from public key
- `new-sequence <admin-key> <name> <music-file>`: Init a sequence
- `remove-sequence <admin-key> <name>`: Removes a sequence and deletes its files
- `id-user <private-key>`: Identify user by ssh key (public key in repo)
- `list-permissions`: Get list of all available permissions
- `set-permission <admin-key> (add | remove) <name> <permission> [<target>]`: Change user permissions
- `list-editable [TODO]`: Get list of editable files for a given user
- `resection-sequence [TODO]`: (Re-)Section a sequence
  - On init, section as section1.
    - Number each section, and don't delete.
    - Use patch to copy changes.
  - Use git --find-renames=100%?
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
