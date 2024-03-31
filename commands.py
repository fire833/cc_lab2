from argparse import ArgumentParser
import numpy as np
import jinja2
import sys
import subprocess
import os
import random
import csv
import json
from templates.base1 import template as tmplbase1
from templates.base2 import template as tmplbase2
from templates.asm import template as tmplasm
from templates.cuda1 import template as tmplcuda1
from templates.cuda2 import template as tmplcuda2
from templates.simd1 import template as tmplsimd1
from templates.simd2 import template as tmplsimd2

def new_parser():
	p = ArgumentParser(prog=sys.argv[0], description="Wrapper scripts/utilities for Lab 2 Metaprogramming", add_help=True, allow_abbrev=True)
	sub = p.add_subparsers(title="subcommands")
	
	if os.name == "nt":
		progOut = "./prog.exe"
	else:
		progOut = "./prog"

	gen = sub.add_parser("generate")
	gen.add_argument("--pattern", help="Provide the pattern you want to generate against (should be a string with spaces)", type=str, dest="pattern")
	gen.add_argument("--template", help="Provide the template you want to generate from", type=str, default="base1", dest="template")
	gen.add_argument("--output", help="Provide the output location of the generated program executable.", type=str, default=progOut, dest="output")
	gen.set_defaults(func=generate)
	rgen = sub.add_parser("genrand")
	rgen.add_argument("--template", help="Provide the template you want to generate from", type=str, default="base2", dest="template")
	rgen.add_argument("--output", help="Provide the output location of the generated program executable.", type=str, default=progOut, dest="output")
	rgen.add_argument("--arg_count", help="Provide the number of arguments to build with your random permutation.", type=int, default=1000, dest="arg_count")
	rgen.add_argument("--auto_args", help="Automatically input the amount of args needed", action="store_true", dest="auto_args", default=False)
	rgen.add_argument("--sequence_amount", help="format: number of 8 sequences, number of 4 sequences", type=str, default=",", dest="sequence_amount")
	rgen.set_defaults(func=generate_rand)
	r = sub.add_parser("run")
	r.add_argument("--values", help="Provide a comma-separated list of integers to pass to the generated program.", type=str, dest="values")
	r.set_defaults(func=run)
	rrand = sub.add_parser("runrand")
	rrand.add_argument("--input", help="Provide a path to the generated program to execute.", type=str, default=progOut, dest="input")
	rrand.add_argument("--arg_count", help="Provide number of values your generated program expects.", type=int, dest="arg_count")
	rrand.add_argument("--iter", help="Provide the number of iterations you want to execute.", type=int, default=25, dest="iterations")
	rrand.set_defaults(func=run_rand)
	rrprt = sub.add_parser("runbench")
	rrprt.add_argument("--input", help="Provide a path to the generated program to execute.", type=str, default="benchmarks", dest="input")
	rtest = sub.add_parser("runtests")

	return p

def runner(args: ArgumentParser):
	# print(args)
	if sys.argv[1] == "generate":
		print("generating new output program")
		return generate(args.pattern, args.template, args.output, False, False)
	elif sys.argv[1] == "genrand":
		return generate_rand(args.template, args.output, args.arg_count, False, False, args.auto_args, args.sequence_amount)
	elif sys.argv[1] == "run":
		print("running output program")
		return run(args.input, [int(arg) for arg in args.values.split(",")])
	elif sys.argv[1] == "runrand":
		print("running output program with random inputs")
		outputs = run_rand(args.input, args.iterations, args.arg_count)
		print(outputs)
	elif sys.argv[1] == "runbench":
		print("running reporting")
		bench(args.input)
	elif sys.argv[1] == "runtests":
		print("running tests")
		return run_tests([])

templates = {
	"base1": tmplbase1,
	"base2": tmplbase2,
	# "asm1": tmplasm,
	"cuda1": tmplcuda1,
	"cuda2": tmplcuda2,
	"simd1": tmplsimd1,
	"simd2": tmplsimd2,
}

def generate(pattern: str, template: str, output: str, asm: bool, omp: bool):
	
	#the code looks a little more goofy but it works like this
	if pattern.find(';') != 1: #check if theres a second half (an automatically generated input)
		parts = pattern.split(';')
		second_half = parts[1].split(',') if len(parts) > 1 else []
		inputlen = len(second_half)
		second_half = ','.join(map(str, second_half))
		values: [(int, int)] = [(i, int(arg)) for i, arg in enumerate(parts[0].split(","))]
		pattern = parts[0]
	else: 
		values: [(int, int)] = [(i, int(arg)) for i, arg in enumerate(pattern.split(","))]
	arg_count = len(values)

	outfile = templates[template]["program_output"]
	
	permute_gen = ""
	if template == "simd1" or template == "simd2":
		print("generating simd optimizations with bruteforcer")
		permute_gen = subprocess.run(["cargo", "run", "--manifest-path=bruteforcer/Cargo.toml", "--", "-p", pattern, "simplec"], capture_output=True).stdout.decode()
	
	print("rendering template")
	env = jinja2.Environment()
	t = env.from_string(templates[template]["template"])
	f = open(file=outfile, mode="w")
	if 'second_half' in locals():
		f.write(t.render(arg_count=arg_count, values=values, permute_gen=permute_gen, inputstr=second_half))
	else:
		f.write(t.render(arg_count=arg_count, values=values, permute_gen=permute_gen))
	f.close()

	args = templates[template]["compiler_prefix"]
	args.append("-o")
	args.append(output)
	if omp:
		args.append("-fopenmp")
	if asm:
		args.append("-S")
		subprocess.run(args)
		args.pop(len(args) - 1)
		args.pop(len(args) - 1)

	args.append(outfile)
	print("compiling program")
	subprocess.run(args)

