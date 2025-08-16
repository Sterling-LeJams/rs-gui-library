The goal is to be able to move the camera with the arrow keys. 

But I need to be able to pass key presses to the camera struct to do that I had to move the initlization of the camera 
to the WindowState struct. I need to also restructure things to ahve the camera binding passed. 

I think i am going to completely remove the buffer bindings from the VertexShader struct and have a seperate struct with functions to be able to bind stuff to and then I can pass that around and just have it in WindowState.

after I get the key presses to move the camera. I think i need like a trait for whatever object i have to be able to impl make this a buffer