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
makes operating the layout more interesting that just running the trains
endlesslt around an oval or something.