def rand_pattern(arg_count: int):
	nums = list(range(0, arg_count))
	random.shuffle(nums)
	return nums

def generate_rand(template: str, output: str, arg_count: int, asm: bool, omp: bool, auto_args: bool, sequence_amount: str):
	numstr = ""
	if (sequence_amount != ","):
		sequences = sequence_amount.split(",")
		numstr += generate_sequences(100, int(sequences[0]), int(sequences[1]))
	else:
		for i in rand_pattern(arg_count):
			numstr += f"{i},"
		numstr = numstr.removesuffix(",")

	numstr += ";"
	if auto_args:
		for i in list(range(1, arg_count+1)):
			numstr += f"{i},"
	numstr = numstr.removesuffix(",")

	print(f"pattern: {numstr}")

	generate(numstr, template, output, asm, omp)

def run(input: str, args: [int]):
	parsed = ""

	for i in args:
		parsed += f"{i},"

	return json.loads(subprocess.run([input, str(len(args)), parsed.rstrip(",")], capture_output=True).stdout.decode()) 

def run_rand(inputprog: str, iterations: int, arg_count: int):
	values = [i for i in range(arg_count)]
	outputs = []
	
	for i in range(iterations):
		out = run(inputprog, values)
		outputs.append(out["compute"])

	return outputs

def bench(outDir: str):
	patterns = ["3,2,1,0"]

	for tmpl in templates:
		for pattern in patterns:
			name = tmpl[name] + "_" + tmpl[program_output]
			outStr = f"{outDir}/{name}"
			generate(pattern, tmpl, outStr, False, False)

def run_report(patterns: [(str, int)], tmplversions: [str], output: str):
	for tmpl in tmplversions:
		for pattern in patterns:
			pat = pattern[0].replace(" ", "_").replace(",", "-")
			out = f"{output}/{tmpl}_{pat}"

			for i in range(2):
				doOpenMP = False
				csvfile = f"{output}/{tmpl}_{pat}.csv"
				if i == 1:
					csvfile = f"{output}/{tmpl}_{pat}_omp.csv"
					doOpenMP = True

				generate(pattern[0], tmpl, out + ".c", out, "gcc", False, doOpenMP, 1000)
				outputs = run_rand(out, 250, pattern[1], 20000)
				write_csv(outputs, csvfile)

def write_csv(outputs, output: str):
	f = open(output, "w")
	f.write("TIMINGS\n")
	for out in outputs:
		if out["compute"]:
			v = out["compute"]
			f.write(f"{v}\n")

	f.close()

def run_tests(tmplversions: [str]):
	tests = [
		("0,1 2,3", [1,2,3,4], [21]),
		("0,1", [1,2,3,4,5,6], [3, 5, 7, 9, 11]),
	]

	for tmpl in tmplversions:
		for (i, test) in enumerate(tests):
			tOut = f"tests/test{i}_{tmpl}"
			generate(test[0], tmpl, f"tests/test{i}_{tmpl}.c", tOut, "gcc", False, False, 512)
			res = run(tOut, test[1])
			if not res["values"]:
				print(f"test {i} failed: no values outputs")
			else:
				outputArr = np.array(res["values"])
				expectedArr = np.array(test[2])
				if (np.array_equal(outputArr, expectedArr)):
					print(f"test {i} passed")
				else:
					print(f"test {i} failed, comparison inequality")

#Generate an array depending on your desired number of sequences of 8/4 numbers (may mess up on smaller input sizes)
def generate_sequences(length, num_sequences_8, num_sequences_4):
    array = list(range(1, length + 1))
    
    sequences_8 = []
    sequences_4 = []
    
    for i in range(num_sequences_8):
        start_index = random.randint(0, len(array) - 8)
        seq = array[start_index:start_index+8]
        random.shuffle(seq)
        sequences_8.append(seq)

        array = [x for x in array if x not in seq]
    
    for i in range(num_sequences_4):
        start_index = random.randint(0, len(array) - 4)
        seq = array[start_index:start_index+4]
        random.shuffle(seq)
        sequences_4.append(seq)
        array = [x for x in array if x not in seq]
	
    print(sequences_4)
    print(sequences_8)
    random.shuffle(array)

    shuffled_array = []
    for num in array:
        shuffled_array.append(str(num))
    
    for seq in sequences_8 + sequences_4:
        insert_index = random.randint(0, len(shuffled_array))
        shuffled_array[insert_index:insert_index] = map(str, seq)  # Convert numbers to strings
        
    return ','.join(shuffled_array)
