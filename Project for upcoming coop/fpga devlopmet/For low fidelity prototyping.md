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
	