# Maturity

Status: Experimental (Under Development)

Version: 0.1.2

---

## What is Maturity?

`maturity` is a reporting tool for tracking the development state of software; which scans Rust projects, collects metrics, discovers maturity-annotated items, and generates reports describing the current state of the scanned codebase.
> Current release focuses on inventory collection, visibility and scaffolding or foundation for further features and improvements.

The annotation system is intended to be developer-driven; `maturity` reports information; it doesn't modify source code or make decisions on behalf of the developer.

---

## Current Scope (0.1.2)

Version `0.1.2` focuses on inventory reporting.

Current release collects project metrics and discover `#[maturity]` annotated items.

The following systems are not yet implemented:

- Maturity scoring
- Verification systems
- State tracking
- Historical analysis
- Report exporting
  
---

## Current Capabilities

### Project Metrics

- Project name
- Total file count
- Rust file count

### Rust Item Inventory

  - Const
  - Enum
  - ExternCrate
  - Function
  - ForeignMod
  - Implementation
  - Macro
  - Module
  - Static
  - Struct
  - Trait
  - TraitAlias
  - Type
  - Union
  - Use

> [!NOTE]
> `syn::Item::Verbatim` support is planned but not yet implemented.

### Maturity Inventory

The same item kinds are also tracked for `#[maturity]` annotations.

