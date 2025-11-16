# **RAPTOR**

### **Realtime Abstracted Path Tree Observer**

*A dangerously cool 3D filesystem explorer inspired by Jurassic Park’s iconic FNS scene.*

![Demo](./assets/demo.gif)

[It's a Unix system](https://youtu.be/dFUlAQZB9Ng?si=Xwjish_MCRif8j7A)

---

## What Is This?

**RAPTOR** is a 3D filesystem navigator built with **Rust + macroquad**, designed to make browsing your folders feel like hacking the mainframe *inside* Jurassic Park.

Remember that overly dramatic scene where a kid exclaims

> “It’s a UNIX system… I know this!”

and then flies through a cheesy 3D filesystem?

Yeah.
That.
But modern. Smooth. Glowy, because of course.

RAPTOR turns your directories and files into a neon-green cyber-grid of chunky blocks you can rotate around, scan, click on, and explore as if your PC security was being compromised by velociraptors.

## Things

* **3D Blocks** for every file and directory
* **Orbiting Camera** (right-click drag + scroll zoom)
* **Navigation**:
  `h j k l` to move, `o/Enter` to open, `u/-` to go up
* **Automatic Directory Grid Layout**
*  **Selection, Hover, and Glow Effects**
* **Raycast Block Picking**
* **Dynamic Block Heights** (file size / children count)
* **Scanning Intro Line** (because retro sci-fi vibes)
* **Breadcrumb Navigation**
* **Labels Toggle** (Tab)
* **Interactive UI Panels** with live stats
* **Smooth camera tweening**
*  **Cool glowing wireframes because aesthetics**

## Navigation

### **Camera**

* **Right-mouse drag** → Rotate
* **Scroll wheel** → Zoom

### **Navigation**

* `h` → Left
* `j` → Down
* `k` → Up
* `l` → Right
* `o` or `Enter` → Enter directory
* `u` or `-` → Go to parent
* `.` → To show hidden files
* `/` → Go to root
* `Home` → Go to home directory
* `Backspace/Esc` → Back in history

### **Mouse**

* Hover → Highlight block
* Click → Select
* Double click → Enter directory

### **UI**

* `Tab` → Toggle labels


## Why Jurassic Park?

Because *FNS* from Jurassic Park is legendary.
It’s campy. It’s 90s CGI nonsense. It’s a cultural artifact.

And I thought:

> “What if I recreate that…
> but actually make it FUN and SMOOTH and more 60 FPS…
> and with keybinds?”

And boom — **RAPTOR** hatched.

No dinosaurs were harmed in the making of this filesystem explorer.
(Except maybe your CPU when opening a directory with 30k files.)

## Running

```bash
cargo run --release
```

RAPTOR will open in all its neon glory.

## Dependencies

RAPTOR is built using **macroquad**, a Rust game framework that works on Linux, macOS and Windows with minimal setup.

Macroquad repo (installation notes & troubleshooting):
https://github.com/not-fl3/macroquad

## Future Ideas (aka InGen Phase 2)

* File previews
* Drag-and-drop
* Jurassic-Park-style *“Access Denied”* red windows
* Terminal overlay
* Dinosaur noises when selecting something
* VR version?? (life… finds a way)
