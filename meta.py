#!/usr/bin/env python

from commands import new_parser, runner

def main():
	p = new_parser()
	opts = p.parse_args()
	return runner(opts)

# Entrypoint to all the things
if __name__ == '__main__':
	main()
