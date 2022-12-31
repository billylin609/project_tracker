The current aritecture is for the proof of concept validation:

it includes the following ability:
able to accept input from buttons
the type of button will include 
'0' '1' '2' '3' '4' '5' '6' '7' '8' '9'
'+' '-' '*' '/' '='

the calculater should have the following ability:
do basic calculation
able to determine the size of the input and predict the possible output size
able to use the current register or to merge small register together

overflow 
underflow
carry
numerical error 
identify

display human like arithmetic

   1
+  1
----
   2

able able to make it into any format: binary, hex, oct, dec

have the basic ablility to display it on a lcd display

The architecture:
power source: 5v power distribution board (for future expansion) 
	requirement: low noise, better efficiency
	12v to 5v
	connector: round jack

possible fpga option: https://www.digikey.ca/en/products/detail/lattice-semiconductor-corporation/LCMXO1200E-3TN100I/2751828
	reason to use : quad flat package. (easy to replace)

language use: verilog

permanent storage: eeprom or flash not decided

input: 15 button, a turn on button and rest button.
	a total of 17 button

output: lcd display at least 4 line of output
	example product: https://canada.newark.com/fordata/fc1604a01-fsyybw-51lr/display-alphanumeric-16x4-yellow/dp/02AC9715?gclid=EAIaIQobChMI9cah2uqh_AIVhsDICh2iYgMMEAQYAyABEgIf8_D_BwE&s_kwcid=AL!8472!3!!!!x!!&mckv=_dc|pcrid||plid||kword||match||slid||product|02AC9715|pgrid||ptaid||&CMP=KNC-GCA-GEN-Shopping-NP&gross_price=true

debugging option: JTAG support.
