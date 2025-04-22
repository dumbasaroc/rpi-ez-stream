# Folder Layout

```
/root
  - res/
  | - modules/
  | | - <MODULE NAME>/
  | | | - characters/
  | | | | - <CHARACTER_NAME>
  | | | | | - css.png ----> The image shown on the CSS
  | | | | | - render<X>.png --> The image shown on the
  | | | | |                     main screen. The <X> is
  | | | | |                     a zero-indexed costume
  | | | | |                 index.
  | | | | | - icon<X>.png ----> The image shown in the
  | | | | |                     costume selection bar.
  | | | | |                     <X> follows the same
  | | | | |                     rules as above.
  | | | | - <CHARACTER_NAME_2>
  | | | | - <...>
  | | | - module_info.json --> See below 
```

# `module_info.json` Specification

```json
{
    "module_name": "The display name of this Module",

    "default_character": "The 'path_in_folder' value of a character in 'character_data' - used to define which character is selected by default when activating this module.",

    "character_data": [
        // This is a list of Character objects,
        // defined in the order in which they
        // should appear in the CSS. The specification
        // is as follows (example values are for Mario
        // in SSBU):
        {
            // A display name, used as the primary
            // alias for a character in the CSS
            "display_name": "Mario",

            // The directory inside of "characters"
            // where the image data for this
            // Character resides.
            "path_in_folder": "mario",

            // The number of alternate costumes
            // this character has - this should
            // mean that
            // "modules/<modname>/characters/
            // <path_in_folder>" should
            // contain "render0.png" through
            // "render<X-1>.png" and "icon0.png"
            // through "icon<X-1>.png", where X
            // is this value.
            "num_costumes": 8,

            // A list of common aliases for this
            // character, used in the Search Bar
            // on the CSS. Can be an empty list.
            "aliases": [ "Plumber", "Jumpman" ],
        },
        // <...>
    ]
}
```
