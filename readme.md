
## Readme

- TODO

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