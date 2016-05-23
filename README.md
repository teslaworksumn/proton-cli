# cli
Command line interface to manipulate ProtonLights projects.

## Interface

- `init <folder>`: Init empty project
- `new-user <public> <name>`: Add user from public key
- `id-user <private>`: Identify user by ssh key (public key in repo)
- `list-editable`: Get list of editable files for a given user
- `allow [TODO]`: Give permission to user to
  - edit sequence/section
  - update metadata
  - edit permissions
  - edit show
- `new-sequence [TODO]`: Init a sequence
- `resection-sequence [TODO]`: (Re-)Section a sequence
  - On init, section as section1.
    - Number each section, and don't delete.
    - Use patch to copy changes.
  - Use git --find-renames=100%?

## Native Dependencies

- cmake
- libssl-dev
