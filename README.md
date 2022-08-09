# exptext
A tool to retrieve all text from a ExplorerScript file from SkyTemple
```
USAGE:
    exptext [OPTIONS] [FILES]...

ARGS:
    <FILES>...    

OPTIONS:
    -d, --db <DB_PATH>           The path to actor json database [default: actors.json]
    -f, --no-format              Do not format the dialogue of original script
    -h, --help                   Print help information
    -l, --language <LANGUAGE>    Filter by language (ex: english, italian)
    -n, --number                 Number the text at the line number it appears
    -s, --sort                   Sort the dialogue by message type
    -V, --version                Print version information
```

## Overview

Let say we have these lines in an ExplorerScript file:
```rust
message_SetFace(ACTOR_NPC_DAGUTORIO, FACE_NORMAL, FACE_POS_STANDARD);
message_Talk({
    english=" Gwah![K] Gwargh![K] Gwagagah! [hero], [c_kind:NPC_ZUBATTO]",
    french=" Gwah![K] Gwaaaah![K]\nGwaaaaahgaaaaah! [hero], [c_kind:NPC_ZUBATTO]",
    german=" Gwah![K] Gwargh![K] Gwagagah! [hero], [c_kind:NPC_ZUBATTO]",
    italian=" Uh![K] Argh![K] Aaaargh! [hero], [c_kind:NPC_ZUBATTO]",
    spanish=" ¡Ah![K] ¡Aaah![K] ¡Aaaaaaaah! [hero], [c_kind:NPC_ZUBATTO]",
});
```

Then this would output:
```
[Dugtrio]: Gwah! Gwargh! Gwagagah! Muttski, Zubat
[Dugtrio]: Gwah! Gwaaaah! Gwaaaaahgaaaaah! Muttski, Zubat
[Dugtrio]: Gwah! Gwargh! Gwagagah! Muttski, Zubat
[Dugtrio]: Uh! Argh! Aaaargh! Muttski, Zubat
[Dugtrio]: ¡Ah! ¡Aaah! ¡Aaaaaaaah! Muttski, Zubat
```

Ultimately, ``ACTOR_NPC_DAGUTORIO`` is converted to ``Dugtrio`` as the speaker while `[hero]` and `[c_kind:NPC_ZUBATTO]` are substituted with `Muttski` and `Zubat` respectively. 

We can see that these substitutions can be found in ``actors.json``:
```json
{
   "special":{
        "hero":"Muttski",
        "partner":"Dinsdale",
        "team":"Poképals"
   },
   "actors":[
       ...
       {
            "id":"NPC_ZUBATTO",
            "name":"Zubat",
            "kind":"Zubat"
       },
       ...
       {
            "id":"NPC_DAGUTORIO",
            "name":"Dugtrio",
            "kind":"Dugtrio"
       },
       ...
   ]
}
```

The CLI handles other cases such as `message_SwitchTalk` or when the actor is reset. 

## Usage
To reflect the current state of your ROM, edit `actors.json` by adding new actors, deleting unnecessary actors in the array, and editing the names of your player, partner, and team.

## TODOs
- [ ] Clean up code
- [x] Add unit tests 
- [ ] Fix issue with incorrect dialogue by shifting from Regex implementation to using a lexer. 