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
- [x] Android
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
  - [x] See how to have score: (-> using internal_data_path)
  - Instead of back button for now using settings button.
- [x] Should the return to menu save (-> cheap to do so why not)
- [x] Add a look up info (where all the info for a country is displayed)
- [x] Prepared better multi screen
- [x] Correct bug with hint showing 
- [x] keyboard staying up when pressing a look up
- [x] Divide screen in different files ?
- [x] Re order Screen 
  - 0 Start (Chose between game modes and possibility of LookUp)
  - 1.1 Main play settings (with toggle able advanced custom with cat select)
  - 1.2 Main Play 
  - 2.1 Look up select
  - 2.2 Look up
  - 3.1 Choice play settings
  - 3.2 Choice play
- [x] Make current advanced config and make easy playing modes
- [x] Harmonize Text
- [x] Remake back button
- [x] move order and independent to start screen;
- [x] Using Arrays instead of named field 
- [x] Add a 4 choice game mode
  - [x] Screens
  - [x] memoise what has been selected (right/wrong)
  - [x] add swipe and hotkey
  - [x] add images grid
  - [x] code backend 
  - [x] add guess num count and score according
  - [x] Add country name if not in selected infos once found
- [x] Re shuffle between attempts
- [x] Catch score parse error and use reset score instead;
- [x] Add num and out_of to choices play
- [x] Improve UI
  - [x] ScrollChoice add click on arrows and better sizing
  - [x] Dedicated button for more customizations
  - [x] Harmonize sizes
  - [x] Better each Screen
  - [x] Add globals
- [x] (Main play) Remove info level to let user which ever info first
- [x] For choices play treat empty infos
- [x] No release with large svgs -> switch to file and read
- [x] Don't use constants but instead json provided info for category names
- [x] Add geographic position
  - [x] Basic done
  - [x] better edge case (islands etc)
  - [x] clean script
  - [x] See how to use files to reduce size of executable
- [x] Give number of words in hints like for UK: U... ...  and better hints
- [x] Add Region Selection somehow
- [x] Revamp Score
  - [x] Score in a separate window 
  - [x] Add stats
  - [x] Remove cca3
  - [x] Add "Score reset" pop up verif
  - [x] Add Multi profile
- [x] Change fonts and layout in Score for better readability
- [x] Add support for png/jpeg
- [x] Add categories names to json / move to csv
- [x] added Population
- [x] added Area
- [x] Add a help section with contact, sources & desc
- [x] Patch crash when at the end of play
- [x] Add score to look up
- [x] Added outline 
- [x] Try to find more precise country outlines (still could be improved for small islands)
- [x] Add a score recap at the end / toggle
- [x] Make proper icon
- [x] Add a Readme & LICENSE & Publish
- [x] Add a look up info shortcut once play is complete
- [x] `BUGFIX` Scores doesn't properly track current config when opening score screen
- [x] `BUGFIX` play score keep track in 4 choices gets reset
- [x] `SOURCES` add data that is missing -> mostly there, still a few missing but edge cases are ok 
- [x] `ANDROID` back button temp fix -> slint source modification
- [x] `ANDROID` URL Links -> webbrowser
- [x] `UI` Add special buttons with icons
- [x] `BUGFIX` Remove selected text when editing username
- [x] `UI` Score in a single screen
- [x] `SOURCES` Improve contrast for maps -> Some what but could probably be improved
- [x] `FEATURE` Add wiki link in look up
- [x] `BUGFIX` reset select look up
- [x] `ANDROID` open mailto links 
- [x] `PROJECT` Add screenshots to readme
- [x] `UI` Hide custom presets by default to avoid clutter;
- [x] `BUGFIX` show look up in learn mode (main play)
- [x] `BUGFIX` Remove custom when leaving select
- [x] `UI` Renamed Free play to flash cards
- [x] `IMPROVEMENTS` coding improvements (less clones)
- [x] `IMPROVEMENTS` Remove cloning of all countries


## Project 
- [ ] Commit data or find a way to build that is easier ?

## Bug fixes


## New features
- [ ] Add a swipe to next for look up
- [ ] Add a zoomed in version of the map that user can toggle
- [ ] Add a number data flag (is_num field on cat with Option<i32>) to do specific things for number (sort by in choices)
- [ ] Add a full input play mode 

## Android features
- [ ] follow [issue](https://github.com/slint-ui/slint/issues/8323) to patch back button

## Improvements

## Sources