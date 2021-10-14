
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main201501 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2015/01.txt"));
		long n = 0;
		long idx = 0;
		for (String line : lines) {
			System.out.println(line);
			System.out.println(line.length());
			for (int i = 0; i < line.length(); i++) {
				final char c = line.charAt(i);
				if (c == '(') {
					n++;
				} else if (c == ')') {
					n--;
					if (n < 0 && idx == 0) {
						idx = i + 1;
					}
				}
			}
		}
		System.out.println(n);
		System.out.println(idx);
	}

}
