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

//Shareable mutable containers exist to permit mutability in a controlled manner, even in the presence of aliasing. Cell<T>, RefCell<T>, and OnceCell<T> allow doing this in a single-threaded way—they do not implement Sync. (If you need to do aliasing and mutation among multiple threads, Mutex<T>, RwLock<T>, OnceLock<T> or atomic types are the correct data structures to do so). https://fongyoong.github.io/easy_rust/Chapter_41.html
/* 

   You'd use a RefCell for similar reasons to why you'd need a mutex - you need to mutate via a shared reference (i.e. "interior mutability"). So why choose a RefCell?

   it's likely faster (it's just doing simple arithmetic, compared to mutex which usually communicates with the OS)

   it works in #[no_std] environments

   you don't need thread safety

   There are other concurrency primitives you can use. The most obvious one is atomics (e.g. AtomicU64). Like mutex, these are thread safe, and allow mutation via a shared reference. Unlike mutex, however, these are implemented by your CPU, rather than your OS, so typically work in no_std crates (though perhaps there are microcontrollers with no atomics? Not sure). The main downside is you can only use certain types, and it's pretty much just integers and bool (which is kinda already an integer). Custom types aren't supported.

   As for part 4, generally no, a mutex won't panic if another thread has the lock - it will block until the lock is available. However, mutex.lock() returns a result, since the lock can be "poisoned" (if the thread that was holding the lock panicked). This can be safe to ignore (and you can get the mutex guard from the error), but rust has chosen to make you handle it explicitly by default. It's pretty common to see mutex.lock().unwrap() - I wouldn't consider this a code smell
   */ 

// To provide a global writer that can be used as an interface from other modules without carrying a Writer instance around, we try to create a static WRITER:
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/const-and-static.html



/*n Rust, borrow_mut() is a method associated with the RefCell type. It allows you to obtain a mutable reference to the data stored inside a RefCell. This is useful when you need to modify the data within the RefCell, but Rust's ownership system wouldn't normally allow it.

  Here's a breakdown of the key concepts involved:

  RefCell: RefCell is a wrapper type provided by the std::cell module. It acts like a regular reference cell but with additional features for runtime borrowing checks.

  Borrowing: Rust's ownership system relies on the concept of borrowing. You can borrow data from a variable to access its contents for a limited time. There are two types of borrows:
  Immutable Borrow (&): This allows you to read the data but not modify it.
  Mutable Borrow (&mut): This allows you to both read and modify the data.

  Borrow Checker: Rust's borrow checker ensures that only one mutable borrow or multiple immutable borrows exist for a piece of data at any given time. This prevents data races and memory corruption.

  How borrow_mut() Works:

  When you call borrow_mut() on a RefCell, it attempts to acquire a mutable reference to the underlying data.
  The RefCell performs a runtime check to ensure no other borrows (immutable or mutable) exist for the data.
  If the check succeeds, borrow_mut() returns a mutable reference (&mut T) to the data inside the RefCell. You can then use this reference to modify the data.
  If the check fails (because another borrow exists), borrow_mut() typically panics, indicating a potential borrowing violation.

  Why Use borrow_mut()?

  Modifying Data in Immutable Structs: Sometimes, a struct might primarily store immutable data but have a field that needs occasional modification. RefCell allows you to wrap the struct and use borrow_mut() to modify that specific field while maintaining immutability for the rest of the data.
  Safely Releasing Borrows: Unlike raw pointers, RefCell ensures that borrows are released automatically when they go out of scope. This helps prevent memory leaks.

  Things to Consider with borrow_mut():

  Potential Overhead: Using RefCell introduces some runtime overhead compared to plain references. This might be a concern in performance-critical sections of your code.
  Manual Borrow Management: While RefCell helps with safety checks, it doesn't eliminate the need for careful borrow management in your code. You still need to ensure that borrows don't conflict with each other and are released promptly.

  In Summary:

  borrow_mut() is a powerful tool for controlled mutability within RefCell. It allows you to modify data in scenarios where Rust's ownership system might otherwise prevent it. However, use it judiciously and be aware of the potential overhead and need for manual borrow management.
  */

IN    Read from a port
OUT   Write to a port
INS/INSB  Input string from port/Input byte string from port 
INS/INSW  Input string from port/Input word string from port 
INS/INSD  Input string from port/Input doubleword string from port
OUTS/OUTSB    Output string to port/Output byte string to port 
OUTS/OUTSW    Output string to port/Output word string to port 
OUTS/OUTSD    Output string to port/Output doubleword string to port

ram and io devices are connect in the same address, but i the M/#IO can distinguish between which one im trying to use depending on the instruction that im using

interrupt descriptor table 

The IDT entries are called gates. It can contain Interrupt Gates, Task Gates and Trap Gates.  



