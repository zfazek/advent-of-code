import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class Main202106 {
	public static void main(String[] args) throws IOException {
		String input = Files.readString(Paths.get("2021/06/06.txt"));
		one(input);
		two(input);
	}

	private static void one(String input) {
		String[] tokens = input.split(",");
		List<Integer> fishes = new ArrayList<>();
		for (String token : tokens) {
			fishes.add(Integer.parseInt(token));
		}
		for (int i = 0; i < 80; i++) {
			int size = fishes.size();
			for (int j = 0; j < size; j++) {
				int fish = fishes.get(j);
				if (fish == 0) {
					fishes.set(j, 6);
					fishes.add(8);
				} else {
					fishes.set(j, fish - 1);
				}
			}
		}
		System.out.println(fishes.size());
	}

	private static void two(String input) {
		String[] tokens = input.split(",");
		long[] arr = new long[9];
		for (String token : tokens) {
			arr[Integer.parseInt(token)]++;
		}
		for (int i = 0; i < 256; i++) {
			long zeros = arr[0];
			for (int a = 1; a < arr.length; a++) {
				arr[a - 1] = arr[a];
			}
			arr[6] += zeros;
			arr[8] = zeros;
		}
		long n = 0;
		for (int i = 0; i < arr.length; i++) {
			n += arr[i];
		}
		System.out.println(n);
	}
}
