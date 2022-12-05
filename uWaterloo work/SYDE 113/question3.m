rng(14)

matrix = randi ([-10, 10], [2, 2]);

writematrix(matrix, 'question3_generate.txt')

%matrix = readmatrix("question3_generate.txt")

det (matrix)

rank (matrix)