## Output Example
```terminal
╭──────────────────────────────────────────────────────────────────────────────╮
│                            Maturity Report v0.1.2                            │
╰──────────────────────────────────────────────────────────────────────────────╯
Inventory counting   Implemented (partial)
Scoring              Not Implemented
Analysis             Not Implemented

Project
--------------------------------------------------------------------------------
Name                 maturity
File                 59
Rust Files           17

╭───────────────────────────┬───────────────────────┬──────────────────────────╮
│ Item                      ┆ Total Count           ┆ Maturity Count           │
╞═══════════════════════════╪═══════════════════════╪══════════════════════════╡
│ Enums                     ┆                     3 ┆                        3 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Functions                 ┆                     6 ┆                        3 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Implementations           ┆                    15 ┆                        0 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Modules                   ┆                    12 ┆                        0 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Structs                   ┆                     8 ┆                        8 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Types                     ┆                     1 ┆                        1 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Uses                      ┆                    37 ┆                        0 │
╰───────────────────────────┴───────────────────────┴──────────────────────────╯
---

## Installation

### Cargo Plugin
```bash
cargo install cargo-maturity
```

#### Usage:
```bash
cargo maturity
```

### Standalone Cli
```bash
cargo install maturity-cli
```

#### Usage:
```bash
maturity
```

---

## `maturity` ecosystem?

The `maturity` ecosystem currently consists of 5 crates:

### Core functionality
- maturity-macro
- maturity-core
- maturity-report

### Interfaces
- cargo-maturity
- maturity-cli

## Why does it exist?

Over a long duration of developing software, whether alone or as part of a team, one can end up forgetting status of a function, module, or an entire system. Forget forgetting. One can simply lose track of it.
`Where was that function that needed updating?` or `Which systems were already tested?` perhaps `Which modules were documented?`
and the best part `Which parts were considered stable, unstable, experimental or still in development?`.

Yes, Rust already provides attributes such as `deprecated`, and there are crates dedicated to stability, testing, benchmarking, and many other aspects of software development.

However, I often found and still find myself wanting a higher-level view. Not just individual annotations scattered throughout files no, no, no, no.
The information may already exist, but it is scattered throughout the project. Documentation lives in one place, tests live somewhere else, and benchmarks in another location. And the attributes are attached to individual items.

What I want is not another place to store information, but a way to gather and report that information as a whole.

As projects grow, so do the number of TODOS, partially implemented systems, unverified functions, undocumented modules, benchmarks that still need to be written, and ideas that are yet to be implemented.

After let's say good amount of time, it becomes difficult to answer very basic questions to which when we started or not too far from when we started would be able to answer quite easily.

### What is stable?

As in what works just fine without any issues? At the beginning we would say "don't you dare touch xyz", or "xyz is stable", but after dependency additions, integrations, and enough changes, we suddenly understand... "This thing is far more volatile and unpredictable than we originally thought" Something became coupled too tightly... Something broke even though whatever in the world we changed had absolutely no relation to that! or that to it!
Or... at least that's what we thought... "Surely this module and that module couldn't possibly be connected... RighT?..."

### What is still experimental?

We often create some experimental functions, leave it behind, and later "wait... it existed all this time???" When we do remember it however, then we wonder "Where in the world, did I even put that thing?" If we do find it who knows how... then we wonder in what state it is.

### What remains unfinished?

Although we document things, we often end up using brute force first just to have it... working... and only then and only once it works, that we can have it slightly easier documenting it.... Document first, code later? Well... that sounds good... in theory... But it's more of "Let me see if this works~ hehehehe" or "Let me make this thing work first. Then I will clean up later.(ummm clean up?)" The problem is that sometimes we fail to achieve the goal and leave it for later. Maybe we leave a TODO. Maybe we write a comment. Or... we don't even write anything at all... Too tired.... Putting a TODO before even starting something feels strange. But saying: "This system is unfinished." somehow feels much more natural.

### What has already been verified?

We verify things, we run tests, perhaps we benchmark it and move on.
And sometimes we just wonder "Did we actually verify this or did I simply imagine it verifying itself?" I mean the information might be somewhere... in the documentations. umm test files?.. benchmark results... The problem? Organisation. Naming. Date formats (not literally but files having date formats incrementing... hm? take my utilities logger and have it run several times. Once you have ran it i don't know double digit times. Forget checking logging files contents and look at that log folder itself. names of the files. yes.). Experimental benchmarks. Incrementing numbers... All wonderful ideas... until they aren't... Solution is simple, "Just take the latest one and that's the solution!" Genius! Except... The latest was an experimental run out of excitement immediately after finding the actual best result. And not once... Not to mention that haven't renamed or copied the precious one out of excitement. Wonderful. Now we have to go through every single file to find the best results... great.

### What still requires documentation?

Sometimes documentation can wait. A function name, its structure or the algorithm itself might already explain everything we need.
We look at it and think: "yeeaaaah... this one doesn't really need documentation." And perhaps it doesn't. But. when projects grow uncontrollably large... Things become.. far less obvious.. 90% of the project is documented. The remaining 10%? Well.... We know it exists somewhere... Probably... "eeeeeh, Good enough~" ....right?

### Where should development continue?

This question usually appears when everything seems to work just fine. But why do I feel like... something is left unfinished? I think there are optimisations possible? Are there some planned features that hasn't been completed? or systems need verification... We either documented it or TODO'ed it, well for TODO's its wonderful we got plenty of help with that. Documentations helps as well. But reading through hundreds of pages or dozens of modules just to figure out what comes next..... "Tha can become quite heavy".

Documentation? Yes, it definitely helps! Tests, benchmarking helps as well.
If you have proper organisation and flow into project creation and development or some sort of rhyme and reason you probably or most likely going to end up with a good system where you can traverse through entire project with ease and understand, perhaps even recall what most would fail to.

However, there are times where and when we want a good ol'summary of the entire project. A single place where all the questions are answered without needing to go through a terrifying maze of nested directories, modules, notes, source files, and what not.
To which I say,
"Those who are brave and patient to go through entire dungeon shall gain great rewards of what to do next as well as what will need improvement!". Well.... yes, the reward is not as good as the effort for it in most of the giant projects... especially if soloing it.

But say one does venture into this dungeon to gain a bigger picture, well if the documentations are poor.... good luck.

As for me? I shall use my crate for venturing on my behalf, and tell me what in the world this dungeon even is now. Let `maturity` venturing through the dungeon whether annotated with its attributes or not whether there attributes it will recognise and any other features and capabilities given to it, and have it report to me, "Here's the spoils of information." of what it found, all in one place. And hopefully in far less time that I would have spent doing it myself.

The goal is not to replace documentation, testing, benchmarking, or existing Rust attributes-far from it. I do like those systems and I certainly do not wish to re-create them unless it is for learning or if I have time and want to make those for whatever reason. If anything this crate, just adds another layer to documentation and annotation. Essentially adding more attributes to the code, quite literally.

The goal is to make the development state of a project visible, reportable, and easier to understand without first having to navigate through every nested file and module.

---

## Planned Annotation System

Future releases aim to support richer maturity annotations

Example: (might change for a better variant)
```rust
#[maturity(
  state = "stable",
  verified,
  documented
)]
pub fn foo() {}
```

The long-term goal is to allow developers to describe the state of a system directly within the source code and generate maturity reports from those annotations.

---

## Roadmap

See: [Roadmap](ROADMAP.md)

---
  
## Documentation

Additional information can be found in:

- [docs/](docs/)
- [Changelog](CHANGELOG.md)

---

## Security
See: [Security](SECURITY.md)

---

## Contributing
See: [Contributing](CONTRIBUTING.md)

---

## Repository

Primary repository: [Codeberg](https://codeberg.org/SANOTW/maturity.git)

Mirror repository: [GitHub](https://github.com/SANOTW/maturity)

## MSRV

Minimum Supported Rust Version: 1.94.1

---

## License

Copyright 2026 SANOTW

Licensed under the Apache License, Version 2.0.

See the LICENSE file for details.

---
