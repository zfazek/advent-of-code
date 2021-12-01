import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Main202009 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2020/09/09.txt"));
		List<Long> nums = new ArrayList<>();
		for (int i = 0; i < lines.size(); i++) {
			nums.add(Long.parseLong(lines.get(i)));
		}
		final long start = System.currentTimeMillis();
		Set<Long> sums = new HashSet<>();
		final int N = 25;
		long result = 0;
		for (int i = 0; i < nums.size(); i++) {
			sums.clear();
			for (int j = 1; j <= N; j++) {
				for (int k = j + 1; k <= N; k++) {
					if (j != k && i - j >= 0 && i - k >= 0) {
						sums.add(nums.get(i - j) + nums.get(i - k));
					}
				}
			}
			if (i >= N && !sums.contains(nums.get(i))) {
				System.out.println("i: " + i);
				result = nums.get(i);
				System.out.println(result);
				break;
			}
		}
		for (int i = 0; i < nums.size(); i++) {
			long acc = 0;
			int n = 0;
			while (acc < result) {
				acc += nums.get(i + n);
				n++;
			}
			if (acc == result && n > 1) {
				System.out.println("i: " + i);
				System.out.println("n: " + n);
				long min = nums.get(i);
				long max = nums.get(i);
				for (int j = 0; j < n; j++) {
					final long num = nums.get(i + j);
					if (num < min) {
						min = num;
					}
					if (num > max) {
						max = num;
					}
				}
				System.out.format("%d %d %d\n", min, max, min + max);
			}
		}
		final long end = System.currentTimeMillis();
		System.out.format("Elapsed time: %.3f sec\n", (end - start) / 1000.0);
	}

}
