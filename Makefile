graph:
	cargo graph --optional-line-style dashed --optional-line-color red --optional-shape box --build-shape diamond --build-color green --build-line-color orange 
	> cargo-count.dot
	dot -Tpng > rainbow-graph.png cargo-count.dot

