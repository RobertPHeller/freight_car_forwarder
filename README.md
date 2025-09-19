# Freight Car Forwarder in rust

The Freight Car Forwarder is based on Tim O'Connor's Freight Car Forwarder
originally written in QBASIC.  I first ported the program to a pure Tcl/Tk
application.  Then for better performance, I recoded the low-level guts
(mostly heavy data indexing logic) to a C++ class library, using the STL to 
implement the various aggregate collections of objects, retaining Tcl/Tk
for the GUI.  

This program is a port of the C++ class library, with a console (terminal)
main program to access the algorithms and data structures.

This program is used to generate switch lists for realistic operations
on a model railroad layout.  The switch lists direct operators of trains
to drop off and pickup cars as they operate trains on the layout. This
makes operating the layout more interesting than just running the trains
endlessly around an oval or something.

This crate can either be used as a complete runable program using the included
main.rs and its support modules, or it can be used as a library crate, calling
the methods in  system.rs  to load and process a system data set in a specific
way.  I am not  sure if I want to  attempt  to code a GUI in rust and I am not
sure how (or even if it is  possible)  to build a rust  library  into a shared
library  that Tcl can load and access.  My *prefered*  GUI coding  language is
Tcl/Tk.

If someone else is  interested  in  creating a GUI for this crate, I would be
interested in hearing about it.
