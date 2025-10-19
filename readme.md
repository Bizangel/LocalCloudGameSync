
## Readme

- TODO


## How to use in Windows - Wrapping Steam Games for Sync Cloud

1. Download scripts in `scripts/windows_steam` and get them to a folder you like.
2. Copy their absolute paths of both scripts.
3. Done! To add a new game to be synced before and after launch - add to the steam custom launch flags the following:

```bash
wscript <> "C:\UtilityPrograms\steam_sync_wrapper.bat" testsynckey %command%
```

Example:

```bash
wscript "C:\UtilityPrograms\bat_wrapper_shell_hidden.vbs" "C:\UtilityPrograms\steam_sync_wrapper.bat" testsynckey %command%
```

(If you want to add any custom flags - you can just add them after %command%)

### Note:

The need for 2 scripts instead of one is to hide the ugly batch window using VBscript hacky stuff - which allows to do this.
This can also be done with modern powershell but it seems to not fully hide the console window for some reason?

# Limitations

- Remote Server is always expected to be a compliant linux machine with bash.
- Remote path is limited to alphanumeric and -_/ characters.

- UI is currently configured to come after errors config and parsing. So there's only command line output feedback instead of UI-feedback if the game config is misconfigured.
- - This can lead to some frustrating behaviors - if trying to create an steam wrapper - and synckey is wrong it will just fail silently. (As cmd will be hidden)


- Current remote SSH logic does not check remote saves. This means we trust that REMOTE_HEAD will always accurately represent the state of the remote save.
- - If you wish to modify the save files directly on the remote - please update the REMOTE_HEAD hash accordingly (Not recommmended).
- - If you wish to modify save files - simply modify them on a local client - then use the client to push to the remote (Recommended).

- Current ignore glob filters are not respected during when pulling from remote.
- - This means If a client accidentally pushed a `.log` file - this will be included in your folder in the next pull.
- - This shouldn't affect sync logic as the `.log` will still be ignored when calculating the hash - but if this happen for more important files like `.cfg` files it might override local config accidentally.

# Setup for SSH Save user:


Create user - remove password login for user

```bash
sudo adduser --disabled-password zangelgamesyncer
sudo passwd -l zangelgamesyncer
```

Create `.ssh` folder to allow authorized keys.

```bash
sudo mkdir /home/zangelgamesyncer/.ssh
```

Create authorized keys and copy-paste your SSH key.
I personally created a separate one - simply specify name when using `ssh-keygen`
Then I can specify it as such in `.ssh/config`

```config
Host zangelgamesyncer
  HostName <hostname>
  User zangelgamesyncer
  IdentityFile ~/.ssh/local_save_ssh_identity
  IdentitiesOnly yes
```

Then copy the public keys to authorized keys

```bash
sudo micro /home/zangelgamesyncer/.ssh/authorized_keys
```

Create the required folder you want manually - in my case I use:

Also creating a restic password.

```
sudo mkdir /media/game_saves/
sudo mkdir /media/game_saves/.cloudmeta
sudo micro /media/game_saves/restic_password
```

And assign permissions:

```
sudo chown zangelgamesyncer:zangelgamesyncer -R /media/game_saves
```

Done!

(commands above not verified - just for reference)
