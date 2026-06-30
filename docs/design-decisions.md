# Design Decisions

## Project Structure

### Why a workspace?

Initially, `maturity` was just supposed to be a procedural macro crate. That was the entire idea at first, Then I thought: "Well... how am I supposed to report all of this?" So reporting logic came to existence. Then I wanted a CLI interface. Well I thought a Cargo plugin would be fun as well so why not. Then shared data structures and their implementations into core to ensure cross-crate consistency.

Well I went ahead and looked through other public projects and crates that I like using, and many of them follow a similar approach. Multiple crates, each with their own responsibilities, but still living together as part of one ecosystem. And it kinda makes sense.

So workspace felt quite natural. It keeps common things such as versions, editions, dependencies, and licensing in one place while still allowing each crate to have their own necessary configurations on top.

---

## User Interfaces

### Why support both `cargo maturity` and `maturity` instead of only one interface?

Overall both the cargo-plugin and the cli essentially do the same thing. Just cargo version has some nuances handled of currently with how the arguments are parsed and whatever later modifications might arrive.
So, to me cargo plugin feels more natural to me and it fits just nicely into the toolchain, feels complete. However, in The Cargo Book - 3.11. External Tools, near the end or at the end there is statement of cargo as a library is unstable and can have changes without deprecation notices, which is fair and understandable and they are encouraging to utilise CLI interfaces much more.
To which I agree, but also it feels nice to have both options available, essentially more the options the better things are of sorts.
CLI is also helpful in the CI integrations as well as or such as scripting, editors, automation tools, some workflows or other cool stuff that might or might not be integrable with cargo.

I do like the CLI way of doing things but I find it much cooler to do it through cargo. Just my preference maybe. So ye, that's it.

---

## Inventory System

### Why start with inventory instead of maturity scoring?

I thought about maturity scoring first! Believe me, I really did! Then I  started listing what maturity could mean or consists of.. stable? or unstable? or deprecated?, experimental? preview? documented? tested? verified? reviewed? benchmarked? audited? and the list goes on even this list felt like complete but i am missing quite a bit, not to mention that they have their own sets of rules, and possibly sub-types. 
Now how much info to be there is one thing and there are common fields among them all which looks simple to implement but i have a feeling it's better that i start small otherwise i will end up with a spaghetti code. and i also realised that if i tried solve everything first, i would spend forever designing systems and never actually start the project. So  I went with the simplest thing possible: Count things. Literally! how easy can that be? But also doing through a procedural macro. Well if you never done a procedural macro in rust before... it can be confusing. Lucky me that there are documentations all around, and trust me understanding how things works matters more than seeing an example and copying it. Well I wish it was as simple as it sounded. But at least it gives me something working that future features can build upon. After all counting is just the smallest building block and all of the infrastructure well not all but majority of it has been set. Well there's always that other thing that will need to be set that we have no idea about.

---

### Why count items instead of analysing behaviour?

Analysing behaviour isn't easy. This is developer-driven documentation and annotation tool, not a static analysing engine trying to understand everything automatically. Before analysing something, I first need to know what actually exists. Counting items give me that foundation. It tells me:
- What is there
- What is annotated
- What is not annotated
- What the project roughly looks like.

It is arguable when it comes to what a project looks like I mean say 600+ functions with 13 structs... oh... it tells me a lot about the project than i thought it would now. but on a reasonable scale of same number of functions of 600+ with 45 structs eeh... looks better but i don't know. Looks quite imposing (wow) Well once the information of it exists, at least, more advanced systems can be built on top of it!

yee trying to jump straight into behavioural analysis felt like hmmmmmm let's start mixing chemicals without reading what the chemicals are or how many i have of them yay~.... i don't know about you but when i studied the chemistry i understood that the components and chemicals that are brought for students to be taught some of them can be really expensive apparently. Don't how it will be, at the time of you, the reader, reading this weird design decision document.

---

## File Discovery and Collection

### Why use `ignore` instead of `walkdir`?

`walkdir` was one of the first crates I looked at, and honestly I probably would have used it by itself.
But while tinkering about with `walkdir` crate, noticed it going through `target/` folder, which I didn't want it to.. so easy if-else condition. Well I started to think about customisation and easier way to skip files and folders that we do not want reading of and I am essentially reading the project or workspace that is going to be ran rather than every single file that is not needed, and realised that I wanted these:
- `.gitignore` file read
- `.ignore` file read
- `target/` directory skipped
- allow custom exclusion patterns

What I mean by read that the `.gitignore` and `.ignore` the files listed in them to be ignored or skipped.
Yeaaah implementing all of that manually would take time, and the problem has already been solved. Well I came to know about that it has been solved once I searched for alternative walkers or literally searching `ignore .gitignore` in the crates.io and although it wasn't the first result it was there and that was good enough that it was there for me to notice.
Beauty of it is that it already handles `.gitignore`, `.ignore`, glob patterns which is wonderful. It is also built on top of `walkdir`, so I it's an upgrade without having to reimplement everything myself.

Less code for me~

---

## Annotation System

### Why use procedural macros instead of configuration files?

Annotations living next to the code they describe is much easier than maintaining a separate configuration file somewhere else in the project.

I mean look

```rust
#[maturity(state = "experimental")]
pub fn foo() {}
```

It is visible on the spot, I can see immediately see function which items are experimental, stable, or in some other state without having to jump to an external configuration file.

Procedural macros were more difficult to learn initially, but once the foundations are set, extending them becomes much easier and straightforward than maintaining an external configuration systems.

---

### Why is maturity developer-driven instead of automatically inferred?

Software rarely fits into simple rules... A function being large does not automatically make it unstable. Good documentation does not guarantee production readiness. Passing tests does not necessarily mean something should be considered complete.

The people building the software usually understand its current state better than any automated tool ever could.
`maturity` therefore focuses on helping developers express those decisions explicitly rather than trying to infer them automatically.

---

## Development Philosophy

### Why keep maturity local-only and offline-first?

The project simply does not need network access.
Everything required to generate reports already exists on the developer's machine, so introducing external services, telemetry or online dependencies would add complexity without providing much value.
Keeping things local means the tool behaves predictably, works without an internet connection, and only touches the files the user explicitly asks it to analyse.

---

## Community and Governance

### Why is development currently maintainer-driven while encouraging discussions and suggestions?

`maturity` started as a personal tool, but somewhere along the way I realised that other people might find it useful as well.

At the moment, I still want to understand the project deeply and establish its long-term direction before opening development more broadly. The foundations are still evolving, and keeping implementation decisions centralised makes experimentation much easier.

That does not mean outside ideas are unwelcome-quite the opposite. If something is intended for public use, then public feedback matters. Discussions, suggestions, and different perspectives help shape a better tool that I could build entirely in isolation. Not every suggestion will be implemented, but understanding how other developers think about workflows, annotations, reporting, and maturity systems is incredibly valuable.

For now, implementation remains maintainer-driven, while conversations remain open. Whether that changes in the future depends on where the project eventually settles.

---

### Why choose Apache-2.0?

So, I ended up choosing Apache-2.0 as the single license for the project rather than going a dual-licensing model.

Part of the reason is simply that I prefer keeping things straightforward. One license, one set of rules, and no ambiguity about which option people should choose. Dual licensing never really appealed to me, and Apache-2.0 already gives me everything I currently want from the project.

Most importantly, the license keeps the project open, easy to contribute to, and usable in both personal and commercial environments without introducing unnecessary complications. For now, it feels like the right balance between openness, clarity, and long-term maintainability.

---
