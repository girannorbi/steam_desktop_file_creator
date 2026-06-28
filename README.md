# steam_desktop_file_creator
A tool that automates desktop file creation for installed steam games.

# How it works
The tool looks for all installed steamapps' manifest file in all steamlibraries.
For every steamapp, the program finds the cached icon in steam's appcache and copies it to the configured path.
    default: ~/.local/share/steamapp_desktop_gen/icons/
Generates for every installed steamapp a .desktop file in the configured directory
    default: ~/.local/share/steamapp_desktop_gen/desktop/
Removes any uninstalled steamapp's desktop file from the configured path if its still present.

# Usage
- Download and build the project
- Run the executable
- Verify the config that is autogeneerated, make changes if necessary
- Let the program finish
- Move the generated .desktop files to the desired location

## The program can run as an automated script without user input, e.g: on login.
If it's run as a service, setting the DESKTOP_OUTPUT_PATH to ~/.local/share/applications/ is recommended.

# Caveats
- The dekstop file removal functionality only works if the desktop files stay in the configured path.
- All installed apps' desktop file gets generated, that includes any developer tools as well.
- The cached icons are in JPG format and transparent parts of the icons show as black.

# Thanks to:
- Copyright 2026 steam-vdf-parser contributors | https://github.com/mexus/steam-vdf-parser