# DiscordLoops (Discord Rich Presence Intergration for FL Studio)

## CURRENTLY WIP!!!!!

## Config Explanation
- `project_format` is how the text about the project is displayed. `%%` is replaced by the project name
- `plugin_format` is how the text about plugins is displayed. `%x` is replaced by the amount of plugins and `%y` is replaced by the plugin name (which is the `plugin` field)
- `plugin` is the plugin name, which is checked against in the code (case sensitive!!!)
- `update_rate` is how often the rich presence is updated in seconds
- `app_id` is the discord app id (controls what is displayed on your profile)

## Note
this app keeps its config file and logs in the `%APPDATA%/discordloops` directory unless the `config.json` file is in the same directory as the executable

### TODO
- [x] add config file
- [x] choose what plugins to display
- [ ] add button system
- [x] make a tray icon
- [x] embed icon into executable
- [x] generate config by itself and put it in appdata
- [x] logging system
