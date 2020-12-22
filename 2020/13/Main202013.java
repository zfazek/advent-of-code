package main;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Main202013 {

	private static void foo1() throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("13.txt"));
		int origTime = Integer.parseInt(lines.get(0));
		String[] arr = lines.get(1).split(",");
		List<Integer> numbers = new ArrayList<Integer>();
		for (String a : arr) {
			if (a.equals("x")) {
				continue;
			}
			numbers.add(Integer.parseInt(a));
		}
		int time = origTime;
		boolean end = false;
		while (!end) {
			for (int number : numbers) {
				if (time % number == 0) {
					System.out.println(number * (time - origTime));
					end = true;
					break;
				}
			}
			time++;
		}
	}

	private static void foo2() throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("input.txt"));
		String[] arr = lines.get(1).split(",");
		Map<Integer, Integer> numbers = new HashMap<>();
		for (int i = 0; i < arr.length; i++) {
			if (!arr[i].equals("x")) {
				numbers.put(i, Integer.parseInt(arr[i]));
			}
		}
		int idxMax = 0;
		int busMax = 0;
		long nn = 1;
		for (int idx : numbers.keySet()) {
			System.out.format("idx: %d, bus: %d\n", idx, numbers.get(idx));
			if (numbers.get(idx) > busMax) {
				idxMax = idx;
				busMax = numbers.get(idx);
			}
			nn *= numbers.get(idx);
		}
		System.out.format("idxMax: %d, busMax: %d, nn: %d\n", idxMax, busMax, nn);
		long n = 645338524723718L / busMax * busMax - idxMax;
//		long n = nn + busMax - idxMax;
		while (true) {
			boolean good = true;
			for (int idx : numbers.keySet()) {
				final int bus = numbers.get(idx);
				if ((n + idx) % bus != 0) {
					good = false;
					break;
				}
			}
			if (good) {
				System.out.println(n);
				break;
			}
			n += busMax;
			if (n % 100000000 == 0) {
				System.out.println(n);
			}
		}
	}

	public static void main(String[] args) throws IOException {
		foo1();
		foo2();
	}


}
