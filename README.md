# alacritty-wal
Tool written in Rust to update/export [wal](https://github.com/dylanaraps/pywal) colors for alacritty.

## Usage:
Running the tool reads the colors from `~/.cache/wal/colors`, so make sure to run it after [wal](https://github.com/dylanaraps/pywal) colors have been created.
It updates the config with the new color values automatically by default, but can be made to write to a separate file instead (`~/.config/alacritty/colors.yml`), which can be used with your original `alacritty.yml` to create a new one. 

## Example:
After running wal do:
```
alacritty-wal
```
And your alacritty config should be updated.

### Manual mode
If you want to keep the comments in your alacritty config, or for some other reason you want to manually "merge" the colors 
sections and the other sections, you can use manual mode to create a seperate `colors.yml` file instead.
```
alacritty-wal -m

# make sure to rename your normal config to 'settings.yml' and make sure there are no mentions of the color section in it
cat ~/.config/alacritty/settings.yml ~/.config/alacritty/colors.yml > ~/.config/alacritty/alacritty.yml
```
