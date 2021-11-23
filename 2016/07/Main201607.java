import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class Main201607 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2016/07.txt"));
		one(lines);
		two(lines);
	}

	private static boolean isABA(String str, char c, char cc) {
		for (int i = 0; i < str.length() - 2; i++) {
			char c1 = str.charAt(i);
			char c2 = str.charAt(i + 1);
			char c3 = str.charAt(i + 2);
			if (c1 == c && c2 == cc && c1 != c2 && c1 == c3) {
				return true;
			}
		}
		return false;
	}

	private static boolean isABBA(String str) {
		for (int i = 0; i < str.length() - 3; i++) {
			char c1 = str.charAt(i);
			char c2 = str.charAt(i + 1);
			char c3 = str.charAt(i + 2);
			char c4 = str.charAt(i + 3);
			if (c1 != c2 && c1 == c4 && c2 == c3) {
				return true;
			}
		}
		return false;
	}

	private static void one(List<String> lines) {
		long n = 0;
		for (String line : lines) {
			boolean good = false;
			String[] tokens = line.split("\\]");
			for (String token : tokens) {
				String[] t = token.split("\\[");
				if (isABBA(t[0])) {
					good = true;
				}
				if (t.length > 1 && isABBA(t[1])) {
					good = false;
					break;
				}
			}
			if (good) {
				n++;
			}
		}
		System.out.println(n);
	}

	private static void two(List<String> lines) {
		long n = 0;
		for (String line : lines) {
			List<String> supernets = new ArrayList<>();
			List<String> hypernets = new ArrayList<>();
			String[] tokens = line.split("\\]");
			for (String token : tokens) {
				String[] t = token.split("\\[");
				supernets.add(t[0]);
				if (t.length > 1) {
					hypernets.add(t[1]);
				}
			}
			loops:
				for (String supernet : supernets) {
					for (int i = 0; i < supernet.length() - 2; i++) {
						char c1 = supernet.charAt(i);
						char c2 = supernet.charAt(i + 1);
						char c3 = supernet.charAt(i + 2);
						if (c1 != c2 && c1 == c3) {
							for (String hypernet : hypernets) {
								if (isABA(hypernet, c2, c1)) {
									n++;
									break loops;
								}
							}
						}
					}
				}
		}
		System.out.println(n);
	}
}

