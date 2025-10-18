
## Readme

- TODO

# Limitations

- Remote Server is always expected to be a compliant linux machine with bash.
- Remote path is limited to alphanumeric and -_/ characters.

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