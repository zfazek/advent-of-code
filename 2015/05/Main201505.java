
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main201505 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("05.txt"));
		long n1 = 0;
		long n2 = 0;
		for (String line : lines) {
//			System.out.println(line);
			if (isNice1(line)) {
				n1++;
			}
			if (isNice2(line)) {
				n2++;
			}
		}
		System.out.println(n1);
		System.out.println(n2);
	}

	private static boolean isNice1(String line) {
//		aeiou
		int numberOfVowels = 0;
		for (int i = 0; i < line.length(); i++) {
			char c = line.charAt(i);
			switch (c) {
			case 'a':
			case 'e':
			case 'i':
			case 'o':
			case 'u':
				numberOfVowels++;
			}
		}
		if (numberOfVowels < 3) {
			return false;
		}
		boolean doubleLetter = false;
		for (int i = 0; i < line.length() - 1; i++) {
			char c = line.charAt(i);
			char c1 = line.charAt(i + 1);
			if (c == c1) {
				doubleLetter = true;
				break;
			}
		}
		if (!doubleLetter) {
			return false;
		}
		if (line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy")) {
			return false;
		}
		return true;
	}

	private static boolean isNice2(String line) {
		boolean first = false;
		for (int i = 0; i < line.length() - 1; i++) {
			String s = line.substring(i, i + 2);
			if (line.substring(i + 2).contains(s)) {
				first = true;
				break;
			}
		}
		boolean second = false;
		for (int i = 0; i < line.length() - 2; i++) {
			char c1 = line.charAt(i);
			char c2 = line.charAt(i + 2);
			if (c1 == c2) {
				second = true;
				break;
			}
		}
		return first && second;
	}


}
