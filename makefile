# Main Command. Compiles (if needed) and runs the program.
run : run-python


run-python : maturin-develop
	uv run python/main.py


maturin-develop :
	maturin develop

run-raw :
	uv run main.py

check : 
	uv run ruff check