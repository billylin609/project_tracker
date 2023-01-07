/***************
The format of this document will be
task break down:
	specific tasks
	ddl
	people in charge
	milestone to check
	test cases:
***************/

corresponding systems:
Power system
	test cases: 
		battery input verified on the board (no short very small resistence)
		3.3v 2.4v generating on the board
		o-scope low noise output
		other periperials e.g eeprom power output generation
		recharagable 1s battery bms system
		support usb-c charging

input system(matrix input)
	test cases:
		when pin is triggered display the correct sign on the lcd display(asic test)
		

fpga board
	input unit
		test cases:
			see the interupt is set when buttom is triggered
			correct pin is being identified
			order of precedence of each operation
			low lantency input processing
			beep when the operation is invalid(e.g = with no number included)
	processing unit
		reg size determine
			self_indetify the size each register
			data segmentation
			parallel operation (optimized performance)
			pipeline sequencing
		numerical performance
			add substract(float point and int type the same)
			by using reg seperation
				flag for the float point part
			multiply and division(if there is trancation ref IEEE 746 for more detailed)
		error identify
			system data match up
	output unit
		lcd singal generation
output system(16*4 lcd display)
	
