# Sable 
Sable is a programming language written in rust that is aiming to be fast and low-level.

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