package main;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Main202015 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("15a.txt"));
		final long start = System.currentTimeMillis();
		String[] input = lines.get(0).split(",");
		List<Integer> numbers = new ArrayList<>();
		Map<Integer, Integer> dp = new HashMap<>();
		for (String n : input) {
			final int num = Integer.parseInt(n);
//			System.out.println(num);
			numbers.add(num);
			dp.put(num, numbers.size() - 1);
		}
		dp.remove(numbers.get(numbers.size() - 1));
		while (numbers.size() < 10) {
			final int last = numbers.get(numbers.size() - 1);
			System.out.format("idx: %d, last: %d\n",  numbers.size(), last);
			for (int n : numbers) {
				System.out.format("%d ", n);
			}
			System.out.println();
			for (int k : dp.keySet()) {
				System.out.format("%d: %d\n", k, dp.get(k));
			}
			if (!dp.containsKey(last)) {
				dp.put(last, numbers.size() - 1);
				numbers.add(0);
			} else {
				final int idx = dp.get(last);
				int n = numbers.size() - idx - 1;
				numbers.add(n);
				dp.put(last, numbers.size() - 1);
			}
			System.out.println();
		}
		final long end = System.currentTimeMillis();
		System.out.format("time elapsed in sec: %.3f. result: %d\n", (end - start) / 1000.0, numbers.get(numbers.size() - 1));
	}

}
