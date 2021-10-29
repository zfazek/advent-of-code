import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

public class Main201604 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2016/04.txt"));
		one(lines);
		two(lines);
	}

	private static void one(List<String> lines) {
		long sum = 0;
		for (String line : lines) {
			Map<Character, Integer> letters = new TreeMap<>();
			String[] names = line.split("\\[");
			String name = names[0];
			String[] tokens = name.split("-");
			String sectorIdStr = tokens[tokens.length - 1];
			int sectorId = Integer.parseInt(sectorIdStr);
			String checksum = names[1].substring(0, 5);
			for (int i = 0; i < name.length(); i++) {
				char c = name.charAt(i);
				if (c >= 'a' && c <= 'z') {
					if (letters.containsKey(c)) {
						letters.put(c, letters.get(c) + 1);
					} else {
						letters.put(c, 1);
					}
				}
			}
			List<Data> data = new ArrayList<>();
			for (char k : letters.keySet()) {
				data.add(new Data(k, letters.get(k)));
			}
			Collections.sort(data, new MyComparator());
			boolean good = true;
			for (int i = 0; i < 5; i++) {
				if (checksum.charAt(i) != data.get(i).c) {
					good = false;
					break;
				}
			}
			if (good) {
				sum += sectorId;
			}
		}
		System.out.println(sum);
	}

	private static void two(List<String> lines) {
		for (String line : lines) {
			String[] names = line.split("\\[");
			String name = names[0];
			String[] tokens = name.split("-");
			String sectorIdStr = tokens[tokens.length - 1];
			int sectorId = Integer.parseInt(sectorIdStr);
			String text = name.substring(0, name.lastIndexOf('-'));
			String str = "";
			for (int i = 0; i < text.length(); i++) {
				if (text.charAt(i) == '-') {
					str += " ";
				} else {
					str += (char)((text.charAt(i) - 'a' + (sectorId % 26)) % 26 + 'a');
				}
			}
			if (str.contains("north")) {
				System.out.println(sectorId);
			}
		}
	}
}

class Data {
	char c;
	int n;

	Data(char c, int n) {
		this.c = c;
		this.n = n;
	}

	@Override
	public String toString() {
		return String.format("%c %d", c, n);
	}
}

class MyComparator implements Comparator<Data> {

	@Override
	public int compare(Data o1, Data o2) {
		if (o1.n == o2.n) {
			return o1.c - o2.c;
		} else {
			return o2.n - o1.n;
		}
	}
}