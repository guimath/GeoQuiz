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
- [ ] Android
  - [x] Install Android NDK & SDK 
  - [x] building with x build 
  - [x] See how to use release and not debug 
  - [x] Include Files 
    - Using single precomputed json and include str 

## Priority 
- [ ] Make custom combobox (more straight forward for categories)
- [ ] Make android app
  - [ ] Optimize size
  - [ ] See how to block orientation and catch back button
  - [ ] See how to detect swipes
  - [ ] Re organize 
  - [ ] Add AppIcon
  - [ ] See how to have score: https://docs.rs/android-activity/latest/android_activity/struct.AndroidApp.html#method.internal_data_path

## Bonus
- [ ] Dedicated button for play
- [ ] Animate (WIP)
- [ ] Add a previous score / score animation on hover

## Ideas to discuss
- [ ] Should the return to menu save ?
- [ ] Score specific for categories chosen ?
- [ ] Add a way to add more infos ?
- [ ] Add a Disabled hint ?