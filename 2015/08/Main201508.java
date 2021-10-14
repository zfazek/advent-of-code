
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main201508 {

	private static void one(List<String> lines) {
		long n = 0;
		for (String line : lines) {
			n += line.length();
			for (int i = 1; i < line.length() - 1; i++) {
				char c = line.charAt(i);
				if (c == '\\') {
					if (line.charAt(i + 1) == 'x') {
						i += 3;
						n--;
					} else {
						i++;
						n--;
					}
				} else {
					n--;
				}
			}
		}
		System.out.println(n);
	}

	private static void two(List<String> lines) {
		long n = 0;
		for (String line : lines) {
			n += 2;
			for (int i = 0; i < line.length(); i++) {
				char c = line.charAt(i);
				if (c == '"' || c == '\\') {
					n++;
				}
			}
		}
		System.out.println(n);
	}

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2015/08.txt"));
		one(lines);
		two(lines);
	}

}
