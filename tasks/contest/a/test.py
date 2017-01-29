import random

print(random.uniform(0, 25))

max_r = 30000
max_c = 100000

maxc_in_r = 500

with open("test3.txt", "w") as f:
	nextc_len = int(random.uniform(1, maxc_in_r))
	for c in range(0, nextc_len):
		if c != 0:
			f.write(" ")
		nextc = int(random.uniform(1, max_c))
		f.write("{}".format(nextc))
	f.write("\n")
	for r in range(0, max_r):
		nextc_len = int(random.uniform(1, maxc_in_r))
		for c in range(0, nextc_len):
			if c != 0:
				f.write("+")
			nextc = int(random.uniform(1, max_c))
			f.write("{}".format(nextc))
		f.write("->")
		nextc_len = int(random.uniform(1, maxc_in_r))
		for c in range(0, nextc_len):
			if c != 0:
				f.write("+")
			nextc = int(random.uniform(1, max_c))
			f.write("{}".format(nextc))
		f.write("\n")		
