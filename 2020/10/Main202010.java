import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Main202010 {

	private static int foo(List<Integer> group) {
		switch (group.size()) {
		case 3:
			return 2;
		case 4:
			return 4;
		case 5:
			return 7;
		default:
			return 1;
		}
	}

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2020/10/10.txt"));
		List<Integer> nums = new ArrayList<>();
		for (int i = 0; i < lines.size(); i++) {
			nums.add(Integer.parseInt(lines.get(i)));
		}
		nums.add(0);
		nums.add(Collections.max(nums) + 3);
		Collections.sort(nums);
		int acc1 = 0;
		int acc3 = 0;
		int prev = nums.get(0);
		for (int i = 1; i < nums.size(); i++) {
			final int n = nums.get(i);
			if (n - prev == 1) {
				acc1++;
			} else {
				acc3++;
			}
			prev = n;
		}
		System.out.println(acc1 * acc3);
		List<Integer> group = new ArrayList<>();
		for (int n : nums) {
			System.out.format("%d ", n);
		}
		System.out.println();
		long acc = 1;
		for (int i = 0; i < nums.size() - 1;) {
			group.clear();
			group.add(nums.get(i++));
			while (i < nums.size() && nums.get(i) - nums.get(i - 1) < 3) {
				group.add(nums.get(i++));
			}
			if (group.size() < 3) {
				continue;
			}
			for (int n : group) {
				System.out.format("%3d ", n);
			}
			System.out.println();
			acc *= foo(group);
		}
		System.out.println(acc);
	}

}
