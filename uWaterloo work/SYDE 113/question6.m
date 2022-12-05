x1 = -3 * exp(pi*i/6)
x2 = -3 * exp(5*pi*i/6)
x3 = -3 * exp(9*pi*i/6)

x1_real = real(x1)
x1_imag = imag(x1)

x2_real = real(x2)
x2_imag = imag(x2)

x3_real = real(x3)
x3_imag = imag(x3)

plot (x1_real, x1_imag, '*', x2_real, x2_imag, '^', x3_real, x3_imag, 'O')

xlim([-28,28])
ylim([-28,28])

x1^3

x2^3

x3^3