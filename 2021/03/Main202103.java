import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

public class Main202103 {
	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2021/03/03.txt"));
		one(lines);
		two(lines);
	}

	private static void one(List<String> lines) {
		int size = lines.get(0).length();
		String a = "";
		String b = "";
		for (int i = 0; i < size; i++) {
			int[] arr = new int[2];
			for (String line : lines) {
				char c = line.charAt(i);
				arr[c - '0']++;
			}
			if (arr[0] > arr[1]) {
				a += '0';
				b += '1';
			} else {
				a += '1';
				b += '0';
			}
		}
		System.out.println(Integer.parseInt(a, 2) * Integer.parseInt(b, 2));
	}

	private static void two(List<String> lines) {
		List<String> lines1 = new ArrayList<>();
		for (String line : lines) {
			lines1.add(line);
		}
		int size = lines.get(0).length();
		String a = null;
		String b = null;
		for (int i = 0; i < size; i++) {
			int[] arr = new int[2];
			for (String line : lines) {
				char c = line.charAt(i);
				arr[c - '0']++;
			}
			int[] arr1 = new int[2];
			for (String line : lines1) {
				char c = line.charAt(i);
				arr1[c - '0']++;
			}
			System.out.format("i: %d, arr[0]: %d, arr[1]: %d\n lines: %s\nlines1: %s\n\n",
					i, arr[0], arr[1], lines, lines1);
			char pattern = ' ';
			if (arr[0] > arr[1]) {
				pattern = '1';
			} else {
				pattern = '0';
			}
			Iterator<String> it = lines.iterator();
			while (a == null && it.hasNext()) {
				String line = it.next();
				if (line.charAt(i) == pattern) {
					it.remove();
					if (lines.size() == 1) {
						a = lines.get(0);
					}
				}
			}
			char pattern1 = ' ';
			if (arr1[0] > arr1[1]) {
				pattern1 = '0';
			} else {
				pattern1 = '1';
			}
			Iterator<String> it1 = lines1.iterator();
			while (b == null && it1.hasNext()) {
				String line = it1.next();
				if (line.charAt(i) == pattern1) {
					it1.remove();
					if (lines1.size() == 1) {
						b = lines1.get(0);
					}
				}
			}
			System.out.format(" lines: %s\nlines1: %s\n\n", lines, lines1);
			if (a != null && b != null) {
				System.out.println(Integer.parseInt(a, 2) * Integer.parseInt(b, 2));
				break;
			}
		}
	}
}
