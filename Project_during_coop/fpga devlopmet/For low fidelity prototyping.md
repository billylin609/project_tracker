low fidelity prototyping

possible options: 
https://www.digikey.ca/en/products/detail/lattice-semiconductor-corporation/LCMXO1200E-3TN100I/2751828

The type of FPGA needs to be taken into consideration

1. Don't use the FPGA that requires BGA soldering.
	BGA soldering without proper machine is very hard to do.

What is BGA soldering?
BGA stands for Ball Grid Arrays. BGA has proven in reliability and performance. (increase pin count) require reflow technique

The solution for low fidelity prototyping will be using Quad Flat packaging. 

2. current possible product available: Lattice Semiconductor corporation
		provide portability and battery power device
		
info for FPGA: 
https://www.fpga4fun.com/FPGAinfo1.html

FPGA can programmed into any possible function
	working flow
	create a schematic => compiler the logic into code provided by the manufacture => bootload the file to the FPGA
	
	advantages:
		do not need to modify the circuit for every change
		run faster
		lose functionality after power off (voliate RAM)
		
FPGA vendor:
Xilinx Altera Lattice Actel

FPGA vs CPLD
FPGA more logic blocks 
FPGA RAM based and CPLD EEPROM based(storage availability after power cycle)
FPGA support special routing for efficient algorithms

FPGA logic element run in parallel
micro controller: based on CPU sequential manner => on chip peripheral can be parallel but less configurable (most commonly used devices)

note: mosfet is commonly used especially n-fet
More importantly BJT transistor are able to change the output by using pwm generation

note: learn DC offset + saturation

Logic cells:
http://www.andraka.com/whatLogic.php
https://www.fpga4fun.com/FPGAinfo2.html
= look up table (LUT) + D flip flop 

logic cells interconnected: and input and output will be controlled by the IO cells connected to fpga cells

better implementation (e.g: carry chain are used for arithmetic efficiency)
https://www.engr.siu.edu/haibo/ece428/notes/ece428_logcell.pdf
https://www.electronicshub.org/multiplexerandmultiplexing/#:~:text=A%202%2Dto%2D1%20multiplexer,needed%20to%20do%20these%20operations.


	