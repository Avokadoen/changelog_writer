# Changelog Writer
The goal of the changelog writer is to generate a structured changelog for different file formats either creating the file (initialize) or updating existing files

The program should have a simple argument structure and repository specific settings should be defined in a ChangelogWriterConfig.json in the root of the repo

# Current Progress
- [x] Parse arguements
- [x] Parse file settings
- [x] Validate file to argument sanity 
- [] Retrieve git commit history
    - [] Format git hostory to valid md
    - [] Format git hostory to valid xml
    - [] Format git hostory to valid html
    - [] Format git hostory to valid ---
- [] Send correct file to correct config path

# Specifications
## command and arguments
USEAGE:

changelogwriter [OPTIONS]

OPTIONS:
<pre>
-u | --upgrade          case insensitive.
                        the version step you want to take
                        can be any of:
                        of types defined in repo config file

-i | --init             creates a new config file from default
                        (?maybe insert git hook to check for valid
                        commit message or is it too intrusive?)
</pre>



```bash
example:
user@user:~/Projects/my_repo$ changelogwriter -u major ./
```

The changelog writer needs to know how big you think the update is. this is the version format which can be (major | minor | lesser) you can also combine any of these. Not supplying any will result in error unless a default has been set in the config file.  

## config file example
```json
{
    "defaultUpgrade": "minor",
    "versionTypes": [ 
        { "major": "Ma" },
        { "minor": "Mi" },
        { "lesser": "Le" }
    ],
    "versionFormat": "MaMa.MiMi.LeLe",
    "targetFilePaths": [ 
        "./something/somthing/changelog.xml", 
        "./something/somthing2/changelog.xml",
        "./changelog.md"
    ],
    "categories": [
        "bugfix",
        "feature",
        "technical",
        "tests"
    ],
    "appendPosition": "top"
}
```
**default upgrade** *optional* field to set default upgrade

**version types** allow you to create your own version system

**version format** dictates your version structure where "Ma" = major, "Mi" = minor, "Le" = lesser.

**Target file paths** are the paths to your changelogs that you want the changelog writer to work on. 

**Categories** can be anything you want them to be. If you want a commit to be included, then you need to include cat:*category* in your commit message.

**append position** can be either "top" or "bottom" and dictates how the file is structured

toml file should be considered as that is what cargo uses

## changelog formats to support 
- xml
- html
- md
- json 
- ...?

## Valid commit message
Any commit message that is formatted wrong will be ignored (in the future it should be warned when commiting that rules has not been followed). 

When commiting you should tag any sentence relevant to a category as such. Example
```
git commit -m "cat:category 'did the thing'" 
```
This will result in next changelog generation to write
```
*category*
 - did the thing
```

## Resulting action
When you ask the changelog writer to update any changelog it should read the previous version and use this to build a new version number. Mismatch between files should be reported as an error.

The writer should then based on either git history (commit messages) or some unknown mechanic create a list of changes which it categories based on categories defined in config.

The resulting list and version should then be converted to a string and appended to current changelogs. If there is no file it should give it a start version number and create the files.

The file should then be the following: 
```md
*Project name* *current version number* (*date of generation*)
*git commit hash from* .. *git commit has to*

*category 1*
 - *correct commit message1*
 - *correct commit message4*
 - *correct commit message5* 

 *category 2*
 - *correct commit message2*
 - *correct commit message3*

*Project name* *prev version number* (*date of prev generation*)
*git commit hash from* .. *git commit has to*
....
```

# why am i doing this
For personal use i would find this system helpful in allowing me to maintain any changelog and also multiple in different formats in the same project (i.e a webpage that has a public html changelog and a md repository changelog). 

But more importanly I would like to create this to learn more rust.

# pull requests

As of now I would love anyone with experience in rust to point out errors and non-idiomatic code. I am not very interested in pull requests for features before reaching a workable state as the main point currently is to gain experience with rust. When/if i reach a feature complete state I would love to get help in expanding on this project! 
