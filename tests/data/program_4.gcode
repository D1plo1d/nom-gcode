; This example loads a file on to the SD card for a marlin FW 3D printer

M28 teg.gcode; start recording: https://marlinfw.org/docs/gcode/M028.html
G28
G1 Z20
G1 X100 Y100
G28
M29; stop recording
M23 teg.gcode; select the file
M24; start the print

