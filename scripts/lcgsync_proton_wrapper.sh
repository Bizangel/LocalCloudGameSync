#!/bin/bash
# ================================================
# Proton Game Launcher with lcgsync Pre/Post Sync
# ================================================
# Usage: proton_wrapper.sh <game-key> <game-executable-path>
# Example: proton_wrapper.sh "star-renegades" "/home/user/Games/Star Renegades/Star Renegades.exe"
# ================================================

set -euo pipefail

# --- Configuration ---
REAPER_PATH="/var/home/user/.local/share/Steam/ubuntu12_32/reaper"
# What is steam runtime? https://gitlab.steamos.cloud/steamrt/steamrt/-/blob/steamrt/sniper/README.md
STEAM_RUNTIME_PATH="/var/home/user/.local/share/Steam/steamapps/common/SteamLinuxRuntime_sniper/_v2-entry-point"
LCGSYNC_PATH="/var/home/user/.local/bin/lcgsync"
PROTON_PATH="$HOME/.steam/steam/steamapps/common/Proton 9.0 (Beta)/proton"
LOGFILE="/tmp/lcgsync_proton_wrapper.log"


# --- Logging Setup ---
exec > >(tee -a "$LOGFILE") 2>&1

echo "=== Proton Wrapper started at $(date '+%Y-%m-%d %H:%M:%S') ==="

# --- Argument Validation ---
if [[ $# -lt 2 ]]; then
    echo "Usage: $0 <game-key> <path-to-executable>"
    exit 1
fi

GAME_KEY="$1"
GAME_EXE="$2"

if [[ ! -f "$GAME_EXE" ]]; then
    echo "Error: Executable not found: $GAME_EXE"
    exit 1
fi

# --- Change Directory to Game Folder ---
GAME_DIR="$(dirname "$GAME_EXE")"
cd "$GAME_DIR" || {
    echo "Error: Failed to change directory to $GAME_DIR"
    exit 1
}
echo "Changed directory to: $(pwd)"
echo ""

# --- Extract Steam Shortcut ID ---
if [[ -z "${STEAM_COMPAT_MEDIA_PATH:-}" ]]; then
    echo "Error: STEAM_COMPAT_MEDIA_PATH not set. This must be provided by Steam."
    exit 1
fi

if [[ "$STEAM_COMPAT_MEDIA_PATH" =~ /([0-9]{10})/ ]]; then
    SHORTCUT_ID="${BASH_REMATCH[1]}"
else
    echo "Error: Could not extract shortcut ID from STEAM_COMPAT_MEDIA_PATH"
    exit 1
fi

# --- Proton Compatibility Data Setup ---
export STEAM_COMPAT_CLIENT_INSTALL_PATH="$HOME/.local/share/Steam"
export STEAM_COMPAT_DATA_PATH="$HOME/.steam/steam/steamapps/compatdata/$SHORTCUT_ID"

echo "Using compatdata prefix: $STEAM_COMPAT_DATA_PATH"
echo "Game key: $GAME_KEY"
echo ""

# --- Pre-Sync ---
echo "[PRE-SYNC] Running lcgsync..."
if "$LCGSYNC_PATH" ui "$GAME_KEY"; then
    echo "[PRE-SYNC] Success."
else
    echo "[PRE-SYNC] Failed — aborting game launch."
    exit 1
fi
echo ""

# --- Game Execution ---
echo "[GAME] Launching via Proton..."
if "$REAPER_PATH" SteamLaunch AppId=$SHORTCUT_ID -- $STEAM_RUNTIME_PATH --verb=waitforexitandrun -- "$PROTON_PATH" waitforexitandrun "$GAME_EXE"; then
    echo "[GAME] Exited successfully."
    GAME_EXIT=0
else
    GAME_EXIT=$?
    echo "[GAME] Exited with code $GAME_EXIT"
fi
echo ""

# --- Post-Sync ---
echo "[POST-SYNC] Running lcgsync after game..."
if "$LCGSYNC_PATH" ui "$GAME_KEY" --after-game; then
    echo "[POST-SYNC] Success."
else
    echo "[POST-SYNC] Warning: post-sync failed (exit code $?)."
fi
echo ""

echo "=== Script completed at $(date '+%Y-%m-%d %H:%M:%S') ==="
echo "Final exit code: $GAME_EXIT"
echo "Log file: $LOGFILE"
echo "========================================"

exit $GAME_EXIT
