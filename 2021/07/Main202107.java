import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.Stream;

public class Main202107 {
	public static void main(String[] args) throws IOException {
		List<Integer> input = Stream.of(Files.readString(Paths.get("2021/07/07.txt")).split(","))
				.mapToInt(Integer::valueOf)
				.sorted()
				.mapToObj(Integer::valueOf)
				.collect(Collectors.toList());
		int min1 = Integer.MAX_VALUE;
		int min2 = Integer.MAX_VALUE;
		for (int i = input.get(0); i <= input.get(input.size() - 1); i++) {
			int value = i;
			int s1 = input.stream().map(e -> Math.abs(e - value)).mapToInt(Integer::valueOf).sum();
			int s2 = input.stream().map(e -> Math.abs(e - value) * (Math.abs(e - value) + 1) / 2).mapToInt(Integer::valueOf).sum();
			min1 = Math.min(s1, min1);
			min2 = Math.min(s2, min2);
		}
		System.out.println(min1);
		System.out.println(min2);
	}
}