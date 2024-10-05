# DiscordLoops (Discord Rich Presence Intergration for FL Studio)

thingy written in rust that displays what project youre working on in fl studio and additionally counts the amount of otts youve got open

## CURRENTLY WIP!!!!!

## Config Explanation
- `project_format` is how the text about the project is displayed. `%%` is replaced by the project name
- `plugin_format` is how the text about plugins is displayed. `%x` is replaced by the amount of plugins and `%y` is replaced by the plugin name (which is the `plugin` field)
- `plugin` is the plugin name, which is checked against in the code (case sensitive!!!)
- `update_rate` is how often the rich presence is updated in seconds

### TODO
- [x] add config file
- [x] choose what plugins to display
- [ ] add comments to the code
- [ ] add button system
- [ ] make a tray icon