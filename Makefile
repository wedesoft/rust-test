prime: prime.rs
	rustc -o prime prime.rs

closure: closure.rs
	rustc -o closure closure.rs

clean:
	rm -f closure
