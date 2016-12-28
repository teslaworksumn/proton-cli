# cli
Command line interface to manipulate ProtonLights projects.

## Interface

- `new-project <name> <layout-id>`
- `new-user <admin-key> <name>`
- `remove-user <admin-key> <uid>`
- `new-sequence <admin-key> <name> <music-file> <seq-duration> <layout-id>`
- `new-vixen-sequence <admin-key> <name> <music-file> <seq-duration> <frame-duration> <data-file> <- `layout-id>`
- `add-sequence <admin-key> <proj-name> <seqid>`
- `remove-sequence <admin-key> <proj-name> <seqid>`
- `delete-sequence <admin-key> <seqid>`
- `get-sequence <seqid>`
- `get-playlist-data <proj-name>`
- `set-sequence-layout <admin-key> <seqid> <layout-id>`
- `new-layout <layout-file>`
- `patch-layout <admin-key> <layout-id> <patch-file>`
- `new-section <admin-key> <t_start> <t_end> <seqid> <fixid>..`
- `get-user-id <public-key>`
- `get-layout-id <proj-name>`
- `list-permissions <uid>`
- `set-permission <admin-key> (add | remove) <uid> Administrate`
- `set-permission <admin-key> (add | remove) <uid> EditSequence <target-sequence>`
- `set-permission <admin-key> (add | remove) <uid> EditSection <target-sequence> <target-section>`
- `set-permission <admin-key> (add | remove) <name> EditSeqSec <target-section>`

Permissions include:
  - project administration
  - edit sequence [TODO]
  - edit sequence section [TODO]

## Native Dependencies

- cmake
- libssl-dev
- libsfml-dev
- libcsfml-dev
- postgresql (version 9.5 works for sure)

## Setting up the database

Install postgresql  
`$ sudo apt install postgres`

Set password of postgres user (can be anything. You won't be able to see the password while you type it)  
`$ sudo passwd postgres`

Change to postgres user and start the server  
`$ su - postgres`  
`$ psql`

Set psql's postgres password (can/should be different than the other password)  
`# \password postgres`

Create proton user (password used by cli, so keep the same)  
`# CREATE USER proton WITH PASSWORD '1234qwermnbv'`

Create database  
`# CREATE DATABASE proton_cli`

Quit psql  
`# \q`

Load in database structure  
`$ psql proton_cli < /path/to/proton-cli/db_backups/working_xx_p`

Done, so exit su  
`$ exit`
