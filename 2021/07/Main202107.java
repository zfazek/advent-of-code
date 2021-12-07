import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.stream.Stream;

public class Main202107 {
	public static void main(String[] args) throws IOException {
		String input = Files.readString(Paths.get("2021/07/07.txt"));
		one(input);
		two(input);
	}

	private static void one(String input) {
		String[] tokens = input.split(",");
		List<Integer> results = new ArrayList<>();
		for (int i = 0; i < tokens.length; i++) {
			int value = Integer.parseInt(tokens[i]);
			int s = Stream.of(tokens).mapToInt(Integer::valueOf).map(e -> Math.abs(e - value)).sum();
			results.add(s);
		}
		System.out.println(Collections.min(results));
	}

	private static void two(String input) {
		String[] tokens = input.split(",");
		List<Integer> numbers = new ArrayList<>();
		for (int i = 0; i < tokens.length; i++) {
			numbers.add(Integer.parseInt(tokens[i]));
		}
		List<Integer> results = new ArrayList<>();
		for (int i = Collections.min(numbers); i <= Collections.max(numbers); i++) {
			int value = i;
			int s = numbers.stream().mapToInt(Integer::valueOf).map(e -> Math.abs(e - value) * (Math.abs(e - value) + 1) / 2).sum();
			results.add(s);
		}
		System.out.println(Collections.min(results));
	}
}
