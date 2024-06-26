from argparse import ArgumentParser
import numpy as np
import jinja2
import sys
import subprocess
import os
import random
import csv
import json
from patterns import patterns
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
	gen.add_argument("--bin-output", help="Provide the output location of the generated program executable.", type=str, default="./prog", dest="binout")
	gen.add_argument("--output", help="Provide the output location of the generated program source.", type=str, default="", dest="output")
	gen.set_defaults(func=generate)
	rgen = sub.add_parser("genrand")
	rgen.add_argument("--template", help="Provide the template you want to generate from", type=str, default="base2", dest="template")
	rgen.add_argument("--bin-output", help="Provide the output location of the generated program executable.", type=str, default="./prog", dest="binout")
	rgen.add_argument("--output", help="Provide the output location of the generated program source.", type=str, default="", dest="output")
	rgen.add_argument("--arg_count", help="Provide the number of arguments to build with your random permutation.", type=int, default=1000, dest="arg_count")
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
	rrprt.add_argument("--output", help="Specify the output file location for your benchmark data frame results.", type=str, default="results.csv", dest="output") 
	rtest = sub.add_parser("runtests")

	return p

def runner(args: ArgumentParser):
	# print(args)
	if sys.argv[1] == "generate":
		print("generating new output program")
		return generate(args.pattern, args.template, args.binout, args.output, False, False)
	elif sys.argv[1] == "genrand":
		return generate_rand(args.template, args.binout, args.output, args.arg_count, False, False)
	elif sys.argv[1] == "run":
		print("running output program")
		return run(args.input, [int(arg) for arg in args.values.split(",")])
	elif sys.argv[1] == "runrand":
		print("running output program with random inputs")
		outputs = run_rand(args.input, args.iterations, args.arg_count)
		print(outputs)
	elif sys.argv[1] == "runbench":
		print("running reporting")
		bench(args.input, args.output)
	elif sys.argv[1] == "runtests":
		print("running tests")
		return run_tests([])

templates = {
	"base1": tmplbase1,
	"base2": tmplbase2,
	"simd1": tmplsimd1,
	"simd2": tmplsimd2,
	"cuda1": tmplcuda1,
	"cuda2": tmplcuda2,
	# "asm1": tmplasm,
}

benchplates = {
	"base2": tmplbase2,
	"simd2": tmplsimd2,
	"cuda2": tmplcuda2,
}

def generate(pattern: str, template: str, binout: str, output: str, asm: bool, omp: bool):
	values: [(int, int)] = [(i, int(arg)) for i, arg in enumerate(pattern.split(","))]
	arg_count = len(values)

	if output == "":
		output = os.path.join(os.path.dirname(binout), templates[template]["program_output"])

	permute_gen = ""
	if template == "simd1" or template == "simd2":
		print("generating simd optimizations with bruteforcer")
		proc = subprocess.run(["cargo", "run", "--manifest-path=bruteforcer/Cargo.toml", "--", "-p", pattern, "simplec"], capture_output=True)
		if proc.returncode != 0:
			print(proc.stderr.decode())
			exit(1)
		permute_gen = proc.stdout.decode()

	print("rendering template")
	env = jinja2.Environment()
	t = env.from_string(templates[template]["template"])
	f = open(file=output, mode="w")
	f.write(t.render(arg_count=arg_count, values=values, permute_gen=permute_gen))
	f.close()

	args = templates[template]["compiler_prefix"].copy()
	args.append("-o")
	args.append(binout)
	if omp:
		args.append("-fopenmp")
	if asm:
		args.append("-S")
		subprocess.run(args)
		args.pop(len(args) - 1)
		args.pop(len(args) - 1)

	args.append(output)
	print(f"compiling program: {args}")
	subprocess.run(args)

def rand_pattern(arg_count: int):
	nums = list(range(0, arg_count))
	random.shuffle(nums)
	return nums

def generate_rand(template: str, binout: str, output: str, arg_count: int, asm: bool, omp: bool):
	numstr = subprocess.run(["cargo", "run", "--manifest-path=bruteforcer/Cargo.toml", "--", "-l", str(arg_count), "randpat"], capture_output=True).stdout.decode()
	numstr = numstr.strip("\n")
	print(f"pattern: {numstr}")
	generate(numstr, template, binout, output, asm, omp)

def run(input: str, args: [int]):
	parsed = ""

	for i in args:
		parsed += f"{i},"

	data = [input, str(len(args)), parsed.rstrip(",")]
	# print(data)
	proc = subprocess.run(data, capture_output=True)
	if proc.returncode != 0:
		print(f"stderr: {proc.stderr.decode()}")
		print(f"stdout: {proc.stdout.decode()}")
		exit(1)

	return json.loads(proc.stdout.decode()) 

def run_rand(input_prog: str, iterations: int, arg_count: int):
	values = [i for i in range(arg_count)]
	outputs = []
	
	for i in range(iterations):
		out = run_rand_once(input_prog, values, values)
		outputs.append(out["compute"])

	return outputs

def run_rand_once(input_prog: str, arg_count: int, args: [int]):
	return run(input_prog, args)

def bench(outDir: str, outFile: str):
	file = open(outFile, "w")
	results = csv.writer(file, delimiter=",")
	results.writerow(["compute_time", "template_name", "arg_count", "pattern_number", "iteration_number"])

	for name, tmpl in benchplates.items():
		for arg_count, patternset in patterns.items():
			for i, pattern in enumerate(patternset):
				pname = name + "_" + str(arg_count) + "_" + str(i) + "_" + tmpl["program_output"]
				output = f"{outDir}/{pname}"
				outbin = f"{outDir}/prog"
				print(f"running template {name} arg_count {arg_count} pattern {i} at {output}...")
				generate(pattern, name, outbin, output, False, False)
				values = run_rand(outbin, 500, arg_count)
				for ith, value in enumerate(values):
					results.writerow([value, name, arg_count, i, ith])
	
	file.close()

def run_tests(tmplversions: [str]):
	for name, tmpl in templates.items():
		for arg_count, patternset in patterns.items():
			for i, pattern in enumerate(patternset):
				pat = [int(arg) for arg in pattern.split(",")]
				tmp = [v for v in range(arg_count)]
				expected = [v for v in range(arg_count)]
				for b in range(arg_count):
					expected[pat[b]] = tmp[b]

				pname = name + "_" + str(arg_count) + "_" + str(i) + "_" + tmpl["program_output"]
				output = f"tests/{pname}"
				outbin = f"tests/prog"

				# print(f"running template {name} arg_count {arg_count} pattern {i} at {output}...")
				generate(pattern, name, outbin, output, False, False)
				expected = np.array(expected)
				found = np.array(run_rand_once(outbin, arg_count, [i for i in range(arg_count)])["values"])

				# print(f"expected: {expected}")
				# print(f"found: {found}")

				if (np.array_equal(found, expected)):
					print(f"test {pname} passed")
				else:
					print(f"test {pname} failed, comparison inequality")
