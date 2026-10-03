# Sonic Audio Tools, a partial Rust rewrite

A set of tools to modify CRIWARE file formats.

Currently it only supports CSB files (that can be bundled with a CPK file).

## What is left behind / will not be implemented in the Rust rewrite

* Only CSB/CPK files are supported
* the compression of files, since in the original code the reading was only implemented, but I want to write the files as well.
* parallel processing of files since it would still hit the IO bottleneck (original project has a parallel extractor of the files).
* file alignment since it wasn't implemented in the original code.
* also the masking seems to be unused (always `false`)
* ahead-of-time values loading was implemented instead of lazy-loading (I mean the file is read fully in one iteration instead of jumping on the stream in the memory between the pointers and values)

## Tested games

* Sonic 4 Episode 1 (PC) (read/extraction)
* Sonic 4 Episode 2 (PC) (read/extraction)
* Sonic Generations (PC) (read/extraction)

## Want to contribute?

Contributions welcome to the Rust rewrite.

If you want to contribute to the original C# project, please follow the original project's repository.

## Wiki of the original project

If you wish to understand what's going on with the file formats, visit the [wiki](https://github.com/blueskythlikesclouds/SonicAudioTools/wiki) page.

## Special thanks

Special thanks to [Skyth](https://github.com/blueskythlikesclouds) for the original C# tools and for licensing them under the MIT license.

## AI disclosure and usage policy and contribution guide (for the rewritten project)

The word "slop" is not what I want to achieve with this project, so I prepared an [AI usage policy and contribution guide](./AI_USAGE_POLICY_AND_CONTRIBUTION_GUIDE.md).

Currently, AI assistance from SourceCraft Code Assistant helped in a few ways:

* Helped debugging the rewritten library and tools in the cases that I overlooked due to inattention.
