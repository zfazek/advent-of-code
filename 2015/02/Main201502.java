
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.List;

public class Main201502 {

	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("2015/02.txt"));
		long result1 = 0;
		long result2 = 0;
		for (String line : lines) {
			int[] nums = new int[3];
//			System.out.println(line);
			String[] tokens = line.split("x");
			int l = Integer.parseInt(tokens[0]);
			int w = Integer.parseInt(tokens[1]);
			int h = Integer.parseInt(tokens[2]);
			nums[0] = l;
			nums[1] = w;
			nums[2] = h;
			Arrays.sort(nums);
			int lw = l * w;
			int lh = l * h;
			int wh = w * h;
			long r1 = 2 * lw + 2 * lh + 2 * wh;
//			System.out.println(r1);
			result1 += r1;
			long r2 = 2 * nums[0] + 2 * nums[1];
			r2 += l * w * h;
//			System.out.println(r2);
			result2 += r2;
		}
		System.out.println(result1);
		System.out.println(result2);
	}

}
