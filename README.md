GTK4 Relm lab to put a window in front of all others on either x11 or wayland linux desktops.
this window will also be a server which listens for responses on the dbus servicebus.

How to test it

Start App A first in one terminal window. It will register com.example.AppA on your Linux desktop session bus.
Start App B in another terminal window. You will immediately see App A printing the text packets every second.

Bonus Linux Debugging: 
Because this uses standard system architecture, you can open a third terminal and run busctl --user monitor com.example.
AppA to watch the raw text packets fly across your operating system in real-time.
