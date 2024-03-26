from argparse import ArgumentParser
import numpy as np
import jinja2
import sys
import subprocess
import os
import random
import csv
import json
from templates.base import template as tmplbase
from templates.asm import template as tmplasm
from templates.cuda import template as tmplcuda
from templates.simd import template as tmplsimd

def new_parser():
	p = ArgumentParser(prog=sys.argv[0], description="Wrapper scripts/utilities for Lab 2 Metaprogramming", add_help=True, allow_abbrev=True)
	sub = p.add_subparsers(title="subcommands")
	
	if os.name == "windows":
		progOut = "./prog.exe"
	else:
		progOut = "./prog"

	gen = sub.add_parser("generate")
	gen.add_argument("--pattern", help="Provide the pattern you want to generate against (should be a string with spaces)", type=str, dest="pattern")
	gen.add_argument("--template", help="Provide the template you want to generate from", type=str, default="base1", dest="template")
	gen.add_argument("--output", help="Provide the output location of the generated program executable.", type=str, default=progOut, dest="output")
	gen.set_defaults(func=generate)
	rgen = sub.add_parser("generate_rand")
	rgen.add_argument("--template", help="Provide the template you want to generate from", type=str, default="base1", dest="template")
	rgen.add_argument("--output", help="Provide the output location of the generated program executable.", type=str, default=progOut, dest="output")
	rgen.set_defaults(func=generate_rand)
	r = sub.add_parser("run")
	r.add_argument("--values", help="Provide a comma-separated list of integers to pass to the generated program.", type=str, dest="values")
	r.add_argument("--input", help="Provide a path to the generated program to execute.", type=str, default=progOut, dest="input")
	r.set_defaults(func=run)
	rrand = sub.add_parser("runrand")
	rrand.add_argument("--input", help="Provide a path to the generated program to execute.", type=str, default=progOut, dest="input")
	rrand.add_argument("--iter", help="Provide the number of iterations you want to execute.", type=int, default=50, dest="iterations")
	rrand.add_argument("--argc", help="Provide the number of arguments that are expected in your generated program.", type=int, dest="argc")
	rrand.add_argument("--arga", help="Provide the number of additional arguments that are expected in your generated program.", type=int, dest="arga")
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
	elif sys.argv[1] == "generate_rand":
		return generate_rand(args.template, args.output, False, False)
	elif sys.argv[1] == "run":
		print("running output program")
		return run(args.input, [int(arg) for arg in args.values.split(",")])
	elif sys.argv[1] == "runrand":
		print("running output program with random inputs")
		return run_rand(args.input, args.iterations, args.argc, args.arga)
	elif sys.argv[1] == "runbench":
		print("running reporting")
		bench(args.input)
	elif sys.argv[1] == "runtests":
		print("running tests")
		return run_tests([])

templates = {
	"base1": tmplbase,
	"asm1": tmplasm,
	"cuda1": tmplcuda,
	"simd1": tmplsimd,
}

def generate(pattern: str, template: str, output: str, asm: bool, omp: bool):
	values: [(int, int)] = [(i, int(arg)) for i, arg in enumerate(pattern.split(","))]
	arg_count = len(values)

	outfile = templates[template]["program_output"]

	permute_gen = ""
	if template == "simd1":
		permute_gen = subprocess.run(["cargo", "run", "--manifest-path=bruteforcer/Cargo.toml", "--", "-p", pattern, "simplec"], capture_output=True).stdout.decode()

	env = jinja2.Environment()
	t = env.from_string(templates[template]["template"])
	f = open(file=outfile, mode="w")
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
	subprocess.run(args)

def rand_pattern():
	nums = list(range(0, 1000))
	random.shuffle(nums)
	return nums

def generate_rand(template: str, output: str, asm: bool, omp: bool):
	numstr = ""
	for i in rand_pattern():
		numstr += f"{i},"
	numstr = numstr.removesuffix(",")

	generate(numstr, template, output, asm, omp)

def run(input: str, args: [int]):
	parsed = ""

	for i in args:
		parsed += f"{i},"

	return json.loads(subprocess.run([input, str(len(args)), parsed.rstrip(",")], capture_output=True).stdout.decode()) 

def run_rand(input: str, iterations: int, arg_count: int, additional_args: int):
	s = arg_count + additional_args
	outputs = []

	for i in range(iterations):
		outputs.append(run(input, [int(random.randint(1,1000)) for i in range(s)]))

	return outputs

def bench(outDir: str):
	pass

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
