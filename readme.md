
## Readme

- TODO

## Building

You will need to build the UI first - as that generates a minified HTML directly embedded into the executable.

```
cd src-ui/
npm run build
```

Then

```
cd..
cargo build --release
```

## How to use in Linux

1. Install as command line program. I like `lcgsync` as name.

```
sudo cp ./target/release/local_cloud_game_sync /usr/bin/lcgsync
sudo chmod +x /usr/bin/lcgsync
```

2. Setup config

```
lcgsync init-config
lcgsync open-config
```

Example config

```json
{
  "clientName": "Bizangel Laptop",
  "sshHost": "zangelgamesyncer",
  "sshPort": 22,
  "remoteSyncRoot": "/media/game_saves",
  "syncEntries": [
    {
      "remoteSyncKey": "testsynckey",
      "saveFolderPath": "{{HOME}}/Downloads/testsave",
      "saveIgnoreGlob": ["**/*.log"]
    }
  ]
}
```

3. Test your config and ensure the tracked files are correct:

```
❯ lcgsync files testsynckey
Sync key: testsynckey
Save Folder:  /home/arcanzu/Downloads/testsave
Tracked Files:
        example_save.txt
Ignored files:
<no entries>
```

4. Test opening the user interface directly and which will automatically perform your first sync (or show you errors etc).

```
lcgsync ui testsynckey
```

Done! You can use this to sync your game saves whenever you want with a nice user interface.

The end-goal is for this to be ran automatically anytime you open your games - so see below in how to run it automatically.

# How to Wrap Steam

## How to wrap Steam in Linux - Wrapping Steam Games for Local Cloud Game Sync

1. Update your game launch command as follows.

```bash
lcgsync ui testsynckey && %command%; lcgsync ui testsynckey --after-game
```

**Note: Update the key accordingly - of course you want each game to have a different key so you will need to update your config accordingly.**

That's it - done! The script will launch before and after launching your game.

**⚠ Important You might need to use the absolute path above depending on how you're launching steam. If it is not working try absolute path first.

### Optional - Use wrapper script

To make it slightly easier and more clean you can create a wrapper script.
Download the script from `scripts/steam_wrapper/lcgsync_steam_wrapper.sh` and add it to your path.

Then you can just wrap your games in steam launch configs like this:

```bash
/scripts/bin/lcgsync_steam_wrapper.sh testsynckey %command%
```

**Note: Ensure to use absolute path for wrapper script - or don't - if you somehow got it working without it**

### ⚠ A note on SteamOS / Bazzite "Game Mode"

For no reason I could not get this to work using the above in SteamOS / Bazite under gamemode. It works fine running desktop mode with wayland but then things break on gamemode which goes against the whole point of this project.

What's probably happening: When steam runs a game in the proton shortcut then that means our custom script is also running inside the proton sandbox and this poses a lot of problems.
Home paths are broken and cannot be referenced and steam creates a nice cozy sandbox which works for the game but not for the script.
For some reason the script does - seem to run. But it's UI is not displayed and it's just buggy overall.

The best way to go from here is to handle the proton launching ourselves on a custom script.

For this see: `lcgsync_proton_wrapper.sh` get it on your bazzite OS. Update the first few lines with the right absolute paths for your system.

Then you can create a new steam shortcut pointing to the script. And you can use it as usch:

```
Target Executable: <script path>
Start In: None
Launch Options: <sync key> <game exe path to launch with proton>
```

Example:

```
Target Executable: /var/home/user/scripts/lcgsync_proton_wrapper.sh
Start In: None
Launch Options: star-renegades "/home/user/games/Star Renegades/Star Renegades.exe"
```

## How to wrap Steam in Windows - Wrapping Steam Games for Local Cloud Game Sync

1. Download the scripts in `scripts/steam_wrapper/bat_wrapper_shell_hidden.vbs` and `scripts/steam_wrapper/steam_sync_wrapper.bat`  and get them to a folder you like.
2. Modify `steam_sync_wrapper.bat` to make it point to the right executable location.ñ
3. Copy their absolute paths of both scripts.
4. Done! To add a new game to be synced before and after launch - add to the steam custom launch flags the following:

```bash
wscript "C:\UtilityPrograms\steam_sync_wrapper.bat" testsynckey %command%
```

Example:

```bash
wscript "C:\UtilityPrograms\bat_wrapper_shell_hidden.vbs" "C:\UtilityPrograms\steam_sync_wrapper.bat" testsynckey %command%
```

(If you want to add any custom flags - you can just add them after %command%)

### Note:

The need for 2 scripts instead of one is to hide the ugly batch window using VBscript hacky stuff - which allows to do this.
This can also be done with modern powershell but it seems to not fully hide the console window for some reason?

## How to Wrap Steam Shortcuts in Windows

The above will only work for steam games - not for "non-steam" game shortcuts. These work a little bit different.

You will need to instead set the shortcut normally.

Then replace the actual executable with our initial wrapper script - and instead pass it as a parameter - as the example below:

```
Target Executable: "C:\UtilityPrograms\bat_wrapper_shell_hidden.vbs"
Start In: <Should be game original start-in folder>
Launch Options: "C:\UtilityPrograms\steam_sync_wrapper.bat" star-renegades "C:\Games\Star Renegades\Star Renegades.exe"
```

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

Ensure to use right permissions if you just created it.

```bash
sudo chmod 700 /home/zangelgamesyncer/.ssh/
sudo chmod 600 /home/zangelgamesyncer/.ssh/authorized_keys
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
