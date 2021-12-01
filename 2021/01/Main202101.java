import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main202101 {
	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2021/01/01.txt"));
		one(lines);
		two(lines);
	}

	private static void one(List<String> lines) {
		long n = 0;
		for (int i = 1; i < lines.size(); i++) {
			int n1 = Integer.parseInt(lines.get(i - 1));
			int n2 = Integer.parseInt(lines.get(i));
			if (n2 > n1) {
				n++;
			}
		}
		System.out.println(n);
	}

	private static void two(List<String> lines) {
		long n = 0;
		for (int i = 3; i < lines.size(); i++) {
			int n1 = Integer.parseInt(lines.get(i - 3));
			int n2 = Integer.parseInt(lines.get(i));
			if (n2 > n1) {
				n++;
			}
		}
		System.out.println(n);
	}

}
