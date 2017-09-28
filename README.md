# Following thebennybox tutorial

This branch is a port of the [YouTube tutorial by thebennybox](https://www.youtube.com/playlist?list=PLEETnX-uPtBXP_B2yupUKlflXBznWIlL5), which is a tutorial on how to build a Game Engine using Java and OpenGL. I'll be adapting the code that they present in the videos using rust and glium as the OpenGL library.

## Video #1

I have created an `engine` module that contains a `Window` and a `MainComponent`, which correspond to the objects that thebennybox creates in their Java code.

One significant change in my take on the problem is that `glium`'s windows are created with an event loop that is not separable from the window. Thus, it is the window that will contain the code to check for the `isCloseRequested` method in the tutorial.