# cli
Command line interface to manipulate ProtonLights projects.

## Requirements

- Get list of editable files for a given user
- Identify user by ssh key (public key in repo)
- Init empty project
- Init a sequence
- (Re-)Section a sequence
  - On init, section as section1.
    - Number each section, and don't delete.
    - Use patch to copy changes.
  - Use git --find-renames=100%?
- Add user from public key
- Give permission to user to
  - edit sequence/section
  - update metadata
  - edit permissions
  - edit show
