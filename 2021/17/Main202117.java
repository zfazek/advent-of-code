import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Main202117 {
	public static void main(String[] args) throws IOException {
		String input = Files.readString(Paths.get("2021/17/17.txt"));
		String[] tokens = input.split("=");
		int x1 = Integer.parseInt(tokens[1].split("\\.\\.")[0]);
		int x2 = Integer.parseInt(tokens[1].split("\\.\\.")[1].split(",")[0]);
		int y1 = Integer.parseInt(tokens[2].split("\\.\\.")[0]);
		int y2 = Integer.parseInt(tokens[2].split("\\.\\.")[1].split(",")[0]);
		int max_y_overall = 0;
		long n = 0;
		for (int vx = 1; vx <= x2; vx++) {
			for (int vy = -200; vy < 1000; vy++) {
				int x = 0;
				int y = 0;
				int maxY= 0;
				int dx = vx;
				int dy = vy;
				while (true) {
					x += dx;
					y += dy;
					if (dx > 0) {
						dx--;
					} else if (dx < 0) {
						dx++;
					}
					dy--;
					maxY = Math.max(maxY, y);
					if (x >= x1 && x <= x2 && y >= y1 && y <= y2) {
						max_y_overall = Math.max(max_y_overall, maxY);
						n++;
						break;
					}
					if (x > x2 || y < y1) {
						break;
					}
					if (dx == 0 && y < y1) {
						break;
					}
				}
			}
		}
		System.out.println(max_y_overall);
		System.out.println(n);
	}
}
