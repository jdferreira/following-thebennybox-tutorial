# Following thebennybox tutorial

This branch is a port of the [YouTube tutorial by thebennybox](https://www.youtube.com/playlist?list=PLEETnX-uPtBXP_B2yupUKlflXBznWIlL5), which is a tutorial on how to build a Game Engine using Java and OpenGL. I'll be adapting the code that they present in the videos using rust and glium as the OpenGL library.

## Video #1

I have created an `engine` module that contains a `Window` and a `MainComponent`, which correspond to the objects that thebennybox creates in their Java code.

One significant change in my take on the problem is that `glium`'s windows are created with an event loop that is not separable from the window. Thus, it is the window that will contain the code to check for the `isCloseRequested` method in the tutorial.

## Video #2

I have moved the code to handle input and window events into the `MainComponent`. I still don't know what the best way to do this is, but I will leave it as such for now. My reasoning is that the window should be able to handle "window" events (such as resize, close, etc), while the game should be able to handle the "input" events (mouse, keyboard and other human-driven interaction). Since it is not clear to me how to handle this (`glutin` needs a reference to an EventLoop to build the window, and my idea was to also use that in the `Game` struct), I won't change it for now. I'll wait until I have a more clear idea on how to implement this in a clean way.

The main changes (beside the one describe above) in the code are the inclusion of time management code: it is now possible to count frames per second. I added a `Game` struct to handle game-related logic (empty for now), and followed the tutorial by implementing a skeleton organization on this struct: an update and a render method.