import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main202125 {
	public static void main(String[] args) throws IOException {
		List<String> input = Files.readAllLines(Paths.get("2021/25/25.txt"));
		long start = System.currentTimeMillis();
		int sizeX = input.get(0).length();
		int sizeY = input.size();
		char [][] arr = new char[sizeY][sizeX];
		for (int i = 0; i < sizeY; i++) {
			for (int j = 0; j < sizeX; j++) {
				char c = input.get(i).charAt(j);
				if (c == '>' || c == 'v') {
					arr[i][j] = c;
				}
			}
		}
		long step = 0;
		boolean didMove;
		//		print(arr);
		do {
			char [][] newArr = new char[sizeY][sizeX];
			didMove = false;
			step++;
			for (int i = 0; i < sizeY; i++) {
				for (int j = 0; j < sizeX; j++) {
					char c = arr[i][j];
					if (c == 'v') {
						newArr[i][j] = c;
						continue;
					}
					if (c == '>') {
						int nextX = (j + 1) % sizeX;
						if (arr[i][nextX] != 'v' && arr[i][nextX] != '>') {
							didMove = true;
							newArr[i][nextX] = c;
						} else {
							newArr[i][j] = c;
						}
					}
				}
			}
			arr = newArr;
			//			print(arr);
			newArr = new char[sizeY][sizeX];
			for (int i = 0; i < sizeY; i++) {
				for (int j = 0; j < sizeX; j++) {
					char c = arr[i][j];
					if (c == '>') {
						newArr[i][j] = c;
						continue;
					}
					if (c == 'v') {
						int nextY = (i + 1) % sizeY;
						if (arr[nextY][j] != 'v' && arr[nextY][j] != '>') {
							didMove = true;
							newArr[nextY][j] = c;
						} else {
							newArr[i][j] = c;
						}
					}
				}
			}
			arr = newArr;
			//			print(arr);
		} while (didMove && step < 600);
		long end = System.currentTimeMillis();
		System.out.println(step);
		System.out.format("%d ms\n", end - start);
	}

	@SuppressWarnings("unused")
	private static void print(char[][] arr) {
		for (int i = 0; i < arr.length; i++) {
			for (int j = 0; j < arr[0].length; j++) {
				System.out.print(arr[i][j]);
			}
			System.out.println();
		}
		System.out.println();
	}
}
