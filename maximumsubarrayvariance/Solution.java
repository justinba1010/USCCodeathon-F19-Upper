import java.util.Scanner;

public class Solution {
	public static void main(String[] args) {
		Scanner in = new Scanner(System.in);
		int len = in.nextInt();
		int[] arr = new int[len];
		for (int i = 0; i < len; i++) {
			arr[i] = in.nextInt();
		}
		System.out.println(calcMaxN2d(arr));
		in.close();
	}

	public static int calcMaxN2d(int[] array) {
		double maxvar = 0;
		int maxlen = 0;

		for (int start = 0; start < array.length - 1; start++) {
			// Welford's online algorithm
			// go directly from variance(subarray) to variance(subarray + 1 more number)
			// https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance
			// "m" represents the running sum of squares (variance without division by n-1)
			double m = 0;
			// "avg" represents the running average
			double avg = array[start];
			for (int end = start + 1; end < array.length; end++) {
				// length of new subarray ("end" index is inclusive)
				int n = 1 + end - start;
				// go directly from old average to new average
				double newavg = avg + (array[end] - avg) / n;
				// go directly from old sum-of-squares to new sum-of-squares
				m += (array[end] - avg) * (array[end] - newavg);
				avg = newavg;
				// get variance of current subarray and keep track of best solution
				double var = m / (n - 1);
				if (var > maxvar) {
					maxvar = var;
					maxlen = n;
				} else if (var == maxvar && n > maxlen) {
					maxlen = n;
				}
			}
		}

		return maxlen;
	}

}
