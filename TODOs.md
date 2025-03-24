# TODOs

## Done 
- [x] Base UI
- [x] Info parsing
- [x] Score saving/reading
- [x] Separate file for Deserialize struct
- [x] Add parameter easy to show well know flags first
- [x] Add learn mode
- [x] Add a first letter hint on ui (for country and capital)
  - kinda dumb because as of yet no string manipulation possible on slint so have to do it from rust...
- [x] More of the load and save on info_parse
- [x] Using slint struct to simplify updating ?
- [x] Make order parameterized to chose what you first see
- [x] Make more info actually take more info
- [x] Way to change infos displayed (maybe precompute once depending)
- [x] Improve Hints for multi possibilities (Y..., X...)
- [x] Add a hard mode with all flags?
- [x] Make a way to return to main menu
- [x] Make a mode for country outline ?
- [x] Improve UI on Settings screen
  - [x] Make a dedicated widget with animation to make it flashy
- [x] Harmonize Theme & fonts
- [x] Make custom combobox (more straight forward for categories)
- [x] Simplify score icon by using colorize
- [x] Add a previous score / score animation on hover
- [x] Add a Reset score button
- [ ] Android
  - [x] Install Android NDK & SDK 
  - [x] building with x build 
  - [x] See how to use release and not debug 
  - [x] Include Files 
    - Using single precomputed json and include str 
  - [x] blocked orientation using manifest.yaml
  - [x] Add AppIcon (-> via manifest.yaml)
  - [x] See how to detect swipes (-> basic impl with SwipeGestureHandler)
  - [x] Optimize size (-> should be ok with --release)
  - [x] Large text (EG on many languages) is not handled (-> now just one text element so will take more space if needed)
  - [x] Hint resizing makes text wrapping not great, should have hint outside of Horizontal box to avoid
  - [x] Reposition score just below image (easier to use)
  - [x] See how to have score: https://docs.rs/android-activity/latest/android_activity/struct.AndroidApp.html#method.internal_data_path

## Priority 
- [ ] Make android app
  - [ ] See how to catch back button (https://docs.rs/android-activity/latest/android_activity/input/enum.Keycode.html#variant.TvMediaContextMenu)

## Bonus
- [ ] Dedicated button for more customizations
- [ ] Make proper icon
- [ ] Animate (WIP)
  - [ ] Better scrolling on categories : maybe use timer to snap to center? 
  - [ ] Animate between screens? 
- [ ] Add a 4 choice game mode
- [ ] Add a look up info (where all the info for a country is displayed)

## Ideas to discuss
- [ ] Should the return to menu save ?
- [ ] Score specific for categories chosen ?
- [ ] Add a way to add more infos ?
- [ ] Add a Disabled hint ?