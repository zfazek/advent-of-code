package main;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main202012 {

	private static void fooA() throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("12.txt"));
		int degree = 0;
		int x = 0;
		int y = 0;
		for (String line : lines) {
			final char cmd = line.charAt(0);
			final int length = Integer.parseInt(line.substring(1));
			switch (cmd) {
			case 'N':
				y += length;
				break;
			case 'S':
				y -= length;
				break;
			case 'E':
				x += length;
				break;
			case 'W':
				x -= length;
				break;
			case 'L':
				degree += length;
				degree = degree % 360;
				if (degree < 0) {
					degree = 360 + degree;
				}
				break;
			case 'R':
				degree -= length;
				degree = degree % 360;
				if (degree < 0) {
					degree = 360 + degree;
				}
				break;
			case 'F':
				switch (degree) {
				case 0:
					x += length;
					break;
				case 90:
					y += length;
					break;
				case 180:
					x -= length;
					break;
				case 270:
					y -= length;
					break;
					default:
						System.out.println(line);
				}
			}
			System.out.format("%3s x: %d, y: %d, degree: %d\n", line, x, y, degree);
		}
		System.out.format("x: %d, y: %d, Manhattan: %d\n\n", x, y, Math.abs(x) + Math.abs(y));
	}

	private static void fooB() throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("12.txt"));
		int x = 0;
		int y = 0;
		int wx = 10;
		int wy = 1;
		for (String line : lines) {
			final char cmd = line.charAt(0);
			int length = Integer.parseInt(line.substring(1));
			switch (cmd) {
			case 'N':
				wy += length;
				break;
			case 'S':
				wy -= length;
				break;
			case 'E':
				wx += length;
				break;
			case 'W':
				wx -= length;
				break;
			case 'L':
			case 'R':
				if (cmd == 'L') {
					length = -length;
				}
				switch (length) {
				case 90:
				case -270:
					int tmp = wy;
					wy = -wx;
					wx = tmp;
					break;
				case -90:
				case 270:
					int tmp1 = wy;
					wy = wx;
					wx = -tmp1;
					break;
				case 180:
				case -180:
					wx = -wx;
					wy = -wy;
					break;
				}
				break;
			case 'F':
				x += wx * length;
				y += wy * length;
			}
			System.out.format("%3s x: %d, y: %d, wx: %d, wy: %d\n", line, x, y, wx, wy);
		}
		System.out.format("x: %d, y: %d, Manhattan: %d\n", x, y, Math.abs(x) + Math.abs(y));
	}

	public static void main(String[] args) throws IOException {
//		fooA();
		fooB();
	}


}
