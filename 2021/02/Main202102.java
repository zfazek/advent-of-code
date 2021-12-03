import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main202102 {
	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2021/02/02.txt"));
		one(lines);
		two(lines);
	}

	private static void one(List<String> lines) {
		int x = 0;
		int y = 0;
		for (String line : lines) {
			String[] tokens = line.split(" ");
			switch (tokens[0]) {
			case "forward":
				x += Integer.parseInt(tokens[1]);
				break;
			case "down":
				y += Integer.parseInt(tokens[1]);
				break;
			case "up":
				y -= Integer.parseInt(tokens[1]);
				break;
			}
		}
		System.out.println(x * y);
	}

	private static void two(List<String> lines) {
		int x = 0;
		int y = 0;
		int aim = 0;
		for (String line : lines) {
			String[] tokens = line.split(" ");
			switch (tokens[0]) {
			case "forward":
				x += Integer.parseInt(tokens[1]);
				y += aim * Integer.parseInt(tokens[1]);
				break;
			case "down":
				aim += Integer.parseInt(tokens[1]);
				break;
			case "up":
				aim -= Integer.parseInt(tokens[1]);
				break;
			}
		}
		System.out.println(x * y);
	}
}
