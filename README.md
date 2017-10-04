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

In summary, we introduced the `INputState` struct, which keeps the state of input for the duration of a frame. It handles input events triggered by the keyboard and mouse (and window events such as closing the window) and takes notice of which keys and mouse buttons were pressed in each frame and which were released. The state of the input can then be queried by the `Game` instance, which can react to those inputs.

**Note:** As the moment (and reflecting what is done in the tutorial), modifiers (Alt, Ctrl etc.) are not considered. I suspect they will be in the future, in which case I'll have to update the code of the `InputState` struct.

Some aesthetic changes were implemented as well. In particular, frames per second are now reported in the title of the window, not in the terminal.

## Videos #4, #5 and #6

There is a `cgmath` crate that implements the basic 2-, 3- and 4-dimensional algebra objects (vectors, matrices, quaternions and the like). I will rely on that and not implement my own. Since these videos were all about this algebra stuff, I did not code anything, except the necessary boilerplate to include the `cgmath` create in the project.