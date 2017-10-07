# Following thebennybox tutorial

This branch is a port of the [YouTube tutorial by thebennybox](https://www.youtube.com/playlist?list=PLEETnX-uPtBXP_B2yupUKlflXBznWIlL5), which is a tutorial on how to build a Game Engine using Java and OpenGL. I'll be adapting the code that they present in the videos using rust and glium as the OpenGL library.

## Video #1

I have created an `engine` module that contains a `Window` and a `MainComponent`, which correspond to the objects that thebennybox creates in their Java code.

One significant change in my take on the problem is that `glium`'s windows are created with an event loop that is not separable from the window. Thus, it is the window that will contain the code to check for the `isCloseRequested` method in the tutorial.

## Video #2

I have moved the code to handle input and window events into the `MainComponent`. I still don't know what the best way to do this is, but I will leave it as such for now. My reasoning is that the window should be able to handle "window" events (such as resize, close, etc), while the game should be able to handle the "input" events (mouse, keyboard and other human-driven interaction). Since it is not clear to me how to handle this (`glutin` needs a reference to an EventLoop to build the window, and my idea was to also use that in the `Game` struct), I won't change it for now. I'll wait until I have a more clear idea on how to implement this in a clean way.

With this change, the `Window` struct is just a wrapper around `glium::Display`.

The main changes in the code (beside the one describe above) are the inclusion of time management code: it is now possible to count frames per second. I added a `Game` struct to handle game-related logic (empty for now), and followed the tutorial by implementing a skeleton organization on this struct: an update and a render method. The input method (which presumably will be handled in the next video) was left out because of the same issue mentioned above.

## Video #3

I've now moved the input code into it's own module, mimicking the original youtube videos. I'm still not very happy with how this let's us handle input. `glutin` gives us an `EventLoop` which we could use to handle the input in a more direct manner that does not require the indirection that was coded here. For now, I'll just keep the code as is and go on with this. If I feel like a new approach is better than what we have now, I'll change it and mention it in this document.

In summary, we introduced the `InputState` struct, which keeps the state of input for the duration of a frame. It handles input events triggered by the keyboard and mouse (and window events such as closing the window) and takes notice of which keys and mouse buttons were pressed in each frame and which were released. The state of the input can then be queried by the `Game` instance, which can react to those inputs.

**Note:** As the moment (and reflecting what is done in the tutorial), modifiers (Alt, Ctrl etc.) are not considered. I suspect they will be in the future, in which case I'll have to update the code of the `InputState` struct.

Some aesthetic changes were implemented as well. In particular, frames per second are now reported in the title of the window, not in the terminal.

## Videos #4, #5 and #6

There is a `cgmath` crate that implements the basic 2-, 3- and 4-dimensional algebra objects (vectors, matrices, quaternions and the like). I will rely on that and not implement my own. Since these videos were all about this algebra stuff, I did not code anything, except the necessary boilerplate to include the `cgmath` create in the project.

## Video #7

The original series uses *a lot* of static code and global state. Rust doesn't like that (and neither do I, that's one of the reasons to do this in Rust), so we have to circumvent that. Furthermore, `glium` provides a stateless mechanism to draw on the screen, and as such each draw method requires, among other things, the parameters needed to perform the drawing. This contrasts with how thebennybox does it, since the Java library they are using provides ways to set state upfront.

As such, I included a `DrawParameters` instance inside the `Window` struct, which controls how the drawing is performed. I can, therefore, setup an instance-level state and not worry about that in the future. (Let's see if that's what I really want, though. Maybe glium's approach works better! But I don't want to keep copying the parameters from function to function so let's say this premature optimization serves both ergonomics and performance.)

The changes here were, therefore:

- inclusion of a `DrawParameters` instance in the `Window` struct: this required me to include a lifetime parameter to `Window` and `MainComponent`;
- creation of the `DrawParameters` instance to be used on all drawings later on, enabling depth test and culling;
- implementation of an auxiliary `Frame` object (a pair containing a mutable reference to the `glium::Frame` being drawn on and an immutable reference to the window's draw parameters) that contains the actual rendering methods, akin to thebennybox's `RenderUtils`. The engine communicates with the rendering phase by providing a callback that operates on one of these `Frame` instances and thus can draw only based on the methods implemented with this type. The rendering on the window is then controlled by the `Window`, which creates a `Frame`, executes the callback and swaps the buffers.

## Video #8

`glium` takes away from the coder much of the housekeeping of buffers (vertex buffers, index buffers etc.) as well as the actual attributes that are copied into the GPU memory. Thus, I did not need to implement a function to copy the data into the GPU.

In this part of the implementation, I introduced data into the `Game` struct. Namely, I created a `Mesh` struct that can be rendered, in the sense that it contains a `draw` method that is responsible for drawing into `glium`'s Frame. The Mesh contains its own vertex and index buffers, as well as an `Rc` reference to a program (a set of shaders to draw the mesh into the OpenGL context).

As a test, `Game` instances contain a triangle mesh that renders on screen. To allow this, I had to create the shader program in GLSL (currently in the `assets` directory), which is part of Video #9 (but oh!, well; here they are anyway -- the next video will surely be easier to implement because of this).

From a technical point of view, the creation of a `Game` instance now requires a reference to a `glium::Display`, which is use to compile the shaders and to create the buffers. Maybe we can remove this dependency in the future, but the inexistence of global state makes me think we won't be anle to, in a clean way.

## Video #9

This video is just about reading GLSL files, compiling a program, linking the various shaders together and binding the hsaders and the program to the OpenGL runtime. `glium` sets this all up to us, so no need to perform anything!