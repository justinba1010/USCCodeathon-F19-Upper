import java.util.Scanner;

public class Solution {

	// https://en.wikipedia.org/wiki/Sorting_network
	// The actual term for the sorting procedure I describe is a Sorting Network.
	// The easiest way to solve this problem was to google until you found this out.
	// Real programming work is 95% googling anyways.
	//
	// From wikipedia:
	// "There are n! permutations of numbers in an n-wire network, and to test all
	// of them would take a significant amount of time, especially when n is large.
	// The number of test cases can be reduced significantly, to 2^n, using the
	// so-called zero-one principle."
	// "The zero-one principle states that, if a sorting network can correctly sort
	// all 2^n sequences of zeros and ones, then it is also valid for arbitrary
	// ordered inputs. This not only drastically cuts down on the number of tests
	// needed to ascertain the validity of a network, it is of great use in creating
	// many constructions of sorting networks as well."
	//
	// The core logic is that 0s are "low numbers" and 1s are "high numbers". If the
	// sorting procedure can always group low and high numbers together properly, no
	// matter where they are, or how many of each type there are, then it can sort
	// anything.
	// More rigorously, if all of the following hold:
	// - highest number ends up in highest spot no matter its initial position
	// - 2 highest numbers end up in the 2 highest spots for all initial positions
	// - etc.
	// ...then it sorts.
	//
	// The "optimized" solution is just a brute-forcer on 2^(class size) arrays
	// instead of the naive solution of (class size)! arrays.
	//
	// The fact that this is problem is NP-complete was a subtle hint:
	// Every "NP-complete" problem can be solved in 2^(something).

	public static void main(String[] args) {
		Scanner in = new Scanner(System.in);
		int width = in.nextInt();
		int len = in.nextInt();
		int[][] network = new int[len][2];
		for (int i = 0; i < len; i++) {
			network[i][0] = in.nextInt();
			network[i][1] = in.nextInt();
		}
		System.out.println(verifyAll(network, width));
		in.close();
	}

	public static int verifyAll(int[][] network, int width) {
		// brute-forcing loop
		int worsttime = 0;
		for (int num = 1; num < 1 << width; num++) {
			int[] arr = genBitArray(num, width);
			int res = verify(network, arr.clone());
			if (res == -1) {
				return -1;
			} else if (res > worsttime) {
				worsttime = res;
			}
		}
		return worsttime;
	}

	public static int verify(int[][] network, int[] testcase) {
		// measure time for network to sort a particular array
		for (int step = 0; step < network.length; step++) {
			if (isSorted(testcase)) {
				return step;
			}
			int left = network[step][0];
			int right = network[step][1];
			if (testcase[left] > testcase[right]) {
				int temp = testcase[left];
				testcase[left] = testcase[right];
				testcase[right] = temp;
			}
		}
		if (isSorted(testcase)) {
			return network.length;
		}
		return -1;
	}

	public static boolean isSorted(int[] arr) {
		for (int i = 1; i < arr.length; i++) {
			if (arr[i - 1] > arr[i]) {
				return false;
			}
		}
		return true;
	}

	public static int[] genBitArray(int num, int width) {
		// convert from an integer to a 1s-and-0s array
		int[] arr = new int[width];
		for (int i = 0; i < width; i++) {
			arr[i] = num >> i & 1;
		}
		return arr;
	}
}
