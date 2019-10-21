import java.io.*;
import java.util.*;
import java.text.*;
import java.math.*;
import java.util.regex.*;

public class Solution {

	// Open-ended problem, no tricks or research required
	// There's two main approaches:
	//
	// top-down: determine the complexity of the outermost loops, and use
	// this to determine the complexity of the loops inside it, and then the
	// loops inside those, etc. Each loop by itself is assigned a purely
	// multiplicative complexity. Add together the complexities for all loops and
	// then simplify.
	//
	// bottom-up: the "overall" complexity of a program (AKA a list of loops) is the
	// simplified sum of the complexities of the loops in the list. The "overall"
	// complexity of a single loop is the overall complexity of its body (AKA
	// another program) but modified by (1) mapping from the loop's local variable
	// (which is a global variable from the perspective of the body) to the loop's
	// global variable, and (2) multiplying the body's complexity by this global
	// variable since it is executed that many times.
	//
	// I chose the bottom-up approach.
	//
	// The "complexity()" method takes a list of loops, and for each loop
	// recursively calls itself to determine the complexity of that loop's body,
	// then maps from the body's complexity to that loop's complexity, and sums up
	// these loop complexities, and returns that.
	//
	// The "Polynomial" class represents a polynomial as a list of ProductTerm
	// objects. It has a "reduce()" method which removes "small" terms. It also has
	// a "toString()" method.
	//
	// The "ProductTerm" class represents each product term as an array. The array
	// is 26 values long, and the values represent how many times each letter
	// appears in the product. It's easy to add variables by incrementing the
	// relevant values, and also to convert one variable into another. The
	// "toString()" and "compareTo()" are used for output formatting, and the
	// "includes()" method is used to tell whether another ProductTerm should be
	// filtered out by the current one.

	public static void main(String[] args) throws FileNotFoundException {
		Scanner in = new Scanner(System.in);
		System.out.println(complexity(in));
	}

	public static Polynomial complexity(Scanner input) {
		Polynomial res = new Polynomial();
		res.add(new ProductTerm());
		while (input.hasNext()) {
			String token = input.next();
			if (token.equals("end")) {
				break;
			}
			// token was "for"
			char from = input.next().charAt(0);
			input.next(); // remove the "in"
			char to = input.next().charAt(0);
			Polynomial child = complexity(input);
			for (ProductTerm p : child) {
				p.map(from, to);
				p.mult(to);
			}
			res.addAll(child);
		}
		res.reduce();
		return res;
	}

	static class Polynomial extends ArrayList<ProductTerm> {

		void reduce() {
			for (int i = size() - 1; i >= 0; i--) {
				for (int j = 0; j < size(); j++) {
					if (j != i && get(j).includes(get(i))) {
						remove(i);
						break;
					}
				}
			}
		}

		public String toString() {
			Collections.sort(this);
			String res = "";
			for (int i = 0; i < size(); i++) {
				if (i > 0) {
					res += "+";
				}
				res += get(i);
			}
			return res;
		}

	}

	static class ProductTerm implements Comparable<ProductTerm> {
		int[] vars = new int[26];

		public String toString() {
			String res = "";
			for (int i = 0; i < vars.length; i++) {
				for (int j = 0; j < vars[i]; j++) {
					res += (char) ('a' + i);
				}
			}
			return res;
		}

		public int compareTo(ProductTerm arg0) {
			return this.toString().compareTo(arg0.toString());
		}

		void mult(char v) {
			vars[v - 'a']++;
		}

		void map(char from, char to) {
			vars[to - 'a'] += vars[from - 'a'];
			vars[from - 'a'] = 0;
		}

		public boolean includes(ProductTerm b) {
			for (int i = 0; i < vars.length; i++) {
				if (vars[i] < b.vars[i]) {
					return false;
				}
			}
			return true;
		}

	}
}
