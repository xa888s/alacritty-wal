# alacritty-wal
Tool written in Rust to export wal colors for use in alacritty

## Usage:
Running the tool reads the colors from `~/.cache/wal/colors`, so make sure to run it after pywal colors have been created.
It writes the output to `~/.config/alacritty/colors.yml`, which can be used with your original `alacritty.yml` to create a new one. 

### Example:
After running wal do:
```
alacritty-wal

# make sure to rename your normal config to 'settings.yml' and make sure there are no mentions of the color section in it
cat ~/.config/alacritty/settings.yml ~/.config/alacritty/colors.yml > ~/.config/alacritty/alacritty.yml
```
