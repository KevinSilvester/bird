# Bird

An installation organizer of sorts.

### Install
```bash
curl -s "https://raw.githubusercontent.com/KevinSilvester/bird/main/install.sh" | bash
```

***

## TODO:
- [ ] WRITE TESTS!!!!
- [ ] Add skip pre-install option to install command
- [ ] Add force option to install, update and uninstall
- [ ] Create 'init' command to generate eggs file
- [ ] Create 'add' command to add program to eggs file
   - [ ] Add all instruction (install, pre-install, update, uninstall) interactively
   - [ ] Add specific commands
   - [ ] Skip adding specific command
   - [ ] Run specific command after adding
- [ ] Create edit command to edit specific commands for specific program
- [ ] Create find command to allow fuzzy find for program names
- [ ] Implement logging
- [ ] Update json schema and parser for eggs file
   - [ ] Allow eggs file to be accessed using url. For example when installing run `bird install --url https:\\gist.github.com\xyxyxyxyxxz\xxyxyxy --all` 
   - [ ] Allow specific command to stored in url. For example 
      ```json
      {
         "name": "go",
         "install": {
            "url": "https://pastebin.com/raw/xd5JHjra",
            "steps": ["echo '`url` will not be read/run if `steps` are present'"],
            "clean_up": ["echo 'optional clean up commands'"],
            "shell": "fish"
         },
         ...
      }
      ```
   - [ ] Create global/default commands to run for install/update/uninstall
   - [ ] Group programs installed with same package managers together
   - [ ] Allow a default shell to run commands
   - [ ] Allow for the shell option to be specific to command/program
- [ ] Update nest format
   - [ ] Store nest files/folders in 'XDG_DATA_HOME'
   - [ ] Main nest file's json schema should remain same. But a separate log/receipt file should be generated for each program. This file should contain the info such as which command was run (install, update, etc.), any warnings/errors that resulted in running that command, the time the command was run and the time it took run the command.
- [ ] Improve error handling.
- [ ] Improve help messages.
- [ ] Improve installation steps.
- [ ] Create scoop bucket for windows machines.
- [ ] Create homebrew/linuxbrew formula for *nix machines.
