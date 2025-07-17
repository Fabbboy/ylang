# Sable 
Sable is a programming language written in rust that is aiming to be fast and low-level.

## State
Sable is not frozen just a intelectual break. We got parsing of variable declarations, assignments and function declarations done. Next step would be to start implementing the semantic analyzer or try another shot on the highlevel-IR although the last one failed. If I had to start today I would go for a Swift approach because the codebase is easier to read and understand than rust's but do as you please.

Also don't try to split the input codebase into crates or packages just go for a swift/zig approach where the project does not have a fixed architecture it's way easier and makes the compiler implementation way more flexible just go for a `sablec a.sable b.sable c.sable` and then create a module that holds all these ast's and then just build up a lookup table

## a Sable codebase (heavily WIP)
```
<project_name>
 | project.json
``` 

That's it... 
project.json:
```json5
{
    // "name": "Sable-Selfhosted", // name in this case not allowed because its a workspace
    "type": "workspace",  // exe, lib, workspace are valid types
    // "modules": [], // modules contained in this project. Not allowed because its a workspace
    "members": [
        "sable-parser", // sable projects containing their own project.json
        "sable-analyzer", 
    ],
    "dependencies": { // dependencies for modules or if workspace then workspace dependencies
        "git": [], // sable projects pulled from github
        "local": [
            "glibc",
        ], // sable projects pulled from path
        "sys": [], // static or dyn libraries pulled from system
        "inherit": [] // dependencies of any kind inheritated from parent
    }
}
```

workspace member project.json:
```json5
{
   "name": "sable-parser",
   "type": { "kind": "lib", "linkage": ["static", "shared"]},
   "modules": [
    "lexer",
    "parser",
   ],
   "dependencies": {
    "inherit" : [
        "glibc"
    ]
   }
}
```