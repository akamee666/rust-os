## Bare bones

https://wiki.osdev.org/Creating_an_Operating_System#Phase_0:_Introduction

First create the gcc cross compiler following EXACTLY the tutorial is saying because if one step fails you will not be able to finish it.

An operating system is a software controlling the operation of a computer system and its resources. Among other things, there's one very important criteria common to all operating systems:

 Capable of loading and executing user programs providing standardized (hardware-independent) input / output interface for them.

The goal here is to be able to create an program inside my own os!!

A shell is a special program that is usually integrated in an OS distribution, and which offers humans an interface with the computer. The way it appears to users may vary from system to system (command line, file explorer, etc), but the concept is always the same:

    Allow the user to select a program to be started, and optionally give it session-specific arguments.
    Allow trivial operation on the local storage like listing the content of directories, moving and copying files across the system. 

In order to complete those actions, the shell may have to issue numerous system calls, like "open file 'x'; open file 'y' and create it if it doesn't exist; read content from X, write into Y, close both files, write 'done' to standard output".

The shell may also be used by programs that want to start other programs but do not want to do this themselves (e.g. completing file patterns like "*.mp3", retrieving the exact path of the program, etc.).

Modern shells can also have various extra features, such as the following:

    Auto-Completion: By pressing the TAB (or any preferred) key, the word the user is typing will be completed to a valid shell command, a file, directory, or something else. Pressing the auto-complete key multiple times cycles through other completion possibilities.
    Character Insertion: The user can move around in what he or she entered by using the arrow keys. When typing new characters in the middle of a sentence, characters will get "inserted".
    Shell History: By using the up and down arrow keys, the user can scroll through previous input.
    Scrolling: When there are more lines than the console is high, save the output in a buffer and allow the user to scroll up and down in the console.
    Scripting: Some shells have custom scripting languages. Examples of scripting languages are Bash or DOS batch.
    ... 

OSDevers are lucky because there exists a Multiboot Standard, which describes an easy interface between the bootloader and the operating system kernel. It works by putting a few magic values in some global variables (known as a multiboot header), which is searched for by the bootloader. When it sees these values, it recognizes the kernel as multiboot compatible and it knows how to load us, and it can even forward us important information such as memory maps, but you won't need that yet. 

Until now(finished build the cross compiler!) i did not but follow the tutorial to build the cross compiler, made some mistakes that give me some errors and hours troubleshooting but it was complete my fault if you follow the wiki carefully, everything must work.

 
    boot.s - kernel entry point that sets up the processor environment
    kernel.c - your actual kernel routines
    linker.ld - for linking the above files 

This specification adopts a compromise solution to this problem. Multiboot-compliant OS images always contain a magic Multiboot header (see OS image format), which allows the boot loader to load the image without having to understand numerous a.out variants or other executable formats. This magic header does not need to be at the very beginning of the executable file, so kernel images can still conform to the local a.out format variant in addition to being Multiboot-compliant.

While these additional modules could be embedded in the main OS image along with the kernel itself, and the resulting image be split apart manually by the operating system when it receives control, it is often more flexible, more space-efficient, and more convenient to the operating system and user if the boot loader can load these additional modules independently in the first place.

That will take some time to finish for sure, rust can be tough sometimes. what a unique language, but its fun so 


    Packages: A Cargo feature that lets you build, test, and share crates
    Crates: A tree of modules that produces a library or executable
    Modules and use: Let you control the organization, scope, and privacy of paths
    Paths: A way of naming an item, such as a struct, function, or module

crates binary are just programs that can be compiled and run. Each of them need the main.
crates can be libraries as well 
crateroot is the source of files like src and is defined in cargo.toml 


Declaring modules: In the crate root file, you can declare new modules; say, you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:

    Inline, within curly brackets that replace the semicolon following mod garden
    In the file src/garden.rs
    In the file src/garden/mod.rs